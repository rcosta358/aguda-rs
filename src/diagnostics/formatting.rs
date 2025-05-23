use colored::{Color, Colorize};
use crate::diagnostics::Diagnostic;
use crate::diagnostics::errors::*;
use crate::diagnostics::hints::get_syntax_hints;
use crate::diagnostics::warnings::Warning;
use crate::syntax::ast::Span;
use crate::utils::get_position_in_src;

pub fn format_aguda_errors(
    errors: Vec<AgudaError>,
    max_errors: usize,
    suppress_hints: bool,
    file: &str,
    src: &str
) -> String {
    let split_index = max_errors.min(errors.len());
    let (display_errors, suppressed_errors) = errors.split_at(split_index);
    let mut errors_str = display_errors
        .iter()
        .map(|e| format_aguda_error(e, suppress_hints, &file, &src))
        .collect::<Vec<_>>()
        .join("\n");
    if suppressed_errors.len() > 0 {
        errors_str.push_str(
            &format!("\n  (+{} more errors)", suppressed_errors.len()).red().bold().to_string()
        );
    }
    errors_str
}

fn format_aguda_error(e: &AgudaError, suppress_hints: bool, path: &str, src: &str) -> String {
    let diagnostic = Diagnostic::new(path, src, Color::Red, suppress_hints);
    match e {
        AgudaError::Compile(e) => match e {
            CompileError::Lexical(e) => {
                let label = "Lexical Error:";
                let description = match &e.kind {
                    LexicalErrorKind::UnrecognizedToken => "unrecognized token",
                    LexicalErrorKind::UnterminatedString => "unterminated string",
                    LexicalErrorKind::InvalidInteger => "invalid integer literal",
                    LexicalErrorKind::IntegerOverflow => "integer overflow",
                    LexicalErrorKind::FloatingPointNumber => "floating point number",
                };
                diagnostic.render(label, description, e.span.clone())
            }
            CompileError::Syntax(e) => {
                let label = "Syntax Error:";
                let (description, hints) = match e.kind.clone() {
                    SyntaxErrorKind::UnexpectedToken(expected, found) => {
                        ("unexpected token", get_syntax_hints(expected, Some(found)))
                    }
                    SyntaxErrorKind::UnexpectedEof(mut expected) => {
                        expected.push("!eof".to_string());
                        ("unexpected end of input", get_syntax_hints(expected, None))
                    }
                    SyntaxErrorKind::InvalidToken => ("invalid token", vec![]),
                    SyntaxErrorKind::ExtraToken => ("extra token", vec![]),
                };
                diagnostic
                    .hints(hints)
                    .render(label, description, e.span.clone())
            }
            CompileError::Semantic(e) => {
                match e {
                    SemanticError::Declaration(e) => {
                        let label = "Declaration Error:";
                        let (description, hint) = match e.kind.clone() {
                            DeclarationErrorKind::UndeclaredIdentifier(id, similar) => {
                                let msg = if id == "_" {
                                    "wildcard identifier cannot be used".to_string()
                                } else {
                                    format!("undeclared identifier {}", id.bold())
                                };
                                let hint = similar.map(|s| { format!("did you mean {}?", s.bold()) });
                                (msg, hint)
                            },
                            DeclarationErrorKind::DuplicateDeclaration(id) => {
                                let msg = format!("duplicate declaration of {}", id.bold());
                                (msg, None)
                            }
                            DeclarationErrorKind::ReservedIdentifier(id) => {
                                let msg = format!("reserved identifier {} cannot be used", id.bold());
                                (msg, None)
                            }
                            DeclarationErrorKind::FunctionSignatureMismatch { params_found, types_found } => {
                                let msg = format!(
                                    "wrong function signature, found {} parameter{} and {} type{}",
                                    params_found,
                                    if params_found > 1 { "s" } else { "" },
                                    types_found,
                                    if types_found > 1 { "s" } else { "" }
                                );
                                (msg, None)
                            }
                            DeclarationErrorKind::DuplicateMain => ("duplicate main function".to_string(), None),
                            DeclarationErrorKind::MissingMain => {
                                let msg = "missing main function".to_string();
                                let hint = format!(
                                    "define the entry point with {}",
                                    "let main(_) : Unit -> Unit".bold()
                                );
                                (msg, Some(hint))
                            }
                        };
                        diagnostic
                            .hints(hint.map(|h| vec![h]).unwrap_or_default())
                            .render(label, &description, e.span.clone())
                    }
                    SemanticError::Type(e) => {
                        let label = "Type Error:";
                        let description = match e.kind.clone() {
                            TypeErrorKind::TypeMismatch { found, expected } => {
                                format!(
                                    "type mismatch, found {}, expected {}",
                                    found.to_text().bold(),
                                    expected.to_text().bold()
                                )
                            }
                            TypeErrorKind::IncompatibleTypes(lhs, rhs) => {
                                format!(
                                    "expected equal types, found {} and {}",
                                    lhs.to_text().bold(),
                                    rhs.to_text().bold()
                                )
                            }
                            TypeErrorKind::ArgumentCountMismatch { found, expected } => {
                                format!(
                                    "wrong number of arguments, found {}, expected {}",
                                    found.to_string().bold(),
                                    expected.to_string().bold()
                                )
                            }
                            TypeErrorKind::NotCallable { found } => {
                                format!(
                                    "expression not callable, found {}, expected function",
                                    found.to_text().bold()
                                )
                            }
                            TypeErrorKind::NotIndexable { found } => {
                                format!(
                                    "expression not indexable, found {}, expected array",
                                    found.to_text().bold()
                                )
                            },
                            TypeErrorKind::MainSignatureMismatch => {
                                format!("main function must have signature {}", "Unit -> Unit".bold())
                            }
                        };
                        diagnostic.render(label, &description, e.span.clone())
                    }
                }
            }
        },
        AgudaError::Runtime(e) => {
            let label = "Runtime Error";
            diagnostic.render_simple(&label, &e.message)
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
    let diagnostic = Diagnostic::new(path, src, Color::Yellow, suppress_hints);
    let label = "Warning:";
    match warning {
        Warning::UnusedIdentifier(sym) => {
            let msg = format!("unused identifier {}", sym.value.bold());
            let hint = format!(
                "if this is intentional, prefix it with an underscore: {}",
                format!("_{}", sym.value).bold()
            );
            diagnostic
                .hints(vec![hint])
                .render(label, &msg, sym.span.clone())
        }
    }
}

pub fn format_hints(hints: Vec<String>) -> String {
    if hints.len() == 1 {
        format!("\n{} {}", "Hint:".cyan().bold(), hints.first().unwrap())
    } else {
        format!("\n{}\n- {}", "Hints:".cyan().bold(), hints.join("\n- "))
    }
}

pub fn format_message(
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
