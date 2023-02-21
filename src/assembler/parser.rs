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

pub enum Expr {
    Int {
        span: Span,
        value: i64,
    },
    UInt {
        span: Span,
        value: u64,
    },
    Float {
        span: Span,
        value: f64,
    },
    String {
        span: Span,
    },
    Add {
        span: Span,
    },
    Sub {
        span: Span,
    },
    Mul {
        span: Span,
    },
    Div {
        span: Span,
    },
    Var {
        span: Span,
    },
    Call {
        span: Span,
        name_span: Span,
        args: Vec<Expr>,
    },
}

pub struct TmpCall {
    pub start: usize,
    pub name_span: Span,
    pub args: Vec<Expr>,
    pub expect_comma: bool,
}

fn _parse_expr(state: &mut State) -> Result<()> {
    let mut output = Vec::with_capacity(0);
    let mut tmp_stacks = Vec::with_capacity(0);
    while state.stream.has_token() {
        let token = state.stream.peek();
        match token {
            Token::Ident { span } => {
                state.stream.advance();
                if !state.stream.has_token() {
                    return Err(Error::new(
                        ErrorKind::Other,
                        "Invalid end of source in expression",
                    ));
                }
                let next = state.stream.peek();
                if let Token::LeftParen { .. } = next {
                    state.stream.advance();
                    tmp_stacks.push(TmpCall {
                        start: span.start,
                        name_span: span,
                        args: Vec::with_capacity(0),
                        expect_comma: false,
                    });
                    continue;
                }
                if !tmp_stacks.is_empty() {
                    let last = tmp_stacks.last_mut().unwrap();
                    if last.expect_comma {
                        return Err(Error::new(
                            ErrorKind::Other,
                            "Invalid token in expression, expected comma between call arguments",
                        ));
                    }
                    last.expect_comma = true;
                    last.args.push(Expr::Var { span });
                    continue;
                }
                output.push(Expr::Var { span });
            }
            Token::Int { span, value } => {
                state.stream.advance();
                if !tmp_stacks.is_empty() {
                    let last = tmp_stacks.last_mut().unwrap();
                    if last.expect_comma {
                        return Err(Error::new(
                            ErrorKind::Other,
                            "Invalid token in expression, expected comma between call arguments",
                        ));
                    }
                    last.expect_comma = true;
                    last.args.push(Expr::Int { span, value });
                    continue;
                }
                output.push(Expr::Int { span, value });
            }
            Token::UInt { span, value } => {
                state.stream.advance();
                if !tmp_stacks.is_empty() {
                    let last = tmp_stacks.last_mut().unwrap();
                    if last.expect_comma {
                        return Err(Error::new(
                            ErrorKind::Other,
                            "Invalid token in expression, expected comma between call arguments",
                        ));
                    }
                    last.expect_comma = true;
                    last.args.push(Expr::UInt { span, value });
                    continue;
                }
                output.push(Expr::UInt { span, value });
            }
            Token::Float { span, value } => {
                state.stream.advance();
                if !tmp_stacks.is_empty() {
                    let last = tmp_stacks.last_mut().unwrap();
                    if last.expect_comma {
                        return Err(Error::new(
                            ErrorKind::Other,
                            "Invalid token in expression, expected comma between call arguments",
                        ));
                    }
                    last.expect_comma = true;
                    last.args.push(Expr::Float { span, value });
                    continue;
                }
                output.push(Expr::Float { span, value });
            }
            Token::String { span } => {
                state.stream.advance();
                if !tmp_stacks.is_empty() {
                    let last = tmp_stacks.last_mut().unwrap();
                    if last.expect_comma {
                        return Err(Error::new(
                            ErrorKind::Other,
                            "Invalid token in expression, expected comma between call arguments",
                        ));
                    }
                    last.expect_comma = true;
                    last.args.push(Expr::String { span });
                    continue;
                }
                output.push(Expr::String { span });
            }
            Token::Comma { .. } => {
                state.stream.advance();
                if tmp_stacks.is_empty() {
                    return Err(Error::new(
                        ErrorKind::Other,
                        "Invalid token in expression, comma is only allowed between call arguments",
                    ));
                }
                let last = tmp_stacks.last_mut().unwrap();
                if !last.expect_comma {
                    return Err(Error::new(
                        ErrorKind::Other,
                        "Invalid token in expression, unexpected comma",
                    ));
                }
                last.expect_comma = false;
            }
            Token::RightParen { span } => {
                state.stream.advance();
                if tmp_stacks.is_empty() {
                    return Err(Error::new(
                        ErrorKind::Other,
                        "Invalid token in expression, no paranthesis to close",
                    ));
                }
                let TmpCall {
                    start,
                    name_span,
                    args,
                    ..
                } = tmp_stacks.pop().unwrap();
                output.push(Expr::Call {
                    span: start..span.end,
                    name_span,
                    args,
                });
            }
            Token::Semicolon { .. } => {
                if !tmp_stacks.is_empty() {
                    return Err(Error::new(
                        ErrorKind::Other,
                        "Invalid semicolon in expression, not every call was closed",
                    ));
                }
                state.stream.advance();
                break;
            }
            _ => {}
        }
    }
    todo!();
}

