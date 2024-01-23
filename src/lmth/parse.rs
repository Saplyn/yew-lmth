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
            eprintln!("{}", input); // TODO
            None
        }
    }
}

impl Parse for LmthNode {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        eprintln!("[LmthNode::parse()] input: {:#?}", input); // TODO
        let res = match Self::peek_type(input) {
            Some(LmthNodeType::Elem) => Ok(Self::Elem(input.parse()?)),
            Some(LmthNodeType::Block) => Ok(Self::Block(input.parse()?)),
            Some(LmthNodeType::LitStr) => Ok(Self::LitStr(input.parse()?)),
            None => Err(input.error("[TODO; LmthNode::parse()] Invalid syntax!")),
        };
        eprintln!("[LmthNode::parse()] parsed result: {:#?}\n", res); // TODO
        res
    }
}

//~ impl for Elem

impl Parse for Elem {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let tag = input.parse()?;
        eprintln!("[Elem::parse()] tag: {:#?}\n", tag); // TODO

        let attrs = if input.peek(token::Paren) {
            eprintln!("[Elem::parse()] peeked paren"); // TODO
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
        eprintln!("[Elem::parse()] attrs: {:#?}\n", attrs); // TODO

        let content = if input.peek(token::Brace) {
            eprintln!("[Elem::parse()] peeked brace"); // TODO
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
        eprintln!("[Elem::parse()] content: {:#?}\n", content); // TODO

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
            eprintln!("[ElemTag::peek_type()] peeked !"); // TODO
            Some(ElemTagType::Fragment)
        } else if input.peek(Ident::peek_any) {
            eprintln!("[ElemTag::peek_type()] peeked ident"); // TODO
            if input.peek2(Token![<]) {
                eprintln!("[ElemTag::peek_type()] peeked(2) <"); // TODO
                Some(ElemTagType::Custom)
            } else {
                eprintln!("[ElemTag::peek_type()] peeked(2) not <"); // TODO
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
            None => Err(input.error("[TODO; ElemTag::parse()] Invalid syntax!")), // TODO
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
                Err(input.error("[TODO; ElemAttr::parse()] Invalid syntax!")) // TODO
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
