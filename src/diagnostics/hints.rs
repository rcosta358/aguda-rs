use lazy_static::lazy_static;
use crate::syntax::lexer::Token;

pub fn get_syntax_hints(expected: Vec<String>, found: Option<Token>) -> Vec<String> {
    let mut hints = Vec::new();
    let expected = expected
        .iter()
        .map(|e| e.trim_matches('"'))
        .collect::<Vec<_>>();

    if let Some(found) = found {
        let hint = match found {
            Token::Else => "Do you have an 'else' without a matching 'if'?",
            Token::Then => "Do you have an 'then' without a matching 'if'?",
            Token::Do => "Do you have a 'do' without a matching 'while'?",
            Token::RightParen => "Do you have an extra closing parenthesis?",
            Token::RightBracket => "Do you have an extra closing bracket?",
            Token::Assign if expected.contains(&"==") => "Did you mean '==' instead of '='?",
            Token::Equal if expected.contains(&"=")  => "Did you mean '=' instead of '=='?",
            Token::Pipe if expected.contains(&"||") => "Did you mean '||' instead of '|'?",
            Token::Unit if expected.contains(&"Unit") => "Did you mean 'Unit' instead of 'unit'?",
            Token::UnitType if expected.contains(&"unit") => "Did you mean 'unit' instead of 'Unit'?",
            _ => "",
        };
        if !hint.is_empty() {
            hints.push(hint.to_string())
        }
    }

    // expected contains an expression
    if EXPRESSIONS.iter().all(|e| expected.contains(&e)) {
        hints.push("Did you forget an expression?".to_string());
    }

    // expected contains a keyword
    if TYPES.iter().all(|e| expected.contains(&e)) {
        hints.push("Did you forget a type?".to_string());
    }

    // expected contains a literal
    if LITERALS.iter().all(|e| expected.contains(&e)) {
        hints.push("Did you forget a value?".to_string());
    }

    for token in expected.clone() {
        let hint = match token {
            "->" => "Did you forget the return type in the function definition?",
            ")" => "Did you forget a closing parenthesis?",
            "]" => "Did you forget a closing bracket?",
            "then" => "Did you forget a 'then' after your if condition?",
            "do" => "Did you forget a 'do' after your while condition?",
            "id" => "Did you forget an identifier?",
            "|" => "Did you forget the '|' in the array initialization?",
            ":" => "Did you forget the type annotation?",
            "," => "Did you forget a comma?",
            ";" => "Did you forget a semicolon?",
            "!eof" => "Do you have an extra semicolon at the end?",
            _ => "",
        };
        if !hint.is_empty() {
            hints.push(hint.to_string())
        }
    }
    if hints.is_empty() {
        // fallback to a generic hint of expected tokens
        hints.push(format!("expected {}", expected.join(", ")));
    }
    hints
}


lazy_static! {
    static ref LITERALS: [& 'static str; 5] = ["int", "string", "true", "false", "unit"];
    static ref TYPES: [& 'static str; 4] = ["Int", "Bool", "String", "Unit"];
    static ref EXPRESSIONS: [& 'static str; 5] = ["let", "set", "if", "while", "new"];
}