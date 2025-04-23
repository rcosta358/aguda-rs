use std::fs;
use std::path::Path;

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

pub fn get_position_in_src(source: &str, index: usize) -> (usize, usize) {
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
