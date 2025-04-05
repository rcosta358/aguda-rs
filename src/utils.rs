
pub fn format_error_with_line(
    source: &str,
    byte_index: usize,
    label: &str,
    expected: Option<&[String]>
) -> String {
    let pos = get_position(source, byte_index);
    let error_line_str = get_error_line(source, pos);
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

fn get_error_line(source: &str, pos: (usize, usize)) -> String {
    let (line, col) = pos;
    if let Some(line_str) = source.lines().nth(line - 1) {
        format!(
            "{}\n\t{}^",
            line_str,
            " ".repeat(col.saturating_sub(1))
        )
    } else {
        "".to_string()
    }
}

fn get_position(source: &str, index: usize) -> (usize, usize) {
    let mut line = 1;
    let mut col = 1;
    let mut last_newline = 0;

    for (i, ch) in source.char_indices() {
        if i == index {
            break;
        }
        if ch == '\n' {
            line += 1;
            last_newline = i;
            col = 1;
        } else {
            col += 1;
        }
    }
    (line, col)
}
