// TODO

use syn::{
    braced,
    ext::IdentExt,
    parenthesized,
    parse::{Parse, ParseStream},
    token::{self, Brace},
    Block, Expr, Ident, LitStr, Token, Type,
};

use super::ir::{
    Elem, ElemAttr, ElemAttrBind, ElemAttrCopy, ElemAttrVal, ElemTag, ElemTagType, LmthNode,
    LmthNodeType,
};

//~ impl for LmthNode

impl LmthNode {
    fn peek_type(input: ParseStream) -> Option<LmthNodeType> {
        let input = input.fork();

        if input.peek(Ident::peek_any) || input.peek(Token![!]) {
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
            None => Err(input.error("Invalid syntax encountered! You may want to open an issue to help us provide better error messages.")), // TODO
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
            None => Err(input.error("Invalid syntax encountered! You may want to open an issue to help us provide better error messages.")), // TODO
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
                Err(input.error("expected `:` or `=` for attribute binding, or `,` or nothing for shorthand attribute"))
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
            Ok(Self::Block(Block::parse(input)?))
        } else {
            Ok(Self::Expr(Expr::parse(input)?))
        }
    }
}
