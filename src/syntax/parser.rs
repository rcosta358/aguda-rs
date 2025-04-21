lalrpop_util::lalrpop_mod!(pub grammar);

use crate::syntax::lexer::Token;
use crate::syntax::parser::grammar::ProgramParser;
use crate::syntax::ast::Program;
use lalrpop_util::ParseError;
use crate::utils::format_error;

pub struct Parser <'a> {
    parser: ProgramParser,
    src: &'a str,
    tokens: Vec<(usize, Token, usize)>
}

impl <'a> Parser<'a> {
    pub fn new(src: &str, tokens: Vec<(usize, Token, usize)>) -> Parser {
        Parser {
            parser: ProgramParser::new(),
            src,
            tokens
        }
    }

    pub fn parse(&self) -> Result<Program, String> {
        self.parser
            .parse(self.tokens.clone())
            .map_err(|e| match e {
                ParseError::UnrecognizedToken { token: (start, _, end), expected } => {
                    format_error(&self.src, start..end, "unexpected token", Some(&expected))
                }
                ParseError::UnrecognizedEof { location, expected } => {
                    format_error(&self.src, location..location, "unexpected end of input", Some(&expected))
                }
                ParseError::InvalidToken { location } => {
                    format_error(&self.src, location..location, "invalid token", None)
                }
                ParseError::ExtraToken { token: (start, _, end) } => {
                    format_error(&self.src, start..end, "unexpected extra token", None)
                }
                other => format!("unexpected error: {:?}", other),
            })
    }
}
