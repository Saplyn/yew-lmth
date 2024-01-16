// TODO

use lmth::{ir::LmthNode, lmth_act};
use proc_macro as pm;
use quote::quote;
use syn::parse_macro_input;

//~ Sub-modules
mod lmth;
#[cfg(test)]
mod tests;

// TODO
#[proc_macro]
pub fn lmth(input: pm::TokenStream) -> pm::TokenStream {
    if input.is_empty() {
        return quote! {
            yew::prelude::html! {}
        }
        .into();
    }

    let node = parse_macro_input!(input as LmthNode);

    lmth_act(node).into()
}
