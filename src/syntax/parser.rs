lalrpop_util::lalrpop_mod!(pub grammar);

use crate::syntax::lexer::Token;
use crate::syntax::parser::grammar::ProgramParser;
use crate::syntax::ast::{Program, Spanned};
use lalrpop_util::ParseError;
use crate::errors::SyntaxError;

pub struct Parser {
    parser: ProgramParser,
    tokens: Vec<Spanned<Token>>
}

impl Parser {
    pub fn new(tokens: Vec<Spanned<Token>>) -> Parser {
        Parser {
            parser: ProgramParser::new(),
            tokens
        }
    }

    pub fn parse(&self) -> Result<Program, SyntaxError> {
        let tokens = self.tokens.iter().map(|t| (t.span.start, t.value.clone(), t.span.end)).collect::<Vec<_>>();
        self.parser
            .parse(tokens)
            .map_err(|e| match e {
                ParseError::UnrecognizedToken { token: (start, found, end), expected } =>
                    SyntaxError::unexpected_token(start..end, expected, found),
                ParseError::UnrecognizedEof { location, expected } =>
                    SyntaxError::unexpected_eof(location..location, expected),

                ParseError::InvalidToken { location } =>
                    SyntaxError::invalid_token(location..location),

                ParseError::ExtraToken { token: (start, _, end) } =>
                    SyntaxError::extra_token(start..end),

                _ => panic!("unexpected error: {:?}", e),
            })
    }
}
