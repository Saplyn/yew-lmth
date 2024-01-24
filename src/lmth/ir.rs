//! Intermediate representation for `lmth!` macro syntax.

use proc_macro2::Ident;
use syn::{Block, Expr, LitStr, Type};

/// A node of `lmth!` macro.
#[derive(Debug)]
pub enum LmthNode {
    /// e.g. `tag ( ... ) { ... }`
    Elem(Elem),
    /// e.g. `{ expr; expr; expr }`
    Block(Block),
    /// e.g. `"litstr"`
    LitStr(LitStr),
}

/// Type of `LmthNode`.
#[derive(Debug)]
pub enum LmthNodeType {
    Elem,
    Block,
    LitStr,
}

/// An element.
#[derive(Debug)]
pub struct Elem {
    /// **`tag`** `(attr: "val")` `{ ... }`
    pub tag: ElemTag,
    /// `tag` **`(attr: "val")`** `{ ... }`
    pub attrs: Option<Vec<ElemAttr>>,
    /// `tag` `(attr: "val")` **`{ ... }`**
    pub content: Option<Vec<LmthNode>>,
}

/// An element tag.
#[derive(Debug, PartialEq)]
pub enum ElemTag {
    /// e.g. p (`<p></p>`), img(`<img />`)
    Regular(Ident),
    /// e.g. App (`<App></App>`), Switch<Route> (`<Switch<Route> />`)
    Custom(Type),
    /// yew's fragments ! (empty tag `<></>`)
    Fragment,
}

/// Type of `ElemTag`.
pub enum ElemTagType {
    Regular,
    Custom,
    Fragment,
}

/// An element's attribute.
#[derive(Debug)]
pub enum ElemAttr {
    /// e.g. `attr: expr`
    Bind(ElemAttrBind),
    /// e.g. `attr="litstr"`
    Copy(ElemAttrCopy),
    /// e.g. `attr (shorthand for {attr})`
    Sugar(Ident),
}

/// An element's attribute: bind syntax.
#[derive(Debug)]
pub struct ElemAttrBind {
    /// **`attr`**`: val`
    pub key: Ident,
    /// `attr: `**`val`**
    pub val: ElemAttrVal,
}

/// An element's attribute: copy syntax.
#[derive(Debug)]
pub struct ElemAttrCopy {
    /// **`attr`**`="litstr"`
    pub key: Ident,
    /// `attr=`**`"litstr"`**
    pub litstr: LitStr,
}

/// An element's attribute's value.
#[derive(Debug)]
pub enum ElemAttrVal {
    /// `attr: expr`, `attr: "lit"`
    Expr(Expr),
    /// `attr: { expr; expr; expr }`
    Block(Block),
}
