use std::path::Path;

use proc_macro2::{Span, TokenStream};
use quote::{quote, TokenStreamExt};
use syn;
use syn::{Ident, LitStr};
use syn::parse::{Parse, ParseStream};
use syn::parse2;
use syn::spanned::Spanned;
use syn::Token;

struct AutoServiceInput {
    app_ident: Ident,
    dir: String,
}

impl Parse for AutoServiceInput {
    fn parse(input: ParseStream) -> syn::parse::Result<Self> {
        Ok(Self {
            app_ident: input.parse()?,
            dir: {
                input.parse::<Token![;]>().expect("couldn't parse semicolon");
                input.parse::<LitStr>()?.value()
            },
        })
    }
}

pub fn auto_service(ast: TokenStream, call_dir: &str) -> Result<TokenStream, syn::Error> {
    let input = parse2::<AutoServiceInput>(ast)?;
    let app_ident = input.app_ident;
    let mut result = TokenStream::new();

    let inputs_dir = Path::new(call_dir).join(&input.dir).canonicalize().map_err(|_| syn::Error::new(result.span(), "couldn't find inputs directory"))?;
    for file in std::fs::read_dir(inputs_dir).expect("Invalid directory!")
    {
        if let Ok(file) = file {
            let file_name = file.file_name();
            let file_name = file_name.to_string_lossy();
            if !file_name.ends_with(".rs") {
                continue;
            }
            let file_name = &file_name[..file_name.len() - 3];
            let ident_1 = Ident::new(&input.dir, Span::call_site());
            let ident_2 = Ident::new(file_name, Span::call_site());

            result.append_all(quote! {
                #app_ident = #app_ident.service(#ident_1::#ident_2::#ident_2);
            });
        }
    }

    Ok(result)
}
