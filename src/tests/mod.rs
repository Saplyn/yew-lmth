//! Tests for `yew-lmth` crate.

use proc_macro2::TokenStream;
use quote::quote;
use syn::parse2;

use crate::lmth::lmth_act;

#[cfg(test)]
mod lmth;

/// This function serves for testing & debuging perpose.
/// its implementation should be keep in sync with `lmth!()`.
/// In other words, this function is `lmth!()` but for testing.
fn _lmth(input: TokenStream) -> TokenStream {
    eprintln!("input: {:#?}", input);

    if input.is_empty() {
        return quote! {
            yew::prelude::html! {}
        };
    }

    let node = parse2(input).expect("expected valid lmth synax");

    lmth_act(node)
}
