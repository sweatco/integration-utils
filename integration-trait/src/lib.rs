extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, parse_str, FnArg, Ident, ItemTrait, ReturnType, TraitItem, TraitItemFn};

/// Create interface trait suitable for usage in integration tests
#[proc_macro_attribute]
pub fn make_integration_version(_args: TokenStream, stream: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(stream as ItemTrait);

    let trait_name = &input.ident;

    let async_trait_name = Ident::new(&format!("{trait_name}Integration"), trait_name.span());

    let async_methods: Vec<_> = input
        .items
        .iter_mut()
        .filter_map(|item| {
            if let TraitItem::Fn(method) = item {
                let async_method = asyncify_method(method);
                Some(async_method)
            } else {
                None
            }
        })
        .collect();

    quote! {

        #input

        #[cfg(not(target_arch = "wasm32"))]
        pub trait #async_trait_name {
            #(#async_methods)*
        }
    }
    .into()
}

fn asyncify_method(trait_method: &mut TraitItemFn) -> proc_macro2::TokenStream {
    let mut method = trait_method.clone();

    // method.sig.asyncness = Some(Default::default());

    let mut ret = if matches!(method.sig.output, ReturnType::Default) {
        "()".to_string()
    } else {
        let ret = method.sig.output.to_token_stream().to_string();

        let ret = ret.strip_prefix("-> ").unwrap();

        ret.to_string()
    };

    if ret == "Self" {
        let self_arg: FnArg = parse_str("&self").unwrap();
        method.sig.inputs.insert(0, self_arg);
        ret = "()".to_string();
    }

    if ret.starts_with("PromiseOrValue <") {
        let start = ret.find('<').unwrap();
        let end = ret.find('>').unwrap();

        ret = ret[start + 1..end].to_string();
    }

    let ret: Result<ReturnType, _> = parse_str(&format!("-> integration_utils::contract_call::ContractCall<{ret}>"));

    method.sig.output = ret.unwrap();

    if let Some(attr) = method.attrs.first() {
        let attr = attr.path().to_token_stream().to_string();
        method.attrs = vec![];
        trait_method.attrs = vec![];

        match attr.as_str() {
            "update" => method.sig.inputs.push(parse_str("code: Vec<u8>").unwrap()),
            "doc" => (),
            _ => unreachable!("Invalid attribute: '{attr}'. Only 'update' is supported."),
        }
    }

    method.to_token_stream()
}
