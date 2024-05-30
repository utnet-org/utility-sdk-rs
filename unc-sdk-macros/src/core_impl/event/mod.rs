use proc_macro::TokenStream;
use proc_macro2::Span;

use quote::quote;

use darling::ast::NestedMeta;
use darling::Error;
use darling::FromMeta;
use syn::{parse_quote, ItemEnum, LitStr};

#[derive(Default, FromMeta, Clone, Debug)]
pub struct MacroConfig {
    pub event_json: Option<EventsConfig>,
}
#[derive(Default, FromMeta, Clone, Debug)]
pub struct EventsConfig {
    standard: Option<String>,
}

/// this function is used to inject serialization macros and the `unc_sdk::EventMetadata` macro.
/// In addition, this function extracts the event's `standard` value and injects it as a constant to be used by
/// the `unc_sdk::EventMetadata` derive macro
pub(crate) fn unc_events(attr: TokenStream, item: TokenStream) -> TokenStream {
    // get standard from attr args

    let attr_args = match NestedMeta::parse_meta_list(attr.into()) {
        Ok(v) => v,
        Err(e) => {
            return TokenStream::from(Error::from(e).write_errors());
        }
    };

    let args = match MacroConfig::from_list(&attr_args) {
        Ok(v) => v,
        Err(e) => {
            return TokenStream::from(e.write_errors());
        }
    };
    if let Some(standard) = args.event_json.and_then(|event_json| event_json.standard) {
        if let Ok(mut input) = syn::parse::<ItemEnum>(item) {
            let name = &input.ident;
            let standard_name = format!("{}_event_standard", name);
            let standard_ident = syn::Ident::new(&standard_name, Span::call_site());
            // UncEvent Macro handles implementation
            input.attrs.push(
                parse_quote! (#[derive(::unc_sdk::serde::Serialize, ::unc_sdk::EventMetadata)]),
            );
            input.attrs.push(parse_quote! (#[serde(crate="::unc_sdk::serde")]));
            input.attrs.push(parse_quote! (#[serde(tag = "event", content = "data")]));
            input.attrs.push(parse_quote! (#[serde(rename_all = "snake_case")]));

            TokenStream::from(quote! {
                const #standard_ident: &'static str = #standard;
                #input
            })
        } else {
            TokenStream::from(
                syn::Error::new(
                    Span::call_site(),
                    "`#[unc_bindgen(event_json(standard = \"uipXXX\"))]` can only be used as an attribute on enums.",
                )
                .to_compile_error(),
            )
        }
    } else {
        TokenStream::from(
            syn::Error::new(
                Span::call_site(),
                "Unc events must have a `standard` value as an argument for `event_json` in the `unc_bindgen` arguments. The value must be a string literal, e.g. \"uip999\", \"mintbase-marketplace\".",
            )
            .to_compile_error(),
        )
    }
}

/// This function returns the `version` value from `#[event_version("x.x.x")]`.
/// used by `unc_sdk::EventMetadata`
pub(crate) fn get_event_version(var: &syn::Variant) -> Option<LitStr> {
    for attr in var.attrs.iter() {
        if attr.path().is_ident("event_version") {
            return attr.parse_args::<LitStr>().ok();
        }
    }
    None
}
