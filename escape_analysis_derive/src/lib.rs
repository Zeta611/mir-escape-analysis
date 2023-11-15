use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(EscapeAnalysis)]
pub fn escape_analysis_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_escape_analysis_macro(&ast)
}

fn impl_escape_analysis_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl EscapeAnalysis for #name {}
    };
    gen.into()
}
