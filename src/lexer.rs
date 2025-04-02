use logos::Logos;

#[derive(Logos, Debug, PartialEq, Clone)]
#[logos(skip r"[ \t\r\n\f]+")] // ignore whitespace
#[logos(skip r"--(.*)")] // ignore comments
pub enum Token {

    #[regex("[a-zA-Z_][a-zA-Z0-9'_]*", |lex| lex.slice().to_string(), priority = 0)]
    Id(String),

    #[regex(r"-?(0|[1-9][0-9]*)", |lex| lex.slice().parse().unwrap_or(0))]
    Int(i64),

    #[regex("\"[^\"\n]*\"", |lex| lex.slice().to_string())]
    String(String),

    #[token("true")]
    True,

    #[token("false")]
    False,

    #[token("null")]
    Null,

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
    src: &'a str,
}

impl<'a> Lexer<'a> {
    pub fn new(src: &'a str) -> Self {
        Self {
            lexer: Token::lexer(src),
            src
        }
    }

    pub fn tokenize(&mut self) -> Result<Vec<(usize, Token, usize)>, String> {
        let mut tokens = Vec::new();
        while let Some(tok) = self.lexer.next() {
            let span = self.lexer.span();
            match tok {
                Ok(token) => tokens.push((span.start, token, span.end)),
                Err(e) => return Err(format!("Error at {}: {:?}", span.start, e)),
            }
        }
        Ok(tokens)
    }
}
