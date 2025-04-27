use colored::{Color, Colorize};
use crate::diagnostics::errors::*;
use crate::diagnostics::hints::get_syntax_hints;
use crate::diagnostics::warnings::Warning;
use crate::syntax::ast::{Span, Type};
use crate::syntax::lexer::Token;
use crate::utils::get_position_in_src;

pub fn format_errors(
    errors: Vec<CompileError>,
    max_errors: usize,
    suppress_hints: bool,
    file: &str,
    src: &str
) -> String {
    let split_index = max_errors.min(errors.len());
    let (display_errors, suppressed_errors) = errors.split_at(split_index);
    let mut errors_str = display_errors
        .iter()
        .map(|e| format_error(e, suppress_hints, &file, &src))
        .collect::<Vec<_>>()
        .join("\n");
    if suppressed_errors.len() > 0 {
        errors_str.push_str(
            &format!("\n  (+{} more errors)", suppressed_errors.len()).red().bold().to_string()
        );
    }
    errors_str
}

fn format_error(e: &CompileError, suppress_hints: bool, path: &str, src: &str) -> String {
    match e {
        CompileError::Lexical(e) => {
            let label = "Lexical Error:";
            let span = e.span.clone();
            match e.kind.clone() {
                LexicalErrorKind::UnrecognizedToken => {
                    format_message(path, src, span, &label, "unrecognized token", Color::Red)
                }
                LexicalErrorKind::UnterminatedString => {
                    format_message(path, src, span, &label, "unterminated string", Color::Red)
                }
                LexicalErrorKind::InvalidInteger => {
                    format_message(path, src, span, &label, "invalid integer literal", Color::Red)
                }
                LexicalErrorKind::IntegerOverflow => {
                    format_message(path, src, span, &label, "integer overflow", Color::Red)
                }
                LexicalErrorKind::FloatingPointNumber => {
                    format_message(path, src, span, &label, "non-supported floating point number", Color::Red)
                }
            }
        }
        CompileError::Syntax(e) => {
            let label = "Syntax Error:";
            let span = e.span.clone();
            match e.kind.clone() {
                SyntaxErrorKind::UnexpectedToken(expected, found) => {
                    let mut msg = format_message(path, src, span, &label, "unexpected token", Color::Red);
                    if !suppress_hints {
                        msg.push_str(&format_hints(expected, Some(found)))
                    }
                    msg
                }
                SyntaxErrorKind::UnexpectedEof(mut expected) => {
                    let mut msg = format_message(path, src, span, &label, "unexpected end of input", Color::Red);
                    if !suppress_hints {
                        expected.push("!eof".to_string());
                        msg.push_str(&format_hints(expected, None));
                    }
                    msg
                }
                SyntaxErrorKind::InvalidToken => {
                    format_message(path, src, span, &label, "invalid token", Color::Red)
                }
                SyntaxErrorKind::ExtraToken => {
                    format_message(path, src, span, &label, "extra token", Color::Red)
                }
            }
        }
        CompileError::Semantic(e) => {
            match e {
                SemanticError::Declaration(e) => {
                    let label = "Declaration Error:";
                    let span = e.span.clone();
                    match e.kind.clone() {
                        DeclarationErrorKind::UndeclaredIdentifier(id, similar) => {
                            let msg = if id == "_" {
                                "wildcard identifier cannot be used"
                            } else {
                                &format!("undeclared identifier {}", id.bold())
                            };
                            let mut msg = format_message(
                                path,
                                src,
                                span,
                                &label,
                                msg,
                                Color::Red,
                            );
                            if !suppress_hints {
                                if let Some(similar) = similar {
                                    msg.push_str(&format!(
                                        "\n{} did you mean {}?",
                                        "Hint:".cyan().bold(),
                                        similar.bold()
                                    ));
                                }
                            }
                            msg
                        },
                        DeclarationErrorKind::DuplicateDeclaration(id) =>
                            format_message(
                                path,
                                src,
                                span,
                                &label,
                                &format!("duplicate declaration of {}", id.bold()),
                                Color::Red,
                            ),
                        DeclarationErrorKind::ReservedIdentifier(id) =>
                            format_message(
                                path,
                                src,
                                span,
                                &label,
                                &format!("reserved identifier {} cannot be used", id.bold()),
                                Color::Red,
                            ),
                        DeclarationErrorKind::FunctionSignatureMismatch { params_found, types_found } =>
                            format_message(
                                path,
                                src,
                                span,
                                &label,
                                &format!(
                                    "wrong function signature, found {} parameter{} and {} type{}",
                                    params_found,
                                    if params_found > 1 { "s" } else { "" },
                                    types_found,
                                    if types_found > 1 { "s" } else { "" }
                                ),
                                Color::Red,
                            ),
                    }
                }
                SemanticError::Type(e) => {
                    let label = "Type Error:";
                    let span = e.span.clone();
                    match e.kind.clone() {
                        TypeErrorKind::TypeMismatch { found, expected } => {
                            format_message(
                                path,
                                src,
                                span.clone(),
                                &label,
                                &format!(
                                    "type mismatch, found {}, expected {}",
                                    found.to_text().bold(),
                                    expected.to_text().bold()
                                ),
                                Color::Red,
                            )
                        }
                        TypeErrorKind::IncompatibleTypes(lhs, rhs) => {
                            let mut msg = format_message(
                                path,
                                src,
                                span,
                                &label,
                                &format!("expected equal types, found {} and {}", lhs.to_text().bold(), rhs.to_text().bold()),
                                Color::Red,
                            );
                            if !suppress_hints && rhs == Type::Unit {
                                msg.push_str(
                                    &format!(
                                        "\n{} when using an if expression without an 'else' branch, the 'then' branch must be of type {}",
                                        "Hint:".cyan().bold(),
                                        "Unit".bold()
                                    )
                                )
                            };
                            msg
                        }
                        TypeErrorKind::ArgumentCountMismatch { found, expected } => {
                            format_message(
                                path,
                                src,
                                span,
                                &label,
                                &format!("wrong number of arguments, found {}, expected {}", found.to_string().bold(), expected.to_string().bold()),
                                Color::Red,
                            )
                        }
                        TypeErrorKind::NotCallable { found } => {
                            format_message(
                                path,
                                src,
                                span,
                                &label,
                                &format!("expression not callable, found {}, expected function", found.to_text().bold()),
                                Color::Red,
                            )
                        }
                        TypeErrorKind::NotIndexable { found } => {
                            format_message(
                                path,
                                src,
                                span,
                                &label,
                                &format!("expression not indexable, found {}, expected array", found.to_text().bold()),
                                Color::Red,
                            )
                        }
                    }
                }
            }
        }
    }
}

