use std::fs;
use std::path::Path;
use colored::Colorize;
use crate::semantic::SemanticError;
use crate::syntax::ast::Span;

pub fn read_source_file(file: String) -> Result<String, String> {
    if file.is_empty() || !file.ends_with(".agu") {
        return Err("Invalid aguda file".to_string());
    }
    let path = Path::new(&file);
    if !path.exists() {
        return Err(format!("Cannot find source file '{}'", file));
    }
    match fs::read_to_string(path) {
        Ok(content) if content.trim().is_empty() => {
            Err(format!("Source file '{}' is empty", file))
        }
        Ok(content) => Ok(content),
        Err(e) => Err(format!("Error reading file '{}': {}", file, e)),
    }
}

pub fn format_error(
    source: &str,
    span: Span,
    label: &str,
    expected: Option<&[String]>
) -> String {
    let pos = get_position(source, span.start);
    let length = span.end - span.start;
    let error_line_str = get_error_line(source, pos, length);
    let mut msg = format!(
        "{} at line {}, column {}\n\t{}",
        label,
        pos.0,
        pos.1,
        error_line_str
    );
    if let Some(expected) = expected {
        msg.push_str(&format!(
            "\n{} {}",
            "Expected:".blue().bold(),
            expected.join(", ").blue()
        ));
    }
    msg
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

fn get_position(source: &str, index: usize) -> (usize, usize) {
    let mut line = 1;
    let mut col = 1;

    for (i, ch) in source.char_indices() {
        if i == index {
            break;
        }
        if ch == '\n' {
            line += 1;
            col = 1;
        } else {
            col += 1;
        }
    }
    (line, col)
}

pub fn indent(level: usize) -> String {
    "  ".repeat(level)
}

pub fn format_checker_errors<T>(
    errors: Vec<T>,
    src: &str,
    label: &str,
    max_errors: usize,
) -> String where T: SemanticError
{
    let mut formatted = errors
        .iter()
        .take(max_errors)
        .map(|e| format!("{} {}", label.red().bold(), e.get_message(src)))
        .collect::<Vec<_>>()
        .join("\n");

    let remaining = errors.len().saturating_sub(max_errors);
    if remaining > 0 {
        formatted.push_str(&format!("\n  (+{} more errors)", remaining).red().to_string());
    }
    formatted
}
