use color_eyre::Result;
use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    layout::{Constraint, Layout, Position},
    style::{Color, Style, Stylize},
    text::{Line, Span, Text},
    widgets::Paragraph,
    DefaultTerminal, Frame,
};
use std::{
    env,
    fs::{read_to_string, write, File},
    path::Path,
};

mod rust;
use rust::rust_tokens;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let save_path;
    let mut file_text: Vec<String> = Vec::new();
    let mut file_opened: bool = false;
    if args.len() > 1 {
        let file_path = &args[1];
        save_path = file_path.to_string();
        if !file_path.is_empty() {
            if Path::new(file_path).exists() {
                let content = read_to_string(file_path).expect("Error: reading file error: ");
                let mut lines: Vec<String> = content.lines().map(|line| line.to_string()).collect();
                if lines.is_empty() {
                    lines.push(" ".to_string());
                }
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

fn syntax_highln(line: String) -> Line<'static> {
    let mut words: Vec<Span> = Vec::new();
    let mut membuf: String = String::new();
    let mut string: Option<char> = None;
    let mut escape = false;

    let token_chars = "(){}[],;+-*=%<>!&|^~:";

    for c in line.chars() {
        if let Some(quote) = string {
            membuf.push(c);
            if escape {
                escape = false;
            } else if c == '\\' {
                escape = true;
            } else if c == quote {
                string = None;
                words.push(Span::styled(
                    membuf.clone(),
                    Style::default().fg(Color::Yellow),
                ));
                membuf.clear();
            }
        } else if c == '"' || c == '\'' {
            if !membuf.is_empty() {
                words.push(rust_tokens(&membuf));
                membuf.clear();
            }
            membuf.push(c);
            string = Some(c);
        } else if c.is_whitespace() || token_chars.contains(c) {
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
        if string.is_some() {
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

fn find_impl(line: String, lookingfor: String) -> Line<'static> {
    let mut words: Vec<Span> = Vec::new();

    let mut last_index = 0;

    for (start, part) in line.match_indices(&lookingfor) {
        if start > last_index {
            words.push(Span::raw(line[last_index..start].to_string()));
        }

        words.push(Span::styled(
            part.to_string(),
            Style::default().bg(Color::White).fg(Color::Black),
        ));

        last_index = start + part.len();
    }

    if last_index < line.len() {
        words.push(Span::raw(line[last_index..].to_string()));
    }

    Line::from(words)
}

fn select_impl(line: String, startp: u32, endp: u32) -> Line<'static> {
    Line::from("select mode")
}

#[derive(Clone)]
struct History {
    code: Vec<String>,
    line_pos: usize,
    col_pos: usize,
}

struct App {
    code: Vec<String>,
    column_index: usize,
    line_index: usize,
    input_mode: InputMode,
    scroll_ofst: usize,
    scroll_hofst: usize,
    info_text: String,
    save_path: String,
    file_open_text: Vec<String>,
    file_opened: bool,
    saved: bool,
    find_str: String,
    history_undo: Vec<History>,
    history_redo: Vec<History>,
}

enum InputMode {
    Normal,
    Editing,
    Find,
    Select,
}

impl App {
    fn new(save_path_arg: String, file_text: Vec<String>, file_opened_arg: bool) -> Self {
        Self {
            code: vec![String::new()],
            input_mode: InputMode::Normal,
            column_index: 0,
            line_index: 0,
            scroll_ofst: 0,
            info_text: String::new(),
            scroll_hofst: 0,
            save_path: save_path_arg,
            file_open_text: file_text,
            file_opened: file_opened_arg,
            saved: false,
            find_str: String::new(),
            history_undo: Vec::new(),
            history_redo: Vec::new(),
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

    fn take_snapshot(&mut self) {
        let snapshot = History {
            code: self.code.clone(),
            line_pos: self.line_index,
            col_pos: self.column_index,
        };
        self.history_undo.push(snapshot);
        self.history_redo.clear();
    }
    fn undo(&mut self) {
        if let Some(snapshot) = self.history_undo.pop() {
            let current = History {
                code: self.code.clone(),
                line_pos: self.line_index,
                col_pos: self.column_index,
            };
            self.history_redo.push(current);

            self.code = snapshot.code;
            self.line_index = snapshot.line_pos;
            self.column_index = snapshot.col_pos;
        }
    }

    fn redo(&mut self) {
        if let Some(snapshot) = self.history_redo.pop() {
            let current = History {
                code: self.code.clone(),
                line_pos: self.line_index,
                col_pos: self.column_index,
            };
            self.history_undo.push(current);

            self.code = snapshot.code;
            self.line_index = snapshot.line_pos;
            self.column_index = snapshot.col_pos;
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
                        KeyCode::Char('/') => self.input_mode = InputMode::Find,
                        KeyCode::Char('d') => self.delete_line(),
                        KeyCode::Char('u') => self.undo(),
                        KeyCode::Char('r') => self.redo(),
                        KeyCode::Char('v') => self.input_mode = InputMode::Select,
                        KeyCode::Left => self.move_cursor_left(),
                        KeyCode::Right => self.move_cursor_right(),
                        KeyCode::Up => self.move_cursor_up(),
                        KeyCode::Down => self.move_cursor_down(),
                        KeyCode::Home => self.column_index = 0,
                        KeyCode::End => self.column_index = self.code[self.line_index].len(),
                        _ => {}
                    },
                    InputMode::Editing if key.kind == KeyEventKind::Press => match key.code {
                        KeyCode::Enter => {
                            self.take_snapshot();
                            self.new_line();
                        }
                        KeyCode::Char(to_insert) => {
                            self.take_snapshot();

                            self.saved = false;
                            self.enter_char(to_insert);

                            self.history_redo.clear();
                        }
                        KeyCode::Home => self.column_index = 0,
                        KeyCode::End => self.column_index = self.code[self.line_index].len(),
                        KeyCode::Backspace => {
                            self.take_snapshot();
                            if !self.code.is_empty() && self.line_index < self.code.len() {
                                if self.code[self.line_index].is_empty() && self.line_index != 0 {
                                    self.code.remove(self.line_index);
                                    self.move_cursor_up();
                                    self.column_index = self.code[self.line_index].len();
                                } else if self.column_index >= 2
                                    && self.code[self.line_index]
                                        .chars()
                                        .nth(self.column_index - 1)
                                        .map(|c| c == ' ')
                                        .unwrap_or(false)
                                    && self.code[self.line_index]
                                        .chars()
                                        .nth(self.column_index - 2)
                                        .map(|c| c == ' ')
                                        .unwrap_or(false)
                                {
                                    self.delete_char();
                                    self.delete_char();
                                } else if self.column_index == 0
                                    && self.code[self.line_index].is_empty()
                                {
                                    if self.line_index > 0 {
                                        let current_line = self.code[self.line_index].clone();
                                        self.code.remove(self.line_index);
                                        self.move_cursor_up();
                                        self.code[self.line_index].push_str(&current_line);
                                    }
                                } else if self.column_index > 0 {
                                    self.delete_char();
                                }
                            }
                        }
                        KeyCode::Tab => {
                            self.take_snapshot();
                            self.enter_char(' ');
                            self.enter_char(' ');
                        }
                        KeyCode::Left => self.move_cursor_left(),
                        KeyCode::Right => self.move_cursor_right(),
                        KeyCode::Up => self.move_cursor_up(),
                        KeyCode::Down => self.move_cursor_down(),
                        KeyCode::Esc => self.input_mode = InputMode::Normal,
                        _ => {}
                    },
                    InputMode::Find if key.kind == KeyEventKind::Press => match key.code {
                        KeyCode::Char(to_find) => {
                            self.saved = false;
                            self.find_str.push(to_find);
                            self.input_mode = InputMode::Find;
                        }
                        KeyCode::Backspace => {
                            self.find_str.pop();
                        }
                        KeyCode::Esc => self.input_mode = InputMode::Normal,
                        KeyCode::Left => self.move_cursor_left(),
                        KeyCode::Right => self.move_cursor_right(),
                        KeyCode::Up => self.move_cursor_up(),
                        KeyCode::Down => self.move_cursor_down(),

                        _ => {}
                    },
                    InputMode::Select if key.kind == KeyEventKind::Press => match key.code {
                        KeyCode::Left => {}
                        KeyCode::Right => {}
                        KeyCode::Up => {}
                        KeyCode::Down => {}
                        KeyCode::Esc => self.input_mode = InputMode::Normal,
                        _ => {}
                    },
                    InputMode::Editing => {}
                    InputMode::Find => {}
                    InputMode::Select => {}
                }
            }
        }
    }

    pub fn normal_info_text(&mut self) {
        self.info_text = format!(
            "  <{}> - edit: i, save: s, find: /, undo-redo: u-r, quit: q ",
            self.save_path
        );
    }

    pub fn edit_info_text(&mut self) {
        self.info_text = format!(
            " <{}> - x:{}|y:{} - quit: ESC ",
            self.save_path, self.column_index, self.line_index,
        );
    }

    pub fn find_info_text(&mut self) {
        self.info_text = format!(
            " Search in <{} for quit: ESC> : {}",
            self.save_path, self.find_str
        );
    }

    pub fn save_info_text(&mut self) {
        self.info_text = format!(" File saved to <{}>", self.save_path);
    }

    pub fn select_info_text(&mut self) {
        self.info_text = format!(
            "Selection x:{}, y:{}, <{}>",
            self.column_index, self.line_index, self.save_path
        );
    }

    fn draw(&mut self, frame: &mut Frame) {
        let vertical = Layout::vertical([Constraint::Length(1), Constraint::Min(1)]);
        let [status_area, edit_area] = vertical.areas(frame.area());
        // let helped_layout = Layout::vertical([
        // Constraint::Length(1),
        // Constraint::Length(2),
        // Constraint::Min(1),
        // ]);

        match self.input_mode {
            InputMode::Normal => {
                if !self.saved {
                    self.normal_info_text();
                } else {
                    self.save_info_text();
                }
            }
            InputMode::Editing => self.edit_info_text(),
            InputMode::Find => self.find_info_text(),
            InputMode::Select => self.select_info_text(),
        }

        let (msg, style) = match self.input_mode {
            InputMode::Normal => (
                vec![
                    " Normal ".bg(Color::Yellow),
                    "".bg(Color::Gray).fg(Color::Yellow),
                    "".fg(Color::Gray).bg(Color::DarkGray),
                    self.info_text
                        .to_string()
                        .fg(Color::White)
                        .bg(Color::DarkGray),
                    "".fg(Color::DarkGray),
                ],
                Style::default().fg(Color::Black),
            ),
            InputMode::Editing => (
                vec![
                    " Edit ".bg(Color::LightBlue),
                    "".bg(Color::Gray).fg(Color::LightBlue),
                    "".fg(Color::Gray).bg(Color::DarkGray),
                    self.info_text
                        .to_string()
                        .fg(Color::White)
                        .bg(Color::DarkGray),
                    "".fg(Color::DarkGray),
                ],
                Style::default().fg(Color::Black),
            ),
            InputMode::Find => (
                vec![
                    " Find ".bg(Color::Red),
                    "".bg(Color::Gray).fg(Color::Red),
                    "".fg(Color::Gray).bg(Color::DarkGray),
                    self.info_text
                        .to_string()
                        .fg(Color::White)
                        .bg(Color::DarkGray),
                    "".fg(Color::DarkGray),
                ],
                Style::default().fg(Color::Black),
            ),
            InputMode::Select => (
                vec![
                    " Select ".bg(Color::Green),
                    "".bg(Color::Gray).fg(Color::Green),
                    "".fg(Color::Gray).bg(Color::DarkGray),
                    self.info_text
                        .to_string()
                        .fg(Color::White)
                        .bg(Color::DarkGray),
                    "".fg(Color::DarkGray),
                ],
                Style::default().fg(Color::Black),
            ),
        };

        let status_bar_text = Text::from(Line::from(msg)).patch_style(style);
        let status_bar = Paragraph::new(status_bar_text);

        frame.render_widget(status_bar, status_area);

        let width = self.code.len().to_string().len();
        let text_lines: Vec<Line> = match self.input_mode {
            InputMode::Normal | InputMode::Editing => {
                self.find_str.clear();
                self.code
                    .iter()
                    .enumerate()
                    .map(|(i, code_line)| syntax_highln(format!("{i:>width$} {code_line}")))
                    .collect()
            }
            InputMode::Find => self
                .code
                .iter()
                .map(|line| find_impl(line.to_string(), self.find_str.clone()))
                .collect(),
            InputMode::Select => self
                .code
                .iter()
                .map(|line| find_impl(line.to_string(), self.find_str.clone()))
                .collect(),
        };

        let text = Text::from(text_lines);
        let mut input = Paragraph::new(text).style(match self.input_mode {
            InputMode::Normal => Style::default().fg(Color::Gray),
            InputMode::Editing => Style::default().fg(Color::White),
            InputMode::Find => Style::default().fg(Color::White),
            InputMode::Select => Style::default().fg(Color::White),
        });
        let visible_height = edit_area.height.saturating_sub(1) as usize;
        let crsrl = self.line_index;
        let sheight = visible_height;
        let swidth = edit_area.width as usize;
        let stop = self.scroll_ofst;
        let sbottom = self.scroll_ofst + sheight.saturating_sub(1);

        if crsrl > sbottom {
            self.scroll_ofst = crsrl - sheight + 1;
        } else if crsrl < stop {
            self.scroll_ofst = crsrl;
        }

        if self.column_index >= self.scroll_hofst + swidth {
            self.scroll_hofst = self.column_index.saturating_sub(swidth).saturating_add(1);
        } else if self.column_index < self.scroll_hofst {
            self.scroll_hofst = self.column_index;
        }

        input = input.scroll((self.scroll_ofst as u16, self.scroll_hofst as u16));
        frame.render_widget(input, edit_area);
        frame.set_cursor_position(Position::new(
            edit_area.x + self.column_index as u16 + width as u16 + 1,
            edit_area.y + (self.line_index - self.scroll_ofst) as u16,
        ));
    }
}
