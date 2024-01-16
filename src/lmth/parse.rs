// TODO

use syn::{
    braced,
    ext::IdentExt,
    parenthesized,
    parse::{Parse, ParseStream},
    token::{self, Brace},
    Block, Expr, Ident, LitStr, Token, Type,
};

use super::ir::{Elem, ElemAttr, ElemAttrVal, ElemTag, ElemTagType, LmthNode, LmthNodeType};

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
            eprintln!("{}", input); // TODO
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
            None => Err(input.error("[TODO; LmthNode::parse()] Invalid syntax!")),
        }
    }
}

//~ impl for Elem

impl Parse for Elem {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let tag = input.parse()?;
        eprintln!("tag: {:#?}", tag); // TODO

        let attrs = if input.peek(token::Paren) {
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
        eprintln!("attrs: {:#?}", attrs); // TODO

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
        eprintln!("content: {:#?}", content); // TODO

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
            eprintln!("peeked !"); // TODO
            Some(ElemTagType::Fragment)
        } else if input.peek(Ident::peek_any) {
            eprintln!("peeked ident"); // TODO
            if input.peek2(Token![<]) {
                eprintln!("peeked(2) <"); // TODO
                Some(ElemTagType::Custom)
            } else {
                eprintln!("peeked(2) not <"); // TODO
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
            Some(ElemTagType::Fragment) => Ok(Self::Fragment),
            None => Err(input.error("[TODO; ElemTag::parse()] Invalid syntax!")),
        }
    }
}

//~ impl for ElemAttr

impl Parse for ElemAttr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let key = input.call(Ident::parse_any)?;
        input.parse::<Token![:]>()?;
        let val = input.parse()?;
        Ok(Self { key, val })
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
