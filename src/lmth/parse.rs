//! Parsing utilities for `lmth` macro.

use syn::{
    braced,
    ext::IdentExt,
    parenthesized,
    parse::{Parse, ParseStream},
    token::{self, Brace},
    Expr, ExprBlock, Ident, LitStr, Pat, Token, Type,
};

use super::ir::{
    Elem, ElemAttr, ElemAttrBind, ElemAttrCopy, ElemAttrVal, ElemTag, ElemTagType, IfCond,
    IfLetCond, LmthNode, LmthNodeType,
};

//~ impl for LmthNode

impl LmthNode {
    fn peek_type(input: ParseStream) -> Option<LmthNodeType> {
        let input = input.fork();

        if input.peek(Token![if]) {
            if input.peek2(Token![let]) {
                Some(LmthNodeType::IfLetCond)
            } else {
                Some(LmthNodeType::IfCond)
            }
        } else if input.peek(Ident::peek_any) || input.peek(Token![!]) || input.peek(Token![@]) {
            Some(LmthNodeType::Elem)
        } else if input.peek(Brace) {
            Some(LmthNodeType::Block)
        } else if input.peek(LitStr) {
            Some(LmthNodeType::LitStr)
        } else {
            None
        }
    }
}

impl Parse for LmthNode {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        match Self::peek_type(input) {
            Some(LmthNodeType::Elem) => Ok(Self::Elem(input.parse()?)),
            Some(LmthNodeType::Block) => Ok(Self::Block(input.parse()?)),
            Some(LmthNodeType::LitStr) => Ok(Self::LitStr(input.parse()?)),
            None => Err(input.error(
                "Invalid syntax encountered: Expected a tag element, a code block, or a literal string.",
            )),
            Some(LmthNodeType::IfCond) => Ok(Self::IfCond(input.parse()?)),
            Some(LmthNodeType::IfLetCond) => Ok(Self::IfLetCond(input.parse()?)),
        }
    }
}

//~ impl for Elem

impl Parse for Elem {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let tag = input.parse()?;
        let attrs = if input.peek(token::Paren) {
            if tag == ElemTag::Fragment {
                return Err(input.error("Fragment cannot have attributes."));
            }
            let raw_attrs;
            parenthesized!(raw_attrs in input);
            Some(
                raw_attrs
                    .parse_terminated(ElemAttr::parse, Token![,])?
                    .into_iter()
                    .collect(),
            )
        } else {
            None
        };

        let content = if input.peek(token::Brace) {
            let mut content = Vec::new();
            let raw_content;
            braced!(raw_content in input);
            while !raw_content.is_empty() {
                content.push(raw_content.parse()?);
            }
            Some(content)
        } else {
            None
        };

        Ok(Self {
            tag,
            attrs,
            content,
        })
    }
}

//~ impl for ElemTag

impl ElemTag {
    fn peek_type(input: ParseStream) -> Option<ElemTagType> {
        let input = input.fork();

        if input.peek(Token![!]) {
            Some(ElemTagType::Fragment)
        } else if input.peek(Token![@]) {
            Some(ElemTagType::Dynamic)
        } else if input.peek(Ident::peek_any) {
            if input.peek2(Token![<]) {
                Some(ElemTagType::Custom)
            } else {
                Some(ElemTagType::Regular)
            }
        } else {
            None
        }
    }
}

impl Parse for ElemTag {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        match Self::peek_type(input) {
            Some(ElemTagType::Regular) => Ok(Self::Regular(input.call(Ident::parse_any)?)),
            Some(ElemTagType::Custom) => Ok(Self::Custom(Type::parse(input)?)),
            Some(ElemTagType::Fragment) => {
                input.parse::<Token![!]>()?;
                Ok(Self::Fragment)
            }
            Some(ElemTagType::Dynamic) => {
                input.parse::<Token![@]>()?;
                Ok(Self::Dynamic(ExprBlock::parse(input)?))
            }
            None => Err(input.error(
                "Invalid tag: Expected a valid element tag, a dynamic named tag, or a fragment.",
            )),
        }
    }
}

//~ impl for ElemAttr

impl Parse for ElemAttr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek2(Token![:]) {
            Ok(Self::Bind(ElemAttrBind::parse(input)?))
        } else if input.peek2(Token![=]) {
            Ok(Self::Copy(ElemAttrCopy::parse(input)?))
        } else if input.peek2(Token![,]) {
            Ok(Self::Sugar(Ident::parse_any(input)?))
        } else {
            let ident = Ident::parse_any(input)?;
            if !input.is_empty() {
                Err(input.error("expected `:` or `=` for attribute binding, or `,` or nothing (last attribute) for shorthand attribute"))
            } else {
                Ok(Self::Sugar(ident))
            }
        }
    }
}

//~ impl for ElemAttrBind

impl Parse for ElemAttrBind {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let key = Ident::parse_any(input)?;
        input.parse::<Token![:]>()?;
        let val = input.parse()?;
        Ok(Self { key, val })
    }
}

//~ impl for ElemAttrCopy

impl Parse for ElemAttrCopy {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let key = Ident::parse_any(input)?;
        input.parse::<Token![=]>()?;
        let litstr = input.parse()?;
        Ok(Self { key, litstr })
    }
}

//~ impl for ElemAttrVal

impl Parse for ElemAttrVal {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek(token::Brace) {
            Ok(Self::Block(ExprBlock::parse(input)?))
        } else {
            Ok(Self::Expr(Expr::parse(input)?))
        }
    }
}

//~ impl for IfCond

impl Parse for IfCond {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse::<Token![if]>()?;
        let cond = input.call(Expr::parse_without_eager_brace)?;
        let then_branch = {
            let mut then_branch = Vec::new();
            let raw_then_branch;
            braced!(raw_then_branch in input);
            while !raw_then_branch.is_empty() {
                then_branch.push(raw_then_branch.parse()?);
            }
            then_branch
        };

        let else_branch = if input.peek(Token![else]) {
            input.parse::<Token![else]>()?;
            let mut else_branch = Vec::new();
            let raw_else_branch: syn::parse::ParseBuffer<'_>;
            braced!(raw_else_branch in input);
            while !raw_else_branch.is_empty() {
                else_branch.push(raw_else_branch.parse()?);
            }
            Some(else_branch)
        } else {
            None
        };

        Ok(Self {
            cond,
            then_branch,
            else_branch,
        })
    }
}

//~ impl for IfLetCond

impl Parse for IfLetCond {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse::<Token![if]>()?;
        input.parse::<Token![let]>()?;

        let pat = Pat::parse_single(input)?;
        input.parse::<Token![=]>()?;
        let expr = Expr::parse_without_eager_brace(input)?;

        let then_branch = {
            let mut then_branch = Vec::new();
            let raw_then_branch;
            braced!(raw_then_branch in input);
            while !raw_then_branch.is_empty() {
                then_branch.push(raw_then_branch.parse()?);
            }
            then_branch
        };

        let else_branch = if input.peek(Token![else]) {
            input.parse::<Token![else]>()?;
            let mut else_branch = Vec::new();
            let raw_else_branch: syn::parse::ParseBuffer<'_>;
            braced!(raw_else_branch in input);
            while !raw_else_branch.is_empty() {
                else_branch.push(raw_else_branch.parse()?);
            }
            Some(else_branch)
        } else {
            None
        };

        Ok(Self {
            pat,
            expr,
            then_branch,
            else_branch,
        })
    }
}
