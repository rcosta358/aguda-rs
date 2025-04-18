use crate::syntax::ast::Span;

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
        msg.push_str(&format!("\nExpected: {}", expected.join(", ")));
    }
    msg
}

fn get_error_line(source: &str, pos: (usize, usize), length: usize) -> String {
    let (line, col) = pos;
    if let Some(line_str) = source.lines().nth(line - 1) {
        format!(
            "{}\n\t{}{}",
            line_str,
            " ".repeat(col.saturating_sub(1)),
            "^".repeat(length),
        )
    } else {
        "".to_string()
    }
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
