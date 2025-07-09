use ratatui::text::Span;
pub fn rust_tokens(token: &str) -> Span<'static> {
    use ratatui::style::{Color, Style};
    let keywords = [
        "fn", "let", "mut", "if", "else", "while", "for", "in", "return", "struct", "enum", "impl",
        "trait", "const", "static", "use", "pub", "crate",
    ];

    let data_types = [
        "i8",
        "i16",
        "i32",
        "i64",
        "i128",
        "isize",
        "u8",
        "u16",
        "u32",
        "u64",
        "u128",
        "usize",
        "f32",
        "f64",
        "bool",
        "char",
        "str",
        "String",
        "array",
        "tuple",
        "slice",
        "Vec",
        "Option",
        "Result",
        "Box",
        "Rc",
        "Arc",
        "Vec<String>",
        "Vec<i32>",
        "Vec<f64>",
        "Vec<bool>",
        "Vec<char>",
        "Vec<Vec<String>>",
        "Option<i32>",
        "Option<f64>",
        "Option<String>",
        "Option<Vec<String>>",
        "Result<i32, String>",
        "Result<(), String>",
        "Result<Option<String>, &'static str>",
        "Box<i32>",
        "Box<dyn Fn()>",
        "Box<[i32]>",
        "Box<str>",
        "Rc<String>",
        "Rc<Vec<i32>>",
        "Rc<RefCell<i32>>",
        "Arc<String>",
        "Arc<Mutex<i32>>",
        "Arc<RwLock<Vec<u8>>>",
        "&str",
        "&String",
        "&[i32]",
        "&mut [u8]",
        "(i32, i32)",
        "(String, bool)",
        "(i32, String, Vec<f64>)",
        "[i32; 3]",
        "[u8; 256]",
        "[char; 5]",
        "&'static str",
        "&'a T",
        "&'a mut T",
        "PhantomData<T>",
        "Cell<i32>",
        "RefCell<T>",
        "HashMap<String, i32>",
        "HashSet<String>",
        "BTreeMap<String, i32>",
        "BTreeSet<String>",
        "Cow<'a, str>",
        "Result<Vec<u8>, std::io::Error>",
        "Option<Result<String, E>>",
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
