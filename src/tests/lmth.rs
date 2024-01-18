//! Tests for the `lmth!` macro.

use proc_macro2::TokenStream;
use quote::quote;

use super::_lmth;

/// Converts a `proc_macro2::TokenStream` into a `String`
fn token_str(input: TokenStream) -> String {
    input.to_string()
}

#[test]
/// functionality of `token_str()`
fn fn_token_str() {
    let quoted = token_str(quote! { macro! { these are some tokens } });
    let desired = token_str(quote! { macro! { these are some tokens } });

    eprintln!("quoted: {:?}", quoted);
    eprintln!("desired: {:?}", desired);

    assert_eq!(quoted, desired);
}

#[test]
/// (empty)
fn empty() {
    let output = token_str(_lmth(quote! {}));

    let desired = token_str(quote! {
        yew::prelude::html! {}
    });

    eprintln!("output: {:?}", output);
    eprintln!("desired: {:?}", desired);

    assert_eq!(output, desired);
}

#[test]
/// p { }
fn no_attr() {
    let output = token_str(_lmth(quote! {
        p { }
    }));

    let desired = token_str(quote! {
        yew::prelude::html! { <p></p> }
    });

    eprintln!("output: {:?}", output);
    eprintln!("desired: {:?}", desired);

    assert_eq!(output, desired);
}

#[test]
/// img
fn void_no_attr() {
    let output = token_str(_lmth(quote! {
        img
    }));

    let desired = token_str(quote! {
        yew::prelude::html! { <img /> }
    });

    eprintln!("output: {:?}", output);
    eprintln!("desired: {:?}", desired);

    assert_eq!(output, desired);
}

#[test]
/// img ( src: "/path/to/pic.jpg" )
fn void_signle_dyn_attr() {
    let output = token_str(_lmth(quote! {
        img ( src: "/path/to/pic.jpg" )
    }));

    let desired = token_str(quote! {
        yew::prelude::html! { <img src={"/path/to/pic.jpg"} /> }
    });

    eprintln!("output: {:?}", output);
    eprintln!("desired: {:?}", desired);

    assert_eq!(output, desired);
}

#[test]
/// button ( type: "button" ) { }
fn single_dyn_attr_no_content() {
    let output = token_str(_lmth(quote! {
        button ( type: "button" ) { }
    }));

    let desired = token_str(quote! {
        yew::prelude::html! {
            <button type={"button"}></button>
        }
    });

    eprintln!("output: {:?}", output);
    eprintln!("desired: {:?}", desired);

    assert_eq!(output, desired);
}

#[test]
/// button ( type: "button" ) { "I'm a button" }
fn single_dyn_attr_with_content() {
    let output = token_str(_lmth(quote! {
        button ( type: "button" ) { "I'm a button" }
    }));

    let desired = token_str(quote! {
        yew::prelude::html! {
            <button type={"button"}>{ "I'm a button" }</button>
        }
    });

    eprintln!("output: {:?}", output);
    eprintln!("desired: {:?}", desired);

    assert_eq!(output, desired);
}

#[test]
/// div {
///     p { "paragraph 1" }
///     p { "paragraph 2" }
///     p { "paragraph 3" }
/// }
fn child_elems() {
    let output = token_str(_lmth(quote! {
        div {
            p { "paragraph 1" }
            p { "paragraph 2" }
            p { "paragraph 3" }
        }
    }));

    let desired = token_str(quote! {
        yew::prelude::html! {
            <div>
                <p>{ "paragraph 1" }</p>
                <p>{ "paragraph 2" }</p>
                <p>{ "paragraph 3" }</p>
            </div>
        }
    });

    eprintln!("output: {:?}", output);
    eprintln!("desired: {:?}", desired);

    assert_eq!(output, desired);
}

#[test]
/// div {
///     div {
///         div {
///             p { "deeply nested" }
///
///         }
///     }
/// }
fn deeply_nested() {
    let output = token_str(_lmth(quote! {
        div {
            div {
                div {
                    p { "deeply nested" }
                }
            }
        }
    }));

    let desired = token_str(quote! {
        yew::prelude::html! {
            <div>
                <div>
                    <div>
                        <p>{ "deeply nested" }</p>
                    </div>
                </div>
            </div>
        }
    });

    eprintln!("output: {:?}", output);
    eprintln!("desired: {:?}", desired);

    assert_eq!(output, desired);
}

#[test]
/// div {
///     div {
///         div {
///             p { "nested" }
///         }
///         p { "composed inner" }
///     }
///     p { "composed outer" }
/// }
fn compose_and_nested() {
    let output = token_str(_lmth(quote! {
        div {
            div {
                div {
                    p { "nested" }
                }
                p { "composed inner" }
            }
            p { "composed outer" }
        }
    }));

    let desired = token_str(quote! {
        yew::prelude::html! {
            <div>
                <div>
                    <div>
                        <p>{ "nested" }</p>
                    </div>
                    <p>{ "composed inner" }</p>
                </div>
                <p>{ "composed outer" }</p>
            </div>
        }
    });

    eprintln!("output: {:?}", output);
    eprintln!("desired: {:?}", desired);

    assert_eq!(output, desired);
}

#[test]
/// ! {}
fn empty_fragment() {
    let output = token_str(_lmth(quote! {
        ! { }
    }));

    let desired = token_str(quote! {
        yew::prelude::html! {
            <></>
        }
    });

    eprintln!("output: {:?}", output);
    eprintln!("desired: {:?}", desired);

    assert_eq!(output, desired);
}

#[test]
/// ! {
///    p { "paragraph" }
/// }
fn fragment_single_content() {
    let output = token_str(_lmth(quote! {
        ! {
            p { "paragraph" }
        }
    }));

    let desired = token_str(quote! {
        yew::prelude::html! {
            <>
                <p>{ "paragraph" }</p>
            </>
        }
    });

    eprintln!("output: {:?}", output);
    eprintln!("desired: {:?}", desired);

    assert_eq!(output, desired);
}
