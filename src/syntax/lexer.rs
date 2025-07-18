
use std::num::ParseIntError;
use logos::Logos;
use crate::diagnostics::errors::{LexicalError, LexicalErrorKind};
use crate::syntax::ast::Spanned;

#[derive(Logos, Debug, PartialEq, Clone)]
#[logos(error=LexicalErrorKind)]
#[logos(skip r"[ \t\r\n\f]+")] // ignore whitespaces
#[logos(skip r"--(.*)")] // ignore comments
pub enum Token {

    #[regex("[a-zA-Z_][a-zA-Z0-9'_]*", |lex| lex.slice().to_string(), priority = 0)]
    Id(String),

    #[regex("[0-9]+", |lex| lex.slice().parse().map_err(|e| LexicalErrorKind::from(e)))]
    #[regex("[0-9]+\\.[0-9]+", |_| Err(LexicalErrorKind::FloatingPointNumber), priority = 0)]
    Int(i64),

    #[regex(r#""([^"\\]|\\.)*""#, |lex| parse_string(lex.slice()), priority = 2)]
    #[regex(r#""([^"\\]|\\.)*"#, |_| Err(LexicalErrorKind::UnterminatedString), priority = 1)]
    String(String),

    #[token("true")]
    True,

    #[token("false")]
    False,

    #[token("unit")]
    Unit,

    #[token(";")]
    Semicolon,

    #[token("+")]
    Plus,

    #[token("-")]
    Minus,

    #[token("*")]
    Multiply,

    #[token("/")]
    Divide,

    #[token("%")]
    Modulo,

    #[token("^")]
    Power,

    #[token("==")]
    Equal,

    #[token("!=")]
    NotEqual,

    #[token("<")]   
    Less,

    #[token("<=")]
    LessOrEqual,

    #[token(">")]
    Greater,

    #[token(">=")]
    GreaterOrEqual,

    #[token("!")]
    Not,

    #[token("||")]
    Or,

    #[token("&&")]
    And,

    #[token("(")]
    LeftParen,

    #[token(")")]
    RightParen,

    #[token("[")]
    LeftBracket,

    #[token("]")]
    RightBracket,

    #[token(",")]
    Comma,

    #[token("set")]
    Set,

    #[token("let")]
    Let,

    #[token(":")]
    Colon,

    #[token("=")]
    Assign,

    #[token("->")]
    Arrow,

    #[token("if")]
    If,

    #[token("then")]
    Then,

    #[token("else")]
    Else,

    #[token("while")]
    While,

    #[token("do")]
    Do,

    #[token("new")]
    New,

    #[token("|")]
    Pipe,

    #[token("Int")]
    IntType,

    #[token("Bool")]
    BoolType,

    #[token("String")]
    StringType,

    #[token("Unit")]
    UnitType,
}

pub struct Lexer<'a> {
    lexer: logos::Lexer<'a, Token>,
}

impl<'a> Lexer<'a> {
    pub fn new(src: &'a str) -> Self {
        Self {
            lexer: Token::lexer(src)
        }
    }

    pub fn tokenize(&mut self) -> Result<Vec<Spanned<Token>>, LexicalError> {
        let mut tokens = Vec::new();
        while let Some(tok) = self.lexer.next() {
            let span = self.lexer.span();
            match tok {
                Ok(token) => tokens.push(Spanned { value: token, span }),
                Err(e) => {
                    let error = LexicalError { kind: e, span };
                    return Err(error);
                }
            }
        }
        Ok(tokens)
    }
}

impl From<ParseIntError> for LexicalErrorKind {
    fn from(err: ParseIntError) -> Self {
        use std::num::IntErrorKind::*;
        match err.kind() {
            PosOverflow | NegOverflow => LexicalErrorKind::IntegerOverflow,
            _ => LexicalErrorKind::InvalidInteger,
        }
    }
}

fn parse_string(src: &str) -> Result<String, LexicalErrorKind> {
    let mut out = String::with_capacity(src.len() - 2); // without quotes
    let mut chars = src[1..src.len() - 1].chars(); // remove quotes
    while let Some(c) = chars.next() {
        // normal character
        if c != '\\' {
            out.push(c);
            continue;
        }
        // special escape sequences
        match chars.next() {
            Some('n')  => out.push('\n'),
            Some('r')  => out.push('\r'),
            Some('t')  => out.push('\t'),
            Some('0')  => out.push('\0'),
            Some('"')  => out.push('"'),
            Some('\\') => out.push('\\'),
            _ => return Err(LexicalErrorKind::InvalidEscape),
        }
    }
    Ok(out)
}