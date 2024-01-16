//! Intermediate representation for `lmth!` macro syntax.

use proc_macro2::Ident;
use syn::{Block, Expr, LitStr, Type};

// TODO
#[derive(Debug)]
pub enum LmthNode {
    Elem(Elem),     // e.g. tag ( ... ) { ... }
    Block(Block),   // e.g. { expr; expr; expr }
    LitStr(LitStr), // e.g. "litstr"
}

// TODO
pub enum LmthNodeType {
    Elem,
    Block,
    LitStr,
}

// TODO
#[derive(Debug)]
pub struct Elem {
    pub tag: ElemTag,                   // `tag` (attr: "val") { ... }
    pub attrs: Option<Vec<ElemAttr>>,   // tag `(attr: "val")` { ... }
    pub content: Option<Vec<LmthNode>>, // tag (attr: "val") `{ ... }`
}

// TODO
#[derive(Debug)]
pub enum ElemTag {
    Regular(Ident), // e.g. p (`<p></p>`), img(`<img />`)
    Custom(Type),   // e.g. App (`<App></App>`), Switch<Route> (`<Switch<Route> />`)
    Fragment,       // yew's fragments ! (empty tag `<></>`)
}

// TODO
pub enum ElemTagType {
    Regular,
    Custom,
    Fragment,
}

// TODO
#[derive(Debug)]
pub struct ElemAttr {
    pub key: Ident,       // `attr`: val
    pub val: ElemAttrVal, // attr: `val`
}

// TODO
#[derive(Debug)]
pub enum ElemAttrVal {
    Expr(Expr),   // attr: expr, attr: "lit"
    Block(Block), // attr: { expr; expr; expr }
}
