use rustlr::{TerminalToken, Tokenizer};
use crate::rustlrparser::{rustlrlexer, RetTypeEnum};

const VALID_TOKENS: [&str; 44] = [
    ";", ",", "->", "+", "-", "*", "/", "^", "==", "!=", "<", "<=", ">", ">=", "&&", "!",
    "[", "]", "(", ")", "COLON", "MOD", "PIPE", "ASSIGN", "OR", "WHILE", "LET", "SET",
    "IF", "THEN", "ELSE", "DO", "NEW", "TRUE", "FALSE", "NULL", "UNIT", "TINT", "TBOOL",
    "TSTRING", "TUNIT", "Id", "Num", "Str"
];

/*
 * RUSTLR already comes with a lexer, but we need to extend it in order to
 * distinguish between lexical and syntactic errors
 */
pub struct CustomLexer<'lt> {
    pub inner: rustlrlexer<'lt>,
    pub lex_error: Option<String>,
    input: &'lt str,
}

impl<'lt> CustomLexer<'lt> {
    pub fn new(input: &'lt str) -> Self {
        CustomLexer {
            inner: rustlrlexer::from_str(input),
            lex_error: None,
            input,
        }
    }
}

impl<'lt> Tokenizer<'lt, RetTypeEnum<'lt>> for CustomLexer<'lt> {

    fn nextsym(&mut self) -> Option<TerminalToken<'lt, RetTypeEnum<'lt>>> {
        let sym = self.inner.nextsym();

        if let Some(tok) = &sym {
            if !VALID_TOKENS.contains(&tok.sym) {
                let error_location = get_error_location(self.input, tok.line, tok.column);
                self.lex_error = Some(format!(
                    "LEXICAL ERROR: invalid token '{}' on line {}, column {} ..\n{}",
                    tok.sym, tok.line, tok.column, error_location
                ));
                return None;
            }
        }
        sym
    }

    // rest of the methods are just from the inner lexer
    fn linenum(&self) -> usize {
        self.inner.linenum()
    }
    fn column(&self) -> usize {
        self.inner.column()
    }
    fn position(&self) -> usize {
        self.inner.position()
    }
    fn add_priority_symbol(&mut self, s: &'static str) {
        self.inner.add_priority_symbol(s)
    }
    fn current_line(&self) -> &str {
        self.inner.current_line()
    }
    fn get_line(&self, i: usize) -> Option<&str> {
        self.inner.get_line(i)
    }
    fn get_slice(&self, s: usize, l: usize) -> &str {
        self.inner.get_slice(s, l)
    }
    fn transform_wildcard(
        &self,
        t: TerminalToken<'lt, RetTypeEnum<'lt>>,
    ) -> TerminalToken<'lt, RetTypeEnum<'lt>> {
        self.inner.transform_wildcard(t)
    }
}

fn get_error_location(
    input: &str,
    line: usize,
    column: usize,
) -> String {
    let lines: Vec<&str> = input.lines().collect();
    if line > 0 && line <= lines.len() {
        let line_text = lines[line - 1];
        let marker = " ".repeat(column.saturating_sub(1)) + "^";
        format!(" >> {}\n    {}", line_text, marker)
    } else {
        "".to_string()
    }
}