lalrpop_util::lalrpop_mod!(pub grammar);

use crate::lexer::Token;
use crate::ast::Declaration;

use lalrpop_util::ParseError;
use crate::parser::grammar::ProgramParser;
use crate::utils::format_error_with_line;

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

    pub fn parse(&self) -> Result<Vec<Declaration>, String> {
        self.parser
            .parse(self.tokens.clone())
            .map_err(|e| match e {
                ParseError::UnrecognizedToken { token: (start, _, _), expected } => {
                    format_error_with_line(&self.src, start, "unrecognized token", Some(&expected))
                }
                ParseError::UnrecognizedEof { location, expected } => {
                    format_error_with_line(&self.src, location, "unrecognized end of input", Some(&expected))
                }
                ParseError::InvalidToken { location } => {
                    format_error_with_line(&self.src, location, "invalid token", None)
                }
                ParseError::ExtraToken { token: (start, _, _) } => {
                    format_error_with_line(&self.src, start, "unexpected extra token", None)
                }
                other => format!("unexpected error: {:?}", other),
            })
    }
}
