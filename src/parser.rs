use rustlr::Tokenizer;
use crate::ast::Program;
use crate::lexer::CustomLexer;
use crate::rustlrparser::{make_parser, parse_with, rustlrlexer};

pub fn parse_aguda_program(src: &str) -> Result<Program, String> {
    // custom lexer to detect lexical errors first
    let mut lexer = CustomLexer::new(&src);
    while let Some(_) = lexer.nextsym() { /* consume tokens */ }
    if let Some(lex_err) = &lexer.lex_error {
        return Err(lex_err.to_owned());
    }

    // move on with parsing with auto lexer
    let inner_lexer = rustlrlexer::from_str(&src);
    let mut parser = make_parser(inner_lexer);
    parser.set_err_report(true);
    let result = parse_with(&mut parser);
    match result {
        Ok(raw_ast) => Ok(Program::convert(raw_ast)),
        Err(_) => Err(format!("{}", parser.get_err_report())),
    }
}