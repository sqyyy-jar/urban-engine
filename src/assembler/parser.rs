use std::{
    io::{Error, ErrorKind, Result},
    iter::Peekable,
    slice::Iter,
};

use super::tokenizer::{Span, Token};

pub struct State<'a> {
    pub source: &'a str,
    pub stream: TokenStream<'a>,
    pub uses: Vec<Use>,
    pub consts: Vec<()>,
    pub statics: Vec<()>,
    pub funs: Vec<()>,
}

pub struct Use {
    pub span: Span,
    pub module: Span,
}

pub enum Type {
    U8 { ref_depth: usize },
    I8 { ref_depth: usize },
    U16 { ref_depth: usize },
    I16 { ref_depth: usize },
    U32 { ref_depth: usize },
    I32 { ref_depth: usize },
    U64 { ref_depth: usize },
    I64 { ref_depth: usize },
    F64 { ref_depth: usize },
}

pub struct TokenStream<'a> {
    stream: Peekable<Iter<'a, Token>>,
}

impl<'a> TokenStream<'a> {
    pub fn new(stream: &'a [Token]) -> Self {
        Self {
            stream: stream.iter().peekable(),
        }
    }

    pub fn has_token(&mut self) -> bool {
        self.stream.peek().is_some()
    }

    pub fn peek(&mut self) -> Token {
        (*self.stream.peek().unwrap()).clone()
    }

    pub fn advance(&mut self) {
        self.stream.next().unwrap();
    }

    pub fn expect(&mut self, other: &Token) -> Result<Token> {
        if !self.has_token() {
            return Err(Error::new(ErrorKind::Other, "Illegal end of source"));
        }
        let next = self.peek();
        if std::mem::discriminant(&next) != std::mem::discriminant(other) {
            return Err(Error::new(ErrorKind::Other, "Unexpected token"));
        }
        self.advance();
        Ok(next)
    }
}

pub fn parse(source: &str, tokens: &[Token]) -> Result<()> {
    let mut state = State {
        source,
        stream: TokenStream::new(tokens),
        uses: Vec::with_capacity(0),
        consts: Vec::with_capacity(0),
        statics: Vec::with_capacity(0),
        funs: Vec::with_capacity(0),
    };
    while state.stream.has_token() {
        let token = state.stream.peek();
        match token {
            Token::Use { .. } => parse_use(&mut state)?,
            Token::Const { .. } => {}
            Token::Static { .. } => {}
            Token::Extern { .. } | Token::Inline { .. } | Token::Fun { .. } => {}
            _ => {
                return Err(Error::new(ErrorKind::Other, "Invalid token at root level"));
            }
        }
    }
    todo!()
}

fn parse_use(state: &mut State) -> Result<()> {
    let start = state
        .stream
        .expect(&Token::Use { span: 0..0 })?
        .span()
        .start;
    let Token::Ident { span: module_span } = state.stream.expect(&Token::Ident { span: 0..0 })? else {
        unreachable!();
    };
    let end = state
        .stream
        .expect(&Token::Semicolon { span: 0..0 })?
        .span()
        .end;
    state.uses.push(Use {
        span: start..end,
        module: module_span,
    });
    Ok(())
}

fn _parse_const(state: &mut State) -> Result<()> {
    let _start = state
        .stream
        .expect(&Token::Const { span: 0..0 })?
        .span()
        .start;
    let _const_type = _parse_type_param(state)?;
    let Token::Ident { span: _ident_span } = state.stream.expect(&Token::Ident { span: 0..0 })? else {
        unreachable!();
    };
    todo!("Implement constant expression parsing");
}

fn _parse_type_param(state: &mut State) -> Result<Type> {
    if !state.stream.has_token() {
        return Err(Error::new(ErrorKind::Other, "Invalid type parameter"));
    }
    let Token::LeftBracket { .. } = state.stream.peek() else {
        return Err(Error::new(ErrorKind::Other, "Invalid type parameter"));
    };
    state.stream.advance();
    let inner_type = _parse_type(state)?;
    if !state.stream.has_token() {
        return Err(Error::new(ErrorKind::Other, "Invalid type parameter"));
    }
    let Token::RightBracket { .. } = state.stream.peek() else {
        return Err(Error::new(ErrorKind::Other, "Invalid type parameter"));
    };
    state.stream.advance();
    Ok(inner_type)
}

fn _parse_type(state: &mut State) -> Result<Type> {
    let mut ref_depth = 0;
    while state.stream.has_token() {
        if let Token::And { .. } = state.stream.peek() {
            state.stream.advance();
            ref_depth += 1;
        } else {
            break;
        }
    }
    if !state.stream.has_token() {
        return Err(Error::new(
            ErrorKind::Other,
            "Illegal end of source in type",
        ));
    }
    let Token::Ident { span: ident_span } = state.stream.peek() else {
        return Err(Error::new(
            ErrorKind::Other,
            "Illegal type token, must be identifier",
        ));
    };
    state.stream.advance();
    match &state.source[ident_span] {
        "u8" => Ok(Type::U8 { ref_depth }),
        "i8" => Ok(Type::I8 { ref_depth }),
        "u16" => Ok(Type::U16 { ref_depth }),
        "i16" => Ok(Type::I16 { ref_depth }),
        "u32" => Ok(Type::U32 { ref_depth }),
        "i32" => Ok(Type::I32 { ref_depth }),
        "u64" => Ok(Type::U64 { ref_depth }),
        "i64" => Ok(Type::I64 { ref_depth }),
        "f64" => Ok(Type::F64 { ref_depth }),
        _ => Err(Error::new(ErrorKind::Other, "Unknown type")),
    }
}

pub enum ShuntingYardState {
    ExpectOperand,
    ExpectOperator,
}

fn _parse_expr(ctx: &mut State) -> Result<()> {
    let mut state = ShuntingYardState::ExpectOperand;
    let mut output = Vec::new();
    while ctx.stream.has_token() {
        let token = ctx.stream.peek();
        match token {
            Token::Int { .. } | Token::UInt { .. } | Token::Float { .. } => {
                if let ShuntingYardState::ExpectOperator = state {
                    return Err(Error::new(ErrorKind::Other, "Invalid expression"));
                }
                output.push(token);
                state = ShuntingYardState::ExpectOperator;
            }
            Token::Ident { .. } => {}
            Token::Semicolon { .. } => break,
            _ => {}
        }
    }
    todo!();
}
