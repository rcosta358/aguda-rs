use colored::{Color, Colorize};
use crate::errors::*;
use crate::syntax::ast::Span;
use crate::utils::get_position_in_src;

pub fn format_compile_errors(
    errors: Vec<CompileError>,
    max_errors: usize,
    file: &str,
    src: &str
) -> String {
    let split_index = max_errors.min(errors.len());
    let (display_errors, suppressed_errors) = errors.split_at(split_index);
    let mut errors_str = display_errors
        .iter()
        .map(|e| format_compile_err(e, &file, &src))
        .collect::<Vec<_>>()
        .join("\n");
    if suppressed_errors.len() > 0 {
        errors_str.push_str(&format!("\n  (+{} more errors)", suppressed_errors.len()).red().bold().to_string());
    }
    errors_str
}

fn format_compile_err(e: &CompileError, path: &str, src: &str) -> String {
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
                SyntaxErrorKind::UnexpectedToken(expected) => {
                    let mut str = format_message(path, src, span, &label, "unexpected token", Color::Red);
                    append_expected(&mut str, expected)
                }
                SyntaxErrorKind::UnexpectedEof(expected) => {
                    let mut str = format_message(path, src, span, &label, "unexpected end of input", Color::Red);
                    append_expected(&mut str, expected)
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
                        DeclarationErrorKind::UndeclaredSymbol(id) => {
                            let msg = if id == "_" {
                                "wildcard identifier cannot be used"
                            } else {
                                &format!("undeclared symbol '{}'", id.bold())
                            };
                            format_message(
                                path,
                                src,
                                span,
                                &label,
                                msg,
                                Color::Red,
                            )
                        },
                        DeclarationErrorKind::DuplicateDeclaration(id) =>
                            format_message(
                                path,
                                src,
                                span,
                                &label,
                                &format!("duplicate symbol '{}' in the same scope", id.bold()),
                                Color::Red,
                            ),
                        DeclarationErrorKind::ReservedIdentifier(id) =>
                            format_message(
                                path,
                                src,
                                span,
                                &label,
                                &format!("reserved identifier '{}' cannot be used", id.bold()),
                                Color::Red,
                            ),
                        DeclarationErrorKind::WrongFunctionSignature { params_found, types_found } =>
                            format_message(
                                path,
                                src,
                                span,
                                &label,
                                &format!(
                                    "wrong function signature, found {} parameter(s) and {} type(s)",
                                    params_found.to_string().bold(),
                                    types_found.to_string().bold()
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
                                span,
                                &label,
                                &format!("type mismatch, found {}, expected {}", found.to_text().bold(), expected.to_text().bold()),
                                Color::Red,
                            )
                        }
                        TypeErrorKind::WrongNumberOfArguments { found, expected } => {
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
    let error_line_str = get_error_line(source, pos, length, color);
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

fn append_expected(msg: &mut String, expected: Vec<String>) -> String {
    msg.push_str(&format!(
        "\n{} {}",
        "Expected:".bold(),
        expected.join(", ")
    ));
    msg.clone()
}

fn get_error_line(
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

pub fn format_warnings(
    warnings: Vec<Warning>,
    max_warnings: usize,
    file: &str,
    src: &str
) -> String {
    let split_index = max_warnings.min(warnings.len());
    let (display_warnings, suppressed_warnings) = warnings.split_at(split_index);
    let mut warnings_str = display_warnings
        .iter()
        .map(|w| format_warning(w, &file, &src))
        .collect::<Vec<_>>()
        .join("\n");
    if suppressed_warnings.len() > 0 {
        warnings_str.push_str(&format!("\n  (+{} more warnings)", suppressed_warnings.len()).yellow().bold().to_string());
    }
    warnings_str
}

fn format_warning(w: &Warning, path: &str, src: &str) -> String {
    match w {
        Warning::UnusedSymbol(sym) => {
            let label = "Warning:".yellow().bold().to_string();
            let span = sym.span.clone();
            format_message(
                path,
                src,
                span,
                &label,
                &format!("unused symbol '{}'", sym.value.bold()),
                Color::Yellow,
            )
        }
    }
}