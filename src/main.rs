use color_eyre::Result;
use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    layout::Position,
    style::{Color, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{block, Block, Paragraph},
    DefaultTerminal, Frame,
};
use std::{
    env,
    fs::{read_to_string, write, File},
    path::Path,
};

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let mut save_path = String::new();
    let mut file_text: Vec<String> = Vec::new();
    let mut file_opened: bool = false;
    if args.len() > 1 {
        let file_path = &args[1];
        save_path = file_path.to_string();
        if !file_path.is_empty() {
            if Path::new(file_path).exists() {
                let content = read_to_string(file_path).expect("Error: reading file error: ");
                let lines: Vec<String> = content.lines().map(|line| line.to_string()).collect();
                file_text = lines;
                file_opened = true;
            } else {
                File::create(&save_path).expect("Error: New file creating error.");
            }
        }
    } else {
        save_path = "new_file".to_string();
    }
    color_eyre::install()?;
    let terminal = ratatui::init();
    let app_result = App::new(save_path, file_text, file_opened).run(terminal);
    ratatui::restore();
    app_result
}

fn rust_tokens(token: &str) -> Span<'static> {
    let keywords = [
        "fn", "let", "mut", "if", "else", "while", "for", "in", "return", "struct", "enum", "impl",
        "trait", "const", "static", "use", "pub", "crate",
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

fn syntax_highln(line: &str) -> Line {
    let mut words: Vec<Span> = Vec::new();
    let mut membuf: String = String::new();
    let mut space = false;
    let mut string = false;

    for c in line.chars() {
        if string {
            membuf.push(c);
            if c == '"' {
                string = false;
                words.push(Span::styled(
                    membuf.clone(),
                    Style::default().fg(Color::Yellow),
                ));
                membuf.clear();
            }
        } else if c == '"' {
            if !membuf.is_empty() {
                words.push(rust_tokens(&membuf));
                membuf.clear();
            }
            membuf.push(c);
            string = true;
        } else if c.is_whitespace() {
            if !membuf.is_empty() {
                words.push(rust_tokens(&membuf));
                membuf.clear();
            }
            words.push(Span::raw(c.to_string()));
        } else {
            membuf.push(c);
        }
    }

    if !membuf.is_empty() {
        if string {
            words.push(Span::styled(
                membuf.clone(),
                Style::default().fg(Color::Yellow),
            ));
        } else {
            words.push(rust_tokens(&membuf));
        }
    }

    Line::from(words)
}

struct App {
    code: Vec<String>,
    column_index: usize,
    line_index: usize,
    input_mode: InputMode,
    scroll_ofst: usize,
    scroll_hofst: usize,
    save_path: String,
    file_open_text: Vec<String>,
    file_opened: bool,
    info_text: String,
    saved: bool,
}

enum InputMode {
    Normal,
    Editing,
}

impl App {
    fn new(save_path_arg: String, file_text: Vec<String>, file_opened_arg: bool) -> Self {
        Self {
            code: vec![String::new()],
            input_mode: InputMode::Normal,
            column_index: 0,
            line_index: 0,
            scroll_ofst: 0,
            scroll_hofst: 0,
            save_path: save_path_arg,
            file_open_text: file_text,
            file_opened: file_opened_arg,
            info_text: String::new(),
            saved: false,
        }
    }

    fn move_cursor_left(&mut self) {
        if self.column_index > 0 {
            self.column_index -= 1;
        }
    }

    fn move_cursor_right(&mut self) {
        let line_len = self
            .code
            .get(self.line_index)
            .map(|line| line.chars().count())
            .unwrap_or(0);
        if self.column_index < line_len {
            self.column_index += 1;
        }
    }

    fn move_cursor_up(&mut self) {
        if self.line_index > 0 {
            self.line_index -= 1;
            self.column_index = self.clamp_column_index(self.line_index, self.column_index);
        }
    }

    fn move_cursor_down(&mut self) {
        if self.line_index + 1 < self.code.len() {
            self.line_index += 1;
            self.column_index = self.clamp_column_index(self.line_index, self.column_index);
        }
    }

    fn enter_char(&mut self, new_char: char) {
        let index = self.byte_index();
        self.code[self.line_index].insert(index, new_char);
        self.move_cursor_right();
    }

    fn byte_index(&self) -> usize {
        self.code[self.line_index]
            .char_indices()
            .map(|(i, _)| i)
            .nth(self.column_index)
            .unwrap_or(self.code[self.line_index].len())
    }

    fn delete_char(&mut self) {
        let is_not_cursor_leftmost = self.column_index != 0;
        if is_not_cursor_leftmost {
            let current_index = self.column_index;
            let from_left_to_current_index = current_index - 1;
            let before_char_to_delete = self.code[self.line_index]
                .chars()
                .take(from_left_to_current_index);
            let after_char_to_delete = self.code[self.line_index].chars().skip(current_index);
            self.code[self.line_index] =
                before_char_to_delete.chain(after_char_to_delete).collect();
            self.move_cursor_left();
        }
    }

    fn clamp_column_index(&self, line_index: usize, column_index: usize) -> usize {
        let line_length = self
            .code
            .get(line_index)
            .map(|line| line.chars().count())
            .unwrap_or(0);
        column_index.clamp(0, line_length)
    }

    fn new_line(&mut self) {
        let line = self.code[self.line_index].clone();
        let byte_index = line
            .char_indices()
            .nth(self.column_index)
            .map(|(i, _)| i)
            .unwrap_or(line.len());

        let (left, right) = line.split_at(byte_index);
        self.code[self.line_index] = left.to_string();
        self.line_index += 1;
        self.code.insert(self.line_index, right.to_string());
        self.column_index = 0;
    }

    fn save_file(&mut self) {
        self.saved = true;
        write(&self.save_path, self.code.join("\n")).expect("");
    }
    fn open_file(&mut self) {
        self.code.clear();
        for line in &self.file_open_text {
            self.code.push(line.to_string());
        }
    }

    fn delete_line(&mut self) {
        self.code[self.line_index] = "".to_string();
    }

    fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        if self.file_opened {
            self.open_file();
        }
        loop {
            terminal.draw(|frame| self.draw(frame))?;

            if let Event::Key(key) = event::read()? {
                match self.input_mode {
                    InputMode::Normal => match key.code {
                        KeyCode::Char('i') => {
                            self.input_mode = InputMode::Editing;
                        }
                        KeyCode::Char('q') => {
                            return Ok(());
                        }
                        KeyCode::Char('s') => self.save_file(),
                        KeyCode::Char('o') => self.open_file(),
                        KeyCode::Char('d') => self.delete_line(),
                        KeyCode::Left => self.move_cursor_left(),
                        KeyCode::Right => self.move_cursor_right(),
                        KeyCode::Up => self.move_cursor_up(),
                        KeyCode::Down => self.move_cursor_down(),
                        KeyCode::Home => self.column_index = 0,
                        KeyCode::End => self.column_index = self.code[self.line_index].len(),
                        _ => {}
                    },
                    InputMode::Editing if key.kind == KeyEventKind::Press => match key.code {
                        KeyCode::Enter => self.new_line(),
                        KeyCode::Char(to_insert) => {
                            self.saved = false;
                            self.enter_char(to_insert);
                        }
                        KeyCode::Home => self.column_index = 0,
                        KeyCode::End => self.column_index = self.code[self.line_index].len(),
                        KeyCode::Backspace => {
                            if !self.code.is_empty() && self.line_index < self.code.len() {
                                if self.code[self.line_index].len() == 0 && self.line_index != 0 {
                                    self.code.remove(self.line_index);
                                    self.move_cursor_up();
                                    self.column_index = self.code[self.line_index].len();
                                } else if self.column_index == 0
                                    && self.code[self.line_index].len() == 0
                                {
                                    if self.line_index > 0 {
                                        let current_line = self.code[self.line_index].clone();
                                        self.code.remove(self.line_index);
                                        self.move_cursor_up();
                                        self.code[self.line_index].push_str(&current_line);
                                    }
                                } else {
                                    self.delete_char();
                                }
                            }
                        }
                        KeyCode::Left => self.move_cursor_left(),
                        KeyCode::Right => self.move_cursor_right(),
                        KeyCode::Up => self.move_cursor_up(),
                        KeyCode::Down => self.move_cursor_down(),
                        KeyCode::Esc => self.input_mode = InputMode::Normal,
                        _ => {}
                    },
                    InputMode::Editing => {}
                }
            }
        }
    }

    fn editing_mode_info(&mut self) {
        self.info_text = format!(
            "<{}> - x:{}|y:{} - mode: i - quit: ESC",
            self.save_path, self.column_index, self.line_index,
        );
    }

    fn save_mode_info(&mut self) {
        self.info_text = format!("File saved to path: <{}>", self.save_path,);
    }

    fn is_that_rustcode(&mut self) {
        todo!("rust code not implemented yet.");
    }

    fn normal_mode_info(&mut self) {
        self.info_text = format!("<{}> - edit: i, save: s, quit: q", self.save_path);
    }

    fn draw(&mut self, frame: &mut Frame) {
        match self.input_mode {
            InputMode::Normal => {
                if self.saved {
                    self.save_mode_info();
                } else {
                    self.normal_mode_info();
                }
            }
            InputMode::Editing => self.editing_mode_info(),
        }

        let edit_area = frame.area();

        let text_lines: Vec<Line> = self.code.iter().map(|line| syntax_highln(line)).collect();
        let text = Text::from(text_lines);
        let mut input = Paragraph::new(text)
            .style(match self.input_mode {
                InputMode::Normal => Style::default().fg(Color::Gray),
                InputMode::Editing => Style::default().fg(Color::White),
            })
            .block(
                Block::new()
                    .title(self.info_text.to_string())
                    .title_style(Style::default().fg(Color::Black).bg(Color::Gray))
                    .title_position(block::Position::Top),
            );
        let visible_height = edit_area.height.saturating_sub(1) as usize;
        let crsrL = self.line_index;
        let sheight = visible_height;
        let swidth = edit_area.width as usize;
        let stop = self.scroll_ofst;
        let sbottom = self.scroll_ofst + sheight.saturating_sub(1);

        if crsrL > sbottom {
            self.scroll_ofst = crsrL - sheight + 1;
        } else if crsrL < stop {
            self.scroll_ofst = crsrL;
        }

        if self.column_index >= self.scroll_hofst + swidth {
            self.scroll_hofst = self.column_index.saturating_sub(swidth).saturating_add(1);
        } else if self.column_index < self.scroll_hofst {
            self.scroll_hofst = self.column_index;
        }

        input = input.scroll((self.scroll_ofst as u16, self.scroll_hofst as u16));

        frame.render_widget(input, edit_area);
        frame.set_cursor_position(Position::new(
            edit_area.x + self.column_index as u16,
            edit_area.y + (self.line_index - self.scroll_ofst) as u16 + 1,
        ));
    }
}
