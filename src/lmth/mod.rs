//! The `lmth!` macro implementation.

use proc_macro2::TokenStream;
use quote::quote;

use crate::lmth::ir::{ElemAttr, ElemAttrVal, ElemTag};

use self::ir::{Elem, ElemAttrBind, ElemAttrCopy, LmthNode};

pub mod ir;
pub mod parse;

pub fn lmth_act(node: LmthNode) -> TokenStream {
    let LmthNode::Elem(elem) = node else {
        panic!("The root of `lmth!` must be an element or a yew fragment `! {{}}`");
    };

    let html = elem_act(elem);
    quote! {
        yew::prelude::html! {
            #html
        }
    }
}

fn node_act(node: LmthNode) -> TokenStream {
    match node {
        LmthNode::Elem(elem) => elem_act(elem),
        LmthNode::Block(block) => quote! {
            #block
        },
        LmthNode::LitStr(litstr) => quote! {
            { #litstr }
        },
    }
}

fn elem_act(elem: Elem) -> TokenStream {
    let mut quoted_content = None;
    if let Some(content) = elem.content {
        quoted_content = Some(quote! {});
        for node in content {
            let quoted_node = node_act(node);
            quoted_content = Some(quote! {
                #quoted_content
                #quoted_node
            })
        }
    };

    let mut quoted_attrs = quote! {};
    if let Some(attrs) = elem.attrs {
        for attr in attrs {
            quoted_attrs = match attr {
                ElemAttr::Bind(ElemAttrBind { key, val }) => match val {
                    ElemAttrVal::Block(block) => {
                        quote! {
                            #quoted_attrs #key=#block
                        }
                    }
                    ElemAttrVal::Expr(expr) => {
                        quote! {
                            #quoted_attrs #key={ #expr }
                        }
                    }
                },
                ElemAttr::Copy(ElemAttrCopy { key, litstr }) => quote! {
                    #quoted_attrs #key=#litstr
                },
                ElemAttr::Sugar(ident) => quote! {
                    #quoted_attrs { #ident }
                },
            }
        }
    }

    match (elem.tag, quoted_attrs, quoted_content) {
        (ElemTag::Regular(tag), attrs, Some(content)) => quote! {
            <#tag #attrs> #content </#tag>
        },
        (ElemTag::Regular(tag), attrs, None) => quote! {
            <#tag #attrs />
        },
        (ElemTag::Custom(comp), attrs, Some(content)) => quote! {
            <#comp #attrs> #content </#comp>
        },
        (ElemTag::Custom(comp), attrs, None) => quote! {
            <#comp #attrs />
        },
        (ElemTag::Fragment, _, Some(content)) => quote! {
            <> #content </>
        },
        (ElemTag::Fragment, _, None) => quote! {
            <> </>
        },
    }
}
