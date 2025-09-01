use ratatui::text::Span;
pub fn rust_tokens(token: &str) -> Span<'static> {
    use ratatui::style::{Color, Style};
    let keywords = [
        "fn", "let", "mut", "if", "else", "while", "for", "in", "return", "struct", "enum", "impl",
        "trait", "const", "static", "use", "pub", "crate", "new", "union", "false", "true",
    ];

    let data_types = [
        "i8", "i16", "i32", "i64", "i128", "isize", "u8", "u16", "u32", "u64", "u128", "usize",
        "f32", "f64", "bool", "char", "str", "String", "array", "tuple", "slice", "Vec", "Option",
        "Result", "Box", "Rc", "Arc",
    ];

    let is_number = token.chars().all(|c| c.is_ascii_digit());
    let is_comment = token.starts_with("//");

    if is_comment {
        Span::styled(token.to_string(), Style::default().fg(Color::Black))
    } else if keywords.contains(&token) {
        Span::styled(token.to_string(), Style::default().fg(Color::LightBlue))
    } else if data_types.contains(&token) {
        Span::styled(token.to_string(), Style::default().fg(Color::Green))
    } else if is_number {
        Span::styled(token.to_string(), Style::default().fg(Color::Magenta))
    } else {
        Span::raw(token.to_string())
    }
}

