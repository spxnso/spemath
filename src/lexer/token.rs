use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub struct Span {
    pub line: usize,
    pub col: usize,
    pub pos: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Spanned<T> {
    pub value: T,
    pub span: Span,
}

pub type SpannedToken = Spanned<Token>;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Number(f64),
    Identifier(String),
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Caret,
    LParen,
    RParen,
    LBrace,
    RBrace,
    LBracket,
    RBracket,
    Comma,
    Equal,
    EqualEqual,
    Less,
    Greater,
    LessEqual,
    GreaterEqual,
    Exclamation,
    ExclamationEqual,
    Semicolon,
    Newline,
    Whitespace,
    Eof,
}

impl Token {
    pub fn description(&self) -> String {
        match self {
            Token::Number(_) => "number".to_string(),
            Token::Identifier(_) => "identifier".to_string(),
            Token::Eof => "end of input".to_string(),
            Token::Newline => "newline".to_string(),
            Token::Whitespace => "whitespace".to_string(),
            _ => format!("{}", self),
        }
    }

    pub fn span(self, line: usize, col: usize, pos: usize) -> SpannedToken {
        Spanned {
            value: self,
            span: Span { line, col, pos },
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Number(n) => write!(f, "{}", n),
            Token::Identifier(s) => write!(f, "{}", s),
            Token::Plus => write!(f, "+"),
            Token::Minus => write!(f, "-"),
            Token::Star => write!(f, "*"),
            Token::Slash => write!(f, "/"),
            Token::Percent => write!(f, "%"),
            Token::Caret => write!(f, "^"),
            Token::LParen => write!(f, "("),
            Token::RParen => write!(f, ")"),
            Token::LBrace => write!(f, "{{"),
            Token::RBrace => write!(f, "}}"),
            Token::LBracket => write!(f, "["),
            Token::RBracket => write!(f, "]"),
            Token::Comma => write!(f, ","),
            Token::Equal => write!(f, "="),
            Token::EqualEqual => write!(f, "=="),
            Token::Less => write!(f, "<"),
            Token::Greater => write!(f, ">"),
            Token::LessEqual => write!(f, "<="),
            Token::GreaterEqual => write!(f, ">="),
            Token::Exclamation => write!(f, "!"),
            Token::ExclamationEqual => write!(f, "!="),
            Token::Semicolon => write!(f, ";"),
            Token::Newline => write!(f, "\\n"),
            Token::Whitespace => write!(f, " "),
            Token::Eof => write!(f, "end of file"),
        }
    }
}
