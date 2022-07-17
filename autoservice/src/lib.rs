#![feature(proc_macro_span)]
extern crate proc_macro;

use proc_macro::{Span, TokenStream};

use syn::parse_macro_input;

#[proc_macro]
pub fn auto_service(ast: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(ast as proc_macro2::TokenStream);
    match autoservice_codegen::auto_service(
        ast,
        &Span::call_site()
            .source_file()
            .path()
            .parent()
            .unwrap()
            .to_string_lossy(),
    ) {
        Ok(tokens) => tokens.into(),
        Err(err) => TokenStream::from(err.to_compile_error()),
    }
}
