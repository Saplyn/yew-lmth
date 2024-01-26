//! Intermediate representation for `lmth!` macro syntax.

use proc_macro2::Ident;
use syn::{Expr, ExprBlock, LitStr, Pat, Type};

/// A node of `lmth!` macro.
#[derive(Debug)]
pub enum LmthNode {
    /// e.g. `tag ( ... ) { ... }`
    Elem(Elem),
    /// e.g. `{ expr; expr; expr }`
    Block(ExprBlock),
    /// e.g. `"litstr"`
    LitStr(LitStr),
    /// e.g. `if cond {} else {}`
    IfCond(IfCond),
    /// e.g. `if let Some(x) = opt {} else {}`
    IfLetCond(IfLetCond),
}

/// Type of `LmthNode`.
#[derive(Debug)]
pub enum LmthNodeType {
    Elem,
    Block,
    LitStr,
    IfCond,
    IfLetCond,
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
    /// e.g. @{code} (`<@{code}></@>`)
    Dynamic(ExprBlock),
    /// yew's fragments ! (empty tag `<></>`)
    Fragment,
}

/// Type of `ElemTag`.
pub enum ElemTagType {
    Regular,
    Custom,
    Dynamic,
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
    Block(ExprBlock),
}

/// An `if` condition.
#[derive(Debug)]
pub struct IfCond {
    /// `if `**`cond`**` {} else {}`
    pub cond: Expr,
    /// `if cond `**`{}`**` else {}`
    pub then_branch: Vec<LmthNode>,
    /// `if cond {} else `**`{}`**
    pub else_branch: Option<Vec<LmthNode>>,
}

/// An `if let` condition.
#[derive(Debug)]
pub struct IfLetCond {
    /// `let `**`Some(x)`**` = opt {} else {}`
    pub pat: Pat,
    /// `let Some(x) = `**`opt`**` {} else {}`
    pub expr: Expr,
    /// `let Some(x) = opt `**`{}`**` else {}`
    pub then_branch: Vec<LmthNode>,
    /// `let Some(x) = opt {} else `**`{}`**
    pub else_branch: Option<Vec<LmthNode>>,
}
