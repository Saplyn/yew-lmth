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
#[derive(Debug, PartialEq)]
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
pub enum ElemAttr {
    Bind(ElemAttrBind), // e.g. attr: expr
    Copy(ElemAttrCopy), // e.g. attr="litstr"
    Sugar(Ident),       // e.g. attr (shorthand for {attr})
}

// TODO
#[derive(Debug)]
pub struct ElemAttrBind {
    pub key: Ident,       // `attr`: val
    pub val: ElemAttrVal, // attr: `val`
}

// TODO
#[derive(Debug)]
pub struct ElemAttrCopy {
    pub key: Ident,     // `attr`="litstr"
    pub litstr: LitStr, // attr=`"litstr"`
}

// TODO
#[derive(Debug)]
pub enum ElemAttrVal {
    Expr(Expr),   // attr: expr, attr: "lit"
    Block(Block), // attr: { expr; expr; expr }
}
