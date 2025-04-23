use colored::Colorize;
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
        errors_str.push_str(&format!("\n  (+{} more errors)", suppressed_errors.len()).red().to_string());
    }
    errors_str
}

fn format_compile_err(e: &CompileError, path: &str, src: &str) -> String {
    match e {
        CompileError::Lexical(e) => {
            let label = "Lexical Error".red().bold().to_string();
            let span = e.span.clone();
            match e.kind.clone() {
                LexicalErrorKind::UnrecognizedToken => {
                    format_error(path, src, span, &label, "unrecognized token")
                }
                LexicalErrorKind::UnterminatedString => {
                    format_error(path, src, span, &label, "unterminated string")
                }
                LexicalErrorKind::InvalidInteger => {
                    format_error(path, src, span, &label, "invalid integer literal")
                }
                LexicalErrorKind::IntegerOverflow => {
                    format_error(path, src, span, &label, "integer overflow")
                }
                LexicalErrorKind::FloatingPointNumber => {
                    format_error(path, src, span, &label, "non-supported floating point number")
                }
            }
        }
        CompileError::Syntax(e) => {
            let label = "Syntax Error".red().bold().to_string();
            let span = e.span.clone();
            match e.kind.clone() {
                SyntaxErrorKind::UnexpectedToken(expected) => {
                    let mut str = format_error(path, src, span, &label, "unexpected token");
                    append_expected(&mut str, expected)
                }
                SyntaxErrorKind::UnexpectedEof(expected) => {
                    let mut str = format_error(path, src, span, &label,"unexpected end of input");
                    append_expected(&mut str, expected)
                }
                SyntaxErrorKind::InvalidToken => {
                    format_error(path, src, span, &label,"invalid token")
                }
                SyntaxErrorKind::ExtraToken => {
                    format_error(path, src, span, &label, "extra token")
                }
            }
        }
        CompileError::Semantic(e) => {
            match e {
                SemanticError::Declaration(e) => {
                    let label = "Declaration Error".red().bold().to_string();
                    let span = e.span.clone();
                    match e.kind.clone() {
                        DeclarationErrorKind::UndeclaredIdentifier(id) => {
                            let msg = if id == "_" {
                                "wildcard identifier cannot be used"
                            } else {
                                &format!("undeclared identifier '{}'", id.bold())
                            };
                            format_error(
                                path,
                                src,
                                span,
                                &label,
                                msg,
                            )
                        },
                        DeclarationErrorKind::DuplicateDeclaration(id) =>
                            format_error(
                                path,
                                src,
                                span,
                                &label,
                                &format!("duplicate identifier '{}' in the same scope", id.bold()),
                            ),
                        DeclarationErrorKind::ReservedIdentifier(id) =>
                            format_error(
                                path,
                                src,
                                span,
                                &label,
                                &format!("reserved identifier '{}' cannot be used", id.bold()),
                            ),
                        DeclarationErrorKind::WrongFunctionSignature { params_found, types_found } =>
                            format_error(
                                path,
                                src,
                                span,
                                &label,
                                &format!(
                                    "wrong function signature, found {} parameter(s) and {} type(s)",
                                    params_found.to_string().bold(),
                                    types_found.to_string().bold()
                                ),
                            ),
                    }
                }
                SemanticError::Type(e) => {
                    let label = "Type Error".red().bold().to_string();
                    let span = e.span.clone();
                    match e.kind.clone() {
                        TypeErrorKind::TypeMismatch { found, expected } => {
                            format_error(
                                path,
                                src,
                                span,
                                &label,
                                &format!("type mismatch, found {}, expected {}", found.to_text().bold(), expected.to_text().bold()),
                            )
                        }
                        TypeErrorKind::WrongNumberOfArguments { found, expected } => {
                            format_error(
                                path,
                                src,
                                span,
                                &label,
                                &format!("wrong number of arguments, found {}, expected {}", found.to_string().bold(), expected.to_string().bold()),
                            )
                        }
                        TypeErrorKind::NotCallable { found } => {
                            format_error(
                                path,
                                src,
                                span,
                                &label,
                                &format!("expression not callable, found {}, expected function", found.to_text().bold()),
                            )
                        }
                        TypeErrorKind::NotIndexable { found } => {
                            format_error(
                                path,
                                src,
                                span,
                                &label,
                                &format!("expression not indexable, found {}, expected array", found.to_text().bold()),
                            )
                        }
                    }
                }
            }
        }
    }
}

fn format_error(
    path: &str,
    source: &str,
    span: Span,
    label: &str,
    description: &str,
) -> String {
    let pos = get_position_in_src(source, span.start);
    let length = span.end - span.start;
    let error_line_str = get_error_line(source, pos, length);
    let location = format!("{}:{}:{}", path, pos.0, pos.1);
    format!(
        "{}\n{}: {} at line {}, column {}\n\t{}",
        location,
        label,
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

fn get_error_line(source: &str, pos: (usize, usize), length: usize) -> String {
    let (line, col) = pos;
    let line_str = source.lines().nth(line - 1).unwrap_or("");
    let remaining = line_str.len().saturating_sub(col.saturating_sub(1));
    format!(
        "{}\n\t{}{}",
        line_str,
        " ".repeat(col.saturating_sub(1)),
        "^".repeat(length.min(remaining)).red(),
    )
}