pub fn format_warnings(
    warnings: Vec<Warning>,
    max_warnings: usize,
    suppress_hints: bool,
    file: &str,
    src: &str
) -> String {
    let split_index = max_warnings.min(warnings.len());
    let (display_warnings, suppressed_warnings) = warnings.split_at(split_index);
    let mut warnings_str = display_warnings
        .iter()
        .map(|w| format_warning(w, suppress_hints, &file, &src))
        .collect::<Vec<_>>()
        .join("\n");
    if suppressed_warnings.len() > 0 {
        warnings_str.push_str(&format!("\n  (+{} more warnings)", suppressed_warnings.len()).yellow().bold().to_string());
    }
    warnings_str
}

fn format_warning(warning: &Warning, suppress_hints: bool, path: &str, src: &str) -> String {
    let label = "Warning:".yellow().bold().to_string();
    match warning {
        Warning::UnusedIdentifier(sym) => {
            let mut msg = format_message(
                path,
                src,
                sym.span.clone(),
                &label,
                &format!("unused identifier {}", sym.value.bold()),
                Color::Yellow,
            );
            if !suppress_hints {
                let hint = format!(
                    "\n{} if this is intentional, prefix it with an underscore: {}",
                    "Hint:".cyan().bold(),
                    format!("_{}", sym.value).bold()
                );
                msg.push_str(&hint);
            }
            msg
        },
        Warning::RedefinedVariable(id) => {
            format_message(
                path,
                src,
                id.span.clone(),
                &label,
                &format!("the variable {} is redefined in the same scope", id.value.bold()),
                Color::Yellow,
            )
        }
    }
}

pub fn format_hints(expected: Vec<String>, found: Option<Token>) -> String {
    let hints = get_syntax_hints(expected, found);
    format!("\n{}\n- {}", "Hints:".cyan().bold(), hints.join("\n- "))
}

fn format_message(
    path: &str,
    source: &str,
    span: Span,
    label: &str,
    description: &str,
    color: Color,
) -> String {
    let pos = get_position_in_src(source, span.start);
    let length = span.end - span.start;
    let error_line_str = get_line_in_src(source, pos, length, color);
    let location = format!("{}:{}:{}", path, pos.0, pos.1);
    format!(
        "{}\n{} {} at line {}, column {}\n\t{}",
        location,
        label.color(color).bold().to_string(),
        description,
        pos.0,
        pos.1,
        error_line_str
    )
}

fn get_line_in_src(
    source: &str,
    pos: (usize, usize),
    length: usize,
    color: Color,
) -> String {
    let (line, col) = pos;
    let line_str = source.lines().nth(line - 1).unwrap_or("");
    let remaining = line_str.len().saturating_sub(col.saturating_sub(1));
    format!(
        "{}\n\t{}{}",
        line_str,
        " ".repeat(col.saturating_sub(1)),
        "^".repeat(length.min(remaining)).color(color).bold(),
    )
}
