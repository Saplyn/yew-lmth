// TODO

use lmth::{ir::LmthNode, lmth_act};
use proc_macro as pm;
use quote::quote;
use syn::parse_macro_input;

//~ Sub-modules
mod lmth;
#[cfg(test)]
mod tests;

/// Macro for writing HTML-like syntax, which will be translated
/// into a corresponding `yew::prelude::html!()` macro.
///
/// # Syntax
///
/// ## Tags
///
/// | `lmth!` syntax        | meaning                         | `html!` sytax              |
/// | --------------------- | ------------------------------- | -------------------------- |
/// | `! { ... }`           | Yew's fragment                  | `<> ... </>`               |
/// | `tag (attrs) { ... }` | Tag with attributes and content | `<tag attrs>{ ... }</tag>` |
/// | `tag (attrs)`         | Void tag with attributes        | `<tag attrs />`            |
/// | `tag { ... }`         | Tag with content                | `<tag>{ ... }</tag>`       |
/// | `tag`                 | Void tag with no attribute      | `<tag />`                  |
///
/// ## Attributes
///
/// Attributes are separated by commas: `tag (attr: val, attr: val, ...) { ... }`
///
/// | `lmth!` syntax  | meaning                                | `html!` sytax  |
/// | --------------- | -------------------------------------- | -------------- |
/// | `attr: expr`    | Attribute with expression as value     | `attr={expr}`  |
/// | `attr: {code}`  | Attribute with code block as value     | `attr={code}`  |
/// | `attr="litstr"` | Attribute with literal string as value | `attr="litstr"`|
/// | `attr`          | Shorthand for `{attr}` in yew          | `{attr}`       |
///
/// ## Content
///
/// | `lmth!` syntax  | meaning                   | `html!` sytax     |
/// | --------------- | ------------------------- | ----------------- |
/// | `{code}`        | Code as content           | `{code}`          |
/// | `"litstr"`      | Literal string as content | `"litstr"`        |
/// | `tag ...`       | Tag                       | corresponding tag |
///
/// # Example
///
/// ```rust
/// use yew_lmth::lmth;
///
/// lmth! {
///     div (class="container") {
///        h1 { "Hello, world!" }
///        button (onclick: handle_click()) { "Click me!" }
///    }
/// }
///
/// // expands to:
/// // yew::prelude::html! {
/// //     <div class="container">
/// //         <h1>{ "Hello, world!" }</h1>
/// //         <button onclick={handle_click()}>{ "Click me!" }</button>
/// //     </div>
/// // }
/// ```
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
