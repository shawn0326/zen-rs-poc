use proc_macro::TokenStream;
use quote::{ToTokens, quote};
use syn::{DeriveInput, parse_macro_input};

#[proc_macro_derive(UniformBuffer)]
pub fn derive_uniforms(_input: TokenStream) -> TokenStream {
    quote::quote! {
        // todo
        // 只作用于struct
        // struct在渲染器内部将作为UniformBuffer使用，为它实现一些辅助方法
        // 实现一个
    }
    .into()
}

#[proc_macro_derive(Material, attributes(buffer, texture, sampler, binding))]
pub fn derive_material(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let ident = ast.ident;
    let generics = ast.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    // 扫描字段，找出第一个 #[texture]/#[sampler] 的类型；找不到则用 ()
    let mut tex_ty = quote! { () };
    let mut samp_ty = quote! { () };

    if let syn::Data::Struct(data) = &ast.data {
        for f in data.fields.iter() {
            for attr in &f.attrs {
                if attr.path().is_ident("texture") {
                    tex_ty = f.ty.to_token_stream();
                } else if attr.path().is_ident("sampler") {
                    samp_ty = f.ty.to_token_stream();
                }
            }
        }
    }

    // 最小实现：为空 impl（依赖于 Material trait 方法有默认实现）
    // 注意：要求在宿主 crate 中可见 ::zen_material::Material 路径。
    // 如需在定义 trait 的同一 crate 内使用派生，请在该 crate 中添加：
    //   pub use crate as zen_material;
    let expanded = quote! {
        impl #impl_generics zen_material::Material for #ident #ty_generics #where_clause {
            type Texture = #tex_ty;
            type Sampler = #samp_ty;

            fn schema(&self) -> &'static [zen_material::MaterialBindingDesc] {
                &[]
            }
        }
    };

    expanded.into()
}
