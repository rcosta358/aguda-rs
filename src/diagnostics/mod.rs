use colored::{Color, Colorize};
use crate::diagnostics::formatting::{format_hints, format_message};
use crate::syntax::ast::Span;

pub mod formatting;
pub mod errors;
pub mod warnings;
pub mod hints;

struct Diagnostic<'a> {
    path: &'a str,
    src: &'a str,
    hints: Vec<String>,
    color: Color,
    suppress_hints: bool,
}

impl<'a> Diagnostic<'a> {
    fn new(
        path: &'a str,
        src: &'a str,
        color: Color,
        suppress_hints: bool,
    ) -> Self {
        Diagnostic {
            path,
            src,
            color,
            suppress_hints,
            hints: Vec::new(),
        }
    }

    fn hints(mut self, hints: Vec<String>) -> Self {
        self.hints = hints;
        self
    }

    fn render(self, label: &str, description: &str, span: Span) -> String {
        let mut out = format_message(self.path, self.src, span, label, description, self.color);
        if !self.suppress_hints && !self.hints.is_empty() {
            out.push_str(&format_hints(self.hints));
        }
        out
    }

    fn render_simple(self, label: &str, description: &str) -> String {
        let colored_label = format!("{}:", label).color(self.color).bold();
        format!("{} {}", colored_label, description)
    }
}