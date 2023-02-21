use std::{
    io::{Error, ErrorKind, Result},
    iter::Peekable,
    ops::Range,
    str::Chars,
};

pub type Span = Range<usize>;

#[derive(Debug, Clone)]
pub enum Token {
    /// `var`
    Var { span: Span },
    /// `static`
    Static { span: Span },
    /// `const`
    Const { span: Span },
    /// `fun`
    Fun { span: Span },
    /// `if`
    If { span: Span },
    /// `while`
    While { span: Span },
    /// `extern`
    Extern { span: Span },
    /// `inline`
    Inline { span: Span },
    /// `link`
    Use { span: Span },
    /// `return`
    Return { span: Span },
    /// `(`
    LeftParen { span: Span },
    /// `)`
    RightParen { span: Span },
    /// `[`
    LeftBracket { span: Span },
    /// `]`
    RightBracket { span: Span },
    /// `{`
    LeftBrace { span: Span },
    /// `}`
    RightBrace { span: Span },
    /// `;`
    Semicolon { span: Span },
    /// `,`
    Comma { span: Span },
    /// `=`
    Equal { span: Span },
    /// `+`
    Plus { span: Span },
    /// `-`
    Minus { span: Span },
    /// `*`
    Asterisk { span: Span },
    /// `/`
    Slash { span: Span },
    /// `->`
    Arrow { span: Span },
    /// `==`
    Equals { span: Span },
    /// `!`
    Not { span: Span },
    /// `!=`
    NotEqual { span: Span },
    /// `<`
    Less { span: Span },
    /// `>`
    Greater { span: Span },
    /// `<=`
    LessEqual { span: Span },
    /// `>=`
    GreaterEqual { span: Span },
    /// `|`
    Or { span: Span },
    /// `&`
    And { span: Span },
    /// `^`
    Xor { span: Span },
    /// abc
    Ident { span: Span },
    /// 123
    Int { span: Span, value: i64 },
    /// 123u
    UInt { span: Span, value: u64 },
    /// 1.2
    Float { span: Span, value: f64 },
    /// "Hello"
    String { span: Span },
}

impl Token {
    pub fn span(&self) -> Span {
        match self {
            Token::Var { span } => span,
            Token::Static { span } => span,
            Token::Const { span } => span,
            Token::Fun { span } => span,
            Token::If { span } => span,
            Token::While { span } => span,
            Token::Extern { span } => span,
            Token::Inline { span } => span,
            Token::Use { span } => span,
            Token::Return { span } => span,
            Token::LeftParen { span } => span,
            Token::RightParen { span } => span,
            Token::LeftBracket { span } => span,
            Token::RightBracket { span } => span,
            Token::LeftBrace { span } => span,
            Token::RightBrace { span } => span,
            Token::Semicolon { span } => span,
            Token::Comma { span } => span,
            Token::Equal { span } => span,
            Token::Plus { span } => span,
            Token::Minus { span } => span,
            Token::Asterisk { span } => span,
            Token::Slash { span } => span,
            Token::Arrow { span } => span,
            Token::Equals { span } => span,
            Token::Not { span } => span,
            Token::NotEqual { span } => span,
            Token::Less { span } => span,
            Token::Greater { span } => span,
            Token::LessEqual { span } => span,
            Token::GreaterEqual { span } => span,
            Token::Or { span } => span,
            Token::And { span } => span,
            Token::Xor { span } => span,
            Token::Ident { span } => span,
            Token::Int { span, .. } => span,
            Token::UInt { span, .. } => span,
            Token::Float { span, .. } => span,
            Token::String { span } => span,
        }
        .clone()
    }

    pub fn precedence(&self) -> usize {
        match self {
            Token::Plus { .. } => 2,
            Token::Minus { .. } => 2,
            Token::Asterisk { .. } => 3,
            Token::Slash { .. } => 3,
            Token::Or { .. } => 3,
            Token::And { .. } => 3,
            Token::Xor { .. } => 3,
            _ => unreachable!(),
        }
    }
}

pub struct Source<'a> {
    pub index: usize,
    inner: &'a str,
    source: Peekable<Chars<'a>>,
}

impl<'a> Source<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            index: 0,
            inner: source,
            source: source.chars().peekable(),
        }
    }

    pub fn has_next(&mut self) -> bool {
        self.source.peek().is_some()
    }

    pub fn advance(&mut self) {
        let c = self.source.next().unwrap();
        self.index += c.len_utf8();
    }

    pub fn peek(&mut self) -> char {
        *self.source.peek().unwrap()
    }

    pub fn sub(&self, range: Span) -> &str {
        &self.inner[range]
    }
}

