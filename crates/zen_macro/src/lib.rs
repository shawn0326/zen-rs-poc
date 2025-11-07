use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{Attribute, Data, DeriveInput, Fields, parse_macro_input};

#[proc_macro_derive(Uniforms, attributes(uniform))]
pub fn shader_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = &input.ident;
    struct UniField {
        name: String,
        wgsl_ty: String,
    }
    struct TexField {
        name: String,
        sampler_binding: u32,
        texture_binding: u32,
    }

    let fields = match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => &fields.named,
            _ => panic!("Only named fields supported"),
        },
        _ => panic!("Only structs supported"),
    };

    let mut uniform_fields = Vec::new();
    let mut texture_fields = Vec::new();
    let mut binding_idx: u32 = 1;
    for field in fields {
        let name = field.ident.as_ref().unwrap().to_string();
        let is_uniform = field.attrs.iter().any(|attr| is_uniform_attr(attr));
        let ty = &field.ty;
        if !is_uniform {
            continue;
        }
        if is_texture_ref(ty) {
            let sampler_binding = binding_idx;
            let texture_binding = binding_idx + 1;
            texture_fields.push(TexField {
                name: name,
                sampler_binding,
                texture_binding,
            });
            binding_idx += 2;
            continue;
        }
        if let Some(wgsl_ty) = rust_ty_to_wgsl_ty(ty) {
            uniform_fields.push(UniField {
                name: name.clone(),
                wgsl_ty: wgsl_ty.to_string(),
            });
        } else {
            let ty_str = quote!(#ty).to_string();
            return syn::Error::new_spanned(
                &field.ident,
                format!(
                    "Unsupported uniform field type for field '{}': {}",
                    name, ty_str
                ),
            )
            .to_compile_error()
            .into();
        }
    }

    let wgsl_parts = {
        let wgsl_struct_name = "MaterialUniforms";
        let mut wgsl_struct_fields: Vec<String> = Vec::new();
        for uf in &uniform_fields {
            wgsl_struct_fields.push(format!("    {}: {},", uf.name, uf.wgsl_ty));
        }
        let struct_body = wgsl_struct_fields.join("\n");
        let mut wgsl_parts: Vec<String> = Vec::new();
        wgsl_parts.push(format!(
            "struct {} {{\n{}\n}};",
            wgsl_struct_name, struct_body
        ));
        wgsl_parts.push(format!(
            "\n@group(2) @binding(0)\nvar<uniform> material: {};\n",
            wgsl_struct_name
        ));
        for tf in &texture_fields {
            wgsl_parts.push(format!(
                "@group(2) @binding({}) var {}_sampler: sampler;",
                tf.sampler_binding, tf.name
            ));
            wgsl_parts.push(format!(
                "@group(2) @binding({}) var {}: texture_2d<f32>;",
                tf.texture_binding, tf.name
            ));
        }
        wgsl_parts
    };

    let layout_entries = {
        let mut entries = Vec::new();

        entries.push(quote! {
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None
                },
                count: None
            }
        });

        for tf in &texture_fields {
            let sampler_binding = tf.sampler_binding;
            let texture_binding = tf.texture_binding;
            entries.push(quote! {
                wgpu::BindGroupLayoutEntry {
                    binding: #sampler_binding,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None
                }
            });
            entries.push(quote! {
                wgpu::BindGroupLayoutEntry {
                    binding: #texture_binding,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        multisampled: false,
                        view_dimension: wgpu::TextureViewDimension::D2,
                        sample_type: wgpu::TextureSampleType::Float { filterable: true }
                    },
                    count: None
                }
            });
        }
        entries
    };

    let bytes_body = if !uniform_fields.is_empty() {
        let mut write_fields_ts = Vec::new();
        let mut offset: usize = 0;
        let mut struct_align: usize = 1;

        for uf in &uniform_fields {
            let (align, slot_size) = match uf.wgsl_ty.as_str() {
                "f32" | "i32" | "u32" => (4, 4),
                "vec2<f32>" => (8, 8),
                "vec3<f32>" => (16, 16),
                "vec4<f32>" => (16, 16),
                _ => (4, 4),
            };
            if align > struct_align {
                struct_align = align;
            }
            let aligned_off = ((offset + align - 1) / align) * align;

            let field_ident = format_ident!("{}", uf.name);
            let write = match uf.wgsl_ty.as_str() {
                "f32" | "i32" | "u32" => {
                    quote! {
                        {
                            let b = self.#field_ident.to_le_bytes();
                            dst[#aligned_off as usize .. (#aligned_off as usize + 4)]
                                .copy_from_slice(&b);
                        }
                    }
                }
                "vec2<f32>" => {
                    quote! {
                        {
                            let src = bytemuck::cast_slice(&self.#field_ident);
                            dst[#aligned_off as usize .. (#aligned_off as usize + 8)]
                                .copy_from_slice(src);
                        }
                    }
                }
                "vec3<f32>" => {
                    quote! {
                        {
                            let src = bytemuck::cast_slice(&self.#field_ident);
                            dst[#aligned_off as usize .. (#aligned_off as usize + 12)]
                                .copy_from_slice(src);
                        }
                    }
                }
                "vec4<f32>" => {
                    quote! {
                        {
                            let src = bytemuck::cast_slice(&self.#field_ident);
                            dst[#aligned_off as usize .. (#aligned_off as usize + 16)]
                                .copy_from_slice(src);
                        }
                    }
                }
                _ => quote! {},
            };
            write_fields_ts.push(write);
            offset = aligned_off + slot_size;
        }

        let total_size = ((offset + struct_align - 1) / struct_align) * struct_align;

        quote! {
            let mut dst = vec![0u8; #total_size as usize];
            #(#write_fields_ts)*
            dst
        }
    } else {
        quote! { &[] }
    };

    let expanded = quote! {
        impl #struct_name {
            pub fn wgsl(&self) -> String {
                let mut s = String::new();
                #( s.push_str(#wgsl_parts); s.push_str("\n"); )*
                s
            }

            pub fn bindgroup_layout_entries(&self) -> Vec<wgpu::BindGroupLayoutEntry> {
                vec![#(#layout_entries),*]
            }

            /// Byte representation suitable for uploading a uniform buffer.
            pub fn to_std140_bytes(&self) -> Vec<u8> {
                #bytes_body
            }
        }
    };

    expanded.into()
}

// Rust syn::Type -> Option<&'static str>
fn rust_ty_to_wgsl_ty(ty: &syn::Type) -> Option<&'static str> {
    use syn::{Expr, ExprLit, Lit, Type, TypeArray, TypePath};
    match ty {
        Type::Array(TypeArray { elem, len, .. }) => {
            if let Type::Path(TypePath { path, .. }) = &**elem {
                if path.is_ident("f32") {
                    if let Expr::Lit(ExprLit {
                        lit: Lit::Int(litint),
                        ..
                    }) = len
                    {
                        match litint.base10_parse::<usize>().ok() {
                            Some(2) => Some("vec2<f32>"),
                            Some(3) => Some("vec3<f32>"),
                            Some(4) => Some("vec4<f32>"),
                            _ => None,
                        }
                    } else {
                        None
                    }
                } else {
                    None
                }
            } else {
                None
            }
        }
        Type::Path(TypePath { path, .. }) => {
            if path.is_ident("f32") {
                Some("f32")
            } else if path.is_ident("i32") {
                Some("i32")
            } else if path.is_ident("u32") {
                Some("u32")
            } else if path.segments.len() == 1 {
                let seg = &path.segments[0];
                match seg.ident.to_string().as_str() {
                    "Vec2" => Some("vec2<f32>"),
                    "Vec3" => Some("vec3<f32>"),
                    "Vec4" => Some("vec4<f32>"),
                    _ => None,
                }
            } else {
                None
            }
        }
        _ => None,
    }
}

// Option<TextureRef> or TextureRef
fn is_texture_ref(ty: &syn::Type) -> bool {
    use syn::{GenericArgument, PathArguments, Type, TypePath};
    match ty {
        Type::Path(TypePath { path, .. }) => {
            if path.is_ident("TextureRef") {
                return true;
            }

            if let Some(seg) = path.segments.last() {
                if seg.ident == "Option" {
                    if let PathArguments::AngleBracketed(ref ab) = seg.arguments {
                        for arg in &ab.args {
                            if let GenericArgument::Type(inner_ty) = arg {
                                if is_texture_ref(inner_ty) {
                                    return true;
                                }
                            }
                        }
                    }
                }
            }
            false
        }
        _ => false,
    }
}

fn is_uniform_attr(attr: &Attribute) -> bool {
    attr.path().is_ident("uniform")
}
