use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(StructName)]
pub fn struct_name_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    impl_struct_name(&ast)
}

fn impl_struct_name(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl #name {
            fn simple_name(&self) -> &'static str {
                stringify!(#name)
            }
        }
    };
    gen.into()
}