pub fn tokenize(source: &str) -> Result<Vec<Token>> {
    let mut source = Source::new(source);
    let mut result = Vec::new();
    let mut index = source.index;
    let mut len = 0;
    while source.has_next() {
        let c = source.peek();
        match c {
            '#' => {
                while source.has_next() {
                    source.advance();
                    if source.peek() == '\n' {
                        break;
                    }
                }
            }
            '(' => {
                if len > 0 {
                    result.push(parse_token(index..index + len, &source)?);
                    len = 0;
                }
                let index = source.index;
                source.advance();
                result.push(Token::LeftParen {
                    span: index..source.index,
                });
            }
            ')' => {
                if len > 0 {
                    result.push(parse_token(index..index + len, &source)?);
                    len = 0;
                }
                let index = source.index;
                source.advance();
                result.push(Token::RightParen {
                    span: index..source.index,
                });
            }
            '[' => {
                if len > 0 {
                    result.push(parse_token(index..index + len, &source)?);
                    len = 0;
                }
                let index = source.index;
                source.advance();
                result.push(Token::LeftBracket {
                    span: index..source.index,
                });
            }
            ']' => {
                if len > 0 {
                    result.push(parse_token(index..index + len, &source)?);
                    len = 0;
                }
                let index = source.index;
                source.advance();
                result.push(Token::RightBracket {
                    span: index..source.index,
                });
            }
            '{' => {
                if len > 0 {
                    result.push(parse_token(index..index + len, &source)?);
                    len = 0;
                }
                let index = source.index;
                source.advance();
                result.push(Token::LeftBrace {
                    span: index..source.index,
                });
            }
            '}' => {
                if len > 0 {
                    result.push(parse_token(index..index + len, &source)?);
                    len = 0;
                }
                let index = source.index;
                source.advance();
                result.push(Token::RightBrace {
                    span: index..source.index,
                });
            }
            ';' => {
                if len > 0 {
                    result.push(parse_token(index..index + len, &source)?);
                    len = 0;
                }
                let index = source.index;
                source.advance();
                result.push(Token::Semicolon {
                    span: index..source.index,
                });
            }
            ',' => {
                if len > 0 {
                    result.push(parse_token(index..index + len, &source)?);
                    len = 0;
                }
                let index = source.index;
                source.advance();
                result.push(Token::Comma {
                    span: index..source.index,
                });
            }
            '=' => {
                if len > 0 {
                    result.push(parse_token(index..index + len, &source)?);
                    len = 0;
                }
                let index = source.index;
                source.advance();
                if source.has_next() && source.peek() == '=' {
                    source.advance();
                    result.push(Token::Equals {
                        span: index..source.index,
                    });
                    continue;
                }
                result.push(Token::Equal {
                    span: index..source.index,
                });
            }
            '+' => {
                if len > 0 {
                    result.push(parse_token(index..index + len, &source)?);
                    len = 0;
                }
                let index = source.index;
                source.advance();
                result.push(Token::Plus {
                    span: index..source.index,
                });
            }
            '-' => {
                if len > 0 {
                    result.push(parse_token(index..index + len, &source)?);
                    len = 0;
                }
                let index = source.index;
                source.advance();
                if source.has_next() && source.peek() == '>' {
                    source.advance();
                    result.push(Token::Arrow {
                        span: index..source.index,
                    });
                    continue;
                }
                result.push(Token::Minus {
                    span: index..source.index,
                });
            }
            '*' => {
                if len > 0 {
                    result.push(parse_token(index..index + len, &source)?);
                    len = 0;
                }
                let index = source.index;
                source.advance();
                result.push(Token::Asterisk {
                    span: index..source.index,
                });
            }
            '/' => {
                if len > 0 {
                    result.push(parse_token(index..index + len, &source)?);
                    len = 0;
                }
                let index = source.index;
                source.advance();
                result.push(Token::Slash {
                    span: index..source.index,
                });
            }
            '!' => {
                if len > 0 {
                    result.push(parse_token(index..index + len, &source)?);
                    len = 0;
                }
                let index = source.index;
                source.advance();
                if source.has_next() && source.peek() == '=' {
                    source.advance();
                    result.push(Token::NotEqual {
                        span: index..source.index,
                    });
                    continue;
                }
                result.push(Token::Not {
                    span: index..source.index,
                });
            }
            '<' => {
                if len > 0 {
                    result.push(parse_token(index..index + len, &source)?);
                    len = 0;
                }
                let index = source.index;
                source.advance();
                if source.has_next() && source.peek() == '=' {
                    source.advance();
                    result.push(Token::LessEqual {
                        span: index..source.index,
                    });
                    continue;
                }
                result.push(Token::Less {
                    span: index..source.index,
                });
            }
            '>' => {
                if len > 0 {
                    result.push(parse_token(index..index + len, &source)?);
                    len = 0;
                }
                let index = source.index;
                source.advance();
                if source.has_next() && source.peek() == '=' {
                    source.advance();
                    result.push(Token::GreaterEqual {
                        span: index..source.index,
                    });
                    continue;
                }
                result.push(Token::Greater {
                    span: index..source.index,
                });
            }
            '|' => {
                if len > 0 {
                    result.push(parse_token(index..index + len, &source)?);
                    len = 0;
                }
                let index = source.index;
                source.advance();
                result.push(Token::Or {
                    span: index..source.index,
                });
            }
            '&' => {
                if len > 0 {
                    result.push(parse_token(index..index + len, &source)?);
                    len = 0;
                }
                let index = source.index;
                source.advance();
                result.push(Token::And {
                    span: index..source.index,
                });
            }
            '^' => {
                if len > 0 {
                    result.push(parse_token(index..index + len, &source)?);
                    len = 0;
                }
                let index = source.index;
                source.advance();
                result.push(Token::Xor {
                    span: index..source.index,
                });
            }
            '"' => {
                if len > 0 {
                    result.push(parse_token(index..index + len, &source)?);
                    len = 0;
                }
                let index = source.index;
                source.advance();
                let mut len = 0;
                while source.has_next() {
                    match source.peek() {
                        '"' => {
                            len += 1;
                            break;
                        }
                        '\\' => {
                            len += 1;
                            source.advance();
                            if !source.has_next() {
                                return Err(Error::new(
                                    ErrorKind::Other,
                                    "Unexpected end of source in string",
                                ));
                            }
                            let ac = source.peek();
                            match ac {
                                '\\' | 'n' | 't' | 'r' | '"' => {
                                    len += 1;
                                    source.advance();
                                }
                                _ => {
                                    return Err(Error::new(
                                        ErrorKind::Other,
                                        "Invalid escape code in string",
                                    ));
                                }
                            }
                        }
                        c => {
                            len += c.len_utf8();
                            source.advance();
                        }
                    }
                }
                if !source.has_next() {
                    return Err(Error::new(
                        ErrorKind::Other,
                        "Unexpected end of source in string",
                    ));
                }
                source.advance();
                result.push(Token::String {
                    span: index..index + len,
                });
            }
            _ => {
                if c.is_whitespace() {
                    if len > 0 {
                        result.push(parse_token(index..index + len, &source)?);
                        len = 0;
                    }
                    source.advance();
                    continue;
                }
                if len < 1 {
                    index = source.index;
                }
                len += c.len_utf8();
                source.advance();
            }
        }
    }
    if len > 0 {
        result.push(parse_token(index..index + len, &source)?);
    }
    Ok(result)
}

fn parse_token(span: Span, source: &Source) -> Result<Token> {
    let token = source.sub(span.clone());
    if token.ends_with('u') {
        if let Ok(value) = token[0..token.len() - 1].parse() {
            return Ok(Token::UInt { span, value });
        }
    }
    if let Ok(value) = token.parse() {
        return Ok(Token::Int { span, value });
    }
    if let Ok(value) = token.parse() {
        return Ok(Token::Float { span, value });
    }
    if token.chars().next().unwrap().is_ascii_digit() {
        return Err(Error::new(ErrorKind::Other, "Token starting with digit"));
    }
    Ok(match token {
        "var" => Token::Var { span },
        "static" => Token::Static { span },
        "const" => Token::Const { span },
        "fun" => Token::Fun { span },
        "if" => Token::If { span },
        "while" => Token::While { span },
        "extern" => Token::Extern { span },
        "inline" => Token::Inline { span },
        "use" => Token::Use { span },
        "return" => Token::Return { span },
        _ => Token::Ident { span },
    })
}
