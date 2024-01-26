//! The `lmth!` macro implementation.

use proc_macro2::TokenStream;
use quote::quote;

use crate::lmth::ir::{ElemAttr, ElemAttrVal, ElemTag, IfCond};

use self::ir::{Elem, ElemAttrBind, ElemAttrCopy, IfLetCond, LmthNode};

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
        LmthNode::IfCond(if_cond) => if_act(if_cond),
        LmthNode::IfLetCond(if_let_cond) => if_let_act(if_let_cond),
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
        (ElemTag::Dynamic(block), attrs, Some(content)) => quote! {
            <@#block #attrs> #content </@>
        },
        (ElemTag::Dynamic(block), attrs, None) => quote! {
            <@#block #attrs />
        },
        (ElemTag::Fragment, _, Some(content)) => quote! {
            <> #content </>
        },
        (ElemTag::Fragment, _, None) => quote! {
            <> </>
        },
    }
}

fn if_act(if_cond: IfCond) -> TokenStream {
    let IfCond {
        cond,
        then_branch,
        else_branch,
    } = if_cond;

    let mut quoted_then_branch = quote! {};
    for node in then_branch {
        let quoted_node = node_act(node);
        quoted_then_branch = quote! {
            #quoted_then_branch
            #quoted_node
        }
    }

    let quoted_else_branch = if let Some(else_branch) = else_branch {
        let mut quoted_else_branch = quote! {};
        for node in else_branch {
            let quoted_node = node_act(node);
            quoted_else_branch = quote! {
                #quoted_else_branch
                #quoted_node
            }
        }
        Some(quoted_else_branch)
    } else {
        None
    };

    if let Some(quoted_else_branch) = quoted_else_branch {
        quote! {
            if #cond {
                #quoted_then_branch
            } else {
                #quoted_else_branch
            }
        }
    } else {
        quote! {
            if #cond {
                #quoted_then_branch
            }
        }
    }
}

fn if_let_act(if_let_cond: IfLetCond) -> TokenStream {
    let IfLetCond {
        pat,
        expr,
        then_branch,
        else_branch,
    } = if_let_cond;

    let mut quoted_then_branch = quote! {};
    for node in then_branch {
        let quoted_node = node_act(node);
        quoted_then_branch = quote! {
            #quoted_then_branch
            #quoted_node
        }
    }

    let quoted_else_branch = if let Some(else_branch) = else_branch {
        let mut quoted_else_branch = quote! {};
        for node in else_branch {
            let quoted_node = node_act(node);
            quoted_else_branch = quote! {
                #quoted_else_branch
                #quoted_node
            }
        }
        Some(quoted_else_branch)
    } else {
        None
    };

    if let Some(quoted_else_branch) = quoted_else_branch {
        quote! {
            if let #pat = #expr {
                #quoted_then_branch
            } else {
                #quoted_else_branch
            }
        }
    } else {
        quote! {
            if let #pat = #expr {
                #quoted_then_branch
            }
        }
    }
}
