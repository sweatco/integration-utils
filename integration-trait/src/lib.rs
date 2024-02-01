extern crate proc_macro;

use std::str::FromStr;

use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{
    __private::TokenStream2, parse_macro_input, parse_quote, parse_str, FnArg, Ident, ItemFn, ItemTrait, Pat,
    ReturnType, TraitItem, TraitItemFn,
};

/// Create interface trait suitable for usage in integration tests
#[proc_macro_attribute]
pub fn make_integration_version(_args: TokenStream, stream: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(stream as ItemTrait);

    let crate_name = std::env::var("CARGO_PKG_NAME")
        .unwrap_or_else(|_| panic!("let crate_name = std::env::var(\"CARGO_PKG_NAME\")."));

    let Some(index) = crate_name.find("-model") else {
        panic!("Some(index) = crate_name.find(");
    };

    let contract_name = to_camel_case(&crate_name[..index]);

    let trait_name = &input.ident;

    let contract = Ident::new(&format!("{contract_name}Contract"), trait_name.span());

    let integration_trait_name = Ident::new(&format!("{trait_name}Integration"), trait_name.span());

    let integration_trait_methods: Vec<TraitItemFn> = input
        .items
        .iter_mut()
        .filter_map(|item| {
            if let TraitItem::Fn(method) = item {
                let async_method = convert_method_to_integration_trait(method);
                Some(async_method)
            } else {
                None
            }
        })
        .collect();

    let implementation_methods: Vec<ItemFn> = integration_trait_methods
        .iter()
        .map(convert_method_to_implementation)
        .collect();

    quote! {

        #input

        #[cfg(feature = "integration-test")]
        pub trait #integration_trait_name {
            #(#integration_trait_methods)*
        }

        #[cfg(feature = "integration-test")]
        impl #integration_trait_name for #contract<'_> {
            #(#implementation_methods)*
        }
    }
    .into()
}

fn convert_method_to_implementation(trait_method: &TraitItemFn) -> ItemFn {
    let fn_name = trait_method.sig.ident.clone();
    let fn_args = trait_method.sig.inputs.clone();
    let fn_ret = trait_method.sig.output.clone();

    let fn_name_str = TokenStream2::from_str(&format!("\"{fn_name}\"")).expect("Failed to extract method name");

    let call_args = if fn_args.len() > 1 {
        let mut args_quote = quote!();

        for arg in fn_args.iter().skip(1) {
            let FnArg::Typed(arg) = arg else {
                panic!("FnArg::Typed(arg) = arg");
            };

            let Pat::Ident(pat_ident) = &*arg.pat else {
                panic!("Pat::Ident(ident) = &arg.pat");
            };

            let ident = &pat_ident.ident;

            let string_ident = TokenStream2::from_str(&format!("\"{ident}\"")).expect("Failed to extract method name");

            args_quote = quote! {
                #args_quote
                #string_ident : #ident,
            }
        }

        quote! {
            .args_json(near_sdk::serde_json::json!({
                #args_quote
            })).unwrap()
        }
    } else {
        quote!()
    };

    let result: ItemFn = parse_quote!(
        fn #fn_name(#fn_args) #fn_ret {
            use integration_utils::integration_contract::IntegrationContract;
            self.make_call(#fn_name_str) #call_args
        }
    );

    result.clone()
}

fn convert_method_to_integration_trait(trait_method: &mut TraitItemFn) -> TraitItemFn {
    let mut method = trait_method.clone();

    let mut ret = if matches!(method.sig.output, ReturnType::Default) {
        "()".to_string()
    } else {
        let ret = method.sig.output.to_token_stream().to_string();
        let ret = ret.strip_prefix("-> ").unwrap();
        ret.to_string()
    };

    let self_arg: FnArg = parse_str("&self").unwrap();

    if ret == "Self" {
        method.sig.inputs.insert(0, self_arg);
        ret = "()".to_string();
    } else {
        method.sig.inputs[0] = self_arg;
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

    method
}

fn to_camel_case(input: &str) -> String {
    input
        .split('-')
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(first_char) => first_char.to_uppercase().collect::<String>() + chars.as_str(),
            }
        })
        .collect()
}
