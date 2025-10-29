use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, parse_macro_input};

#[proc_macro_derive(Shader, attributes(uniform))]
pub fn shader_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = &input.ident;

    let fields = match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => &fields.named,
            _ => panic!("Only named fields supported"),
        },
        _ => panic!("Only structs supported"),
    };

    let mut wgsl_fields = Vec::new();
    for field in fields {
        let field_name = field.ident.as_ref().unwrap().to_string();
        let ty = quote!(#field.ty).to_string();
        let wgsl_ty = if ty.contains("Vec4") || ty.contains("Color4") {
            "vec4<f32>"
        } else if ty.contains("Vec3") {
            "vec3<f32>"
        } else if ty.contains("Vec2") {
            "vec2<f32>"
        } else if ty.contains("f32") {
            "f32"
        } else if ty.contains("i32") {
            "i32"
        } else if ty.contains("u32") {
            "u32"
        } else {
            continue;
        };
        wgsl_fields.push(format!("    {}: {},", field_name, wgsl_ty));
    }
    let wgsl_struct = format!("struct {} {{\n{}\n}};", struct_name, wgsl_fields.join("\n"));

    let expanded = quote! {
        impl #struct_name {
            pub fn wgsl_struct() -> &'static str {
                #wgsl_struct
            }
        }
    };

    expanded.into()
}
