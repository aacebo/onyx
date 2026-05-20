use proc_macro::TokenStream;
use zyn::{ToTokens, syn, zyn};

use zyn::syn::{
    LitStr, Token,
    parse::{Parse, ParseStream},
};

struct ResourceInput {
    url: LitStr,
    cache: Option<syn::Expr>,
}

impl Parse for ResourceInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            url: input.parse()?,
            cache: if input.peek(Token![=>]) {
                input.parse::<Token![=>]>()?;
                Some(input.parse()?)
            } else {
                None
            },
        })
    }
}

#[proc_macro]
pub fn parse(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as ResourceInput);
    let url = input.url.value();

    if !(url.starts_with("file://")
        || url.starts_with("http://")
        || url.starts_with("https://")
        || url.starts_with("hf://"))
    {
        return syn::Error::new_spanned(
            &input.url,
            "unsupported resource scheme; expected file://, http://, https://, or hf://",
        )
        .to_compile_error()
        .into();
    }

    let cache: syn::Expr = match input.cache {
        Some(expr) => expr,
        None => syn::parse_quote!(::std::env::temp_dir()),
    };

    zyn! {
        @match (input.url.value().as_str()) {
            s if s.starts_with("file://") => {
                <::onyx_core::resource::LocalResource as ::core::str::FromStr>::from_str({{ input.url }})
                    .map_err(::onyx_core::error::ResourceError::parse)?
            },
            s if s.starts_with("http://") || s.starts_with("https://") => {
                ::onyx_core::resource::RemoteResource::new({{ input.url }}, {{ cache }})
            },
            s if cfg!(feature = "huggingface") && s.starts_with("hf://") => {
                <::onyx_core::resource::HFResource as ::core::str::FromStr>::from_str({{ input.url }})
                    .map_err(::onyx_core::error::ResourceError::parse)?
            },
            _ => { ::core::unreachable!() },
        }
    }.into_token_stream().into()
}