/// Shunting Yard element
pub enum YardElement {
    Int {
        span: Span,
        value: i64,
    },
    UInt {
        span: Span,
        value: u64,
    },
    Float {
        span: Span,
        value: f64,
    },
    String {
        span: Span,
    },
    Add {
        span: Span,
    },
    Sub {
        span: Span,
    },
    Mul {
        span: Span,
    },
    Div {
        span: Span,
    },
    Or {
        span: Span,
    },
    And {
        span: Span,
    },
    Xor {
        span: Span,
    },
    Var {
        span: Span,
    },
    Call {
        span: Span,
        name_span: Span,
        args: Vec<YardElement>,
    },
    Group {
        span: Span,
        output: Vec<YardElement>,
    },
}

impl YardElement {
    pub fn precedence(&self) -> usize {
        match self {
            Self::Add { .. } => 2,
            Self::Sub { .. } => 2,
            Self::Mul { .. } => 3,
            Self::Div { .. } => 3,
            Self::Or { .. } => 3,
            Self::And { .. } => 3,
            Self::Xor { .. } => 3,
            _ => unreachable!(),
        }
    }
}

pub enum YardType {
    Root,
    Group,
    Call { name: Span, args: Vec<YardElement> },
}

/// Shunting Yard state
pub struct YardState {
    pub type_: YardType,
    pub op_stack: Vec<YardElement>,
    pub output: Vec<YardElement>,
}

impl Default for YardState {
    fn default() -> Self {
        Self {
            type_: YardType::Root,
            op_stack: Vec::with_capacity(0),
            output: Vec::with_capacity(0),
        }
    }
}

fn _parse_expr_reimpl(state: &mut State) -> Result<()> {
    let mut yards = vec![YardState::default()];
    while state.stream.has_token() {
        let yard = yards.last_mut().unwrap();
        let token = state.stream.peek();
        state.stream.advance();
        match token.clone() {
            Token::Int { span, value } => {
                yard.output.push(YardElement::Int { span, value });
            }
            Token::UInt { span, value } => {
                yard.output.push(YardElement::UInt { span, value });
            }
            Token::Float { span, value } => {
                yard.output.push(YardElement::Float { span, value });
            }
            Token::Ident { span: name } => {
                if !state.stream.has_token() {
                    return Err(Error::new(ErrorKind::Other, "Unexpected end of source"));
                }
                let next = state.stream.peek();
                if let Token::LeftParen { .. } = next {
                    yards.push(YardState {
                        type_: YardType::Call {
                            name,
                            args: Vec::with_capacity(0),
                        },
                        op_stack: Vec::with_capacity(0),
                        output: Vec::with_capacity(0),
                    });
                    continue;
                }
                yard.output.push(YardElement::Var { span: name });
            }
            Token::Plus { .. }
            | Token::Minus { .. }
            | Token::Asterisk { .. }
            | Token::Slash { .. }
            | Token::Or { .. }
            | Token::And { .. }
            | Token::Xor { .. } => {
                while !yard.op_stack.is_empty() {
                    let top = yard.op_stack.last().unwrap();
                    if top.precedence() < token.precedence() {
                        break;
                    }
                    yard.output.push(yard.op_stack.pop().unwrap());
                }
                yard.op_stack.push(match token {
                    Token::Plus { span } => YardElement::Add { span },
                    Token::Minus { span } => YardElement::Sub { span },
                    Token::Asterisk { span } => YardElement::Mul { span },
                    Token::Slash { span } => YardElement::Div { span },
                    Token::Or { span } => YardElement::Or { span },
                    Token::And { span } => YardElement::And { span },
                    Token::Xor { span } => YardElement::Xor { span },
                    _ => unreachable!(),
                })
            }
            Token::RightParen { .. } => {
                if yards.len() < 2 {
                    return Err(Error::new(ErrorKind::Other, "Illegal closing parenthesis"));
                }
                let a = yards.pop().unwrap();
                let last = yards.last_mut().unwrap();
                match a.type_ {
                    YardType::Root => unreachable!(),
                    YardType::Group => last.output.push(YardElement::Group {
                        span: 0..0,
                        output: a.output,
                    }),
                    YardType::Call { .. } => todo!(),
                }
            }
            _ => {}
        }
    }
    todo!();
}
