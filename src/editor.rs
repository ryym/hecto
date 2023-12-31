use crate::{document::Document, row::Row, terminal::Terminal, Position, SearchDirection};
use std::{
    env, io,
    time::{Duration, Instant},
};
use termion::{color, event::Key};

const VERSION: &str = env!("CARGO_PKG_VERSION");
const STATUS_FG_COLOR: color::Rgb = color::Rgb(63, 63, 63);
const STATUS_BG_COLOR: color::Rgb = color::Rgb(239, 239, 239);
const QUIT_TIMES: u8 = 3;

pub struct Editor {
    should_quit: bool,
    terminal: Terminal,
    cursor_position: Position,
    offset: Position,
    document: Document,
    status_message: StatusMessage,
    quit_times: u8,
}

impl Editor {
    pub fn default() -> Self {
        let terminal = Terminal::default().expect("failed to initialize Terminal");
        let mut initial_status =
            String::from("HELP: Ctrl-F = find | Ctrl-S = save | Ctrl-Q = quit");

        let args: Vec<String> = env::args().collect();
        let document = if args.len() > 1 {
            let file_name = &args[1];
            if let Ok(doc) = Document::open(file_name) {
                doc
            } else {
                initial_status = format!("ERR: Could not open file: {file_name}");
                Document::default()
            }
        } else {
            Document::default()
        };

        Self {
            should_quit: false,
            terminal,
            cursor_position: Position::default(),
            offset: Position::default(),
            document,
            status_message: StatusMessage::from(initial_status),
            quit_times: QUIT_TIMES,
        }
    }

    pub fn run(&mut self) {
        loop {
            if let Err(err) = self.refresh_screen() {
                die(&err);
            }
            if self.should_quit {
                break;
            }
            if let Err(err) = self.process_keypress() {
                die(&err);
            }
        }
    }

    fn refresh_screen(&self) -> Result<(), io::Error> {
        Terminal::cursor_hide();
        Terminal::cursor_position(&Position::default());
        if self.should_quit {
            Terminal::clear_screen();
            println!("Goodbye.\r");
        } else {
            self.draw_rows();
            self.draw_status_bar();
            self.draw_message_bar();
            Terminal::cursor_position(&Position {
                x: self.cursor_position.x.saturating_sub(self.offset.x),
                y: self.cursor_position.y.saturating_sub(self.offset.y),
            });
        }
        Terminal::cursor_show();
        Terminal::flush()
    }

    fn draw_rows(&self) {
        let height = self.terminal.size().height;
        for terminal_row in 0..height {
            Terminal::clear_current_line();
            let row_index = terminal_row as usize + self.offset.y;
            if let Some(row) = self.document.row(row_index) {
                self.draw_row(row);
            } else if self.document.is_empty() && terminal_row == height / 3 {
                self.draw_welcome_message();
            } else {
                println!("~\r");
            }
        }
    }

    fn draw_row(&self, row: &Row) {
        let start = self.offset.x;
        let width = self.terminal.size().width as usize;
        let row = row.render(start, start + width);
        println!("{row}\r");
    }

    fn draw_welcome_message(&self) {
        let mut msg = format!("Hecto editor -- versoin {VERSION}\r");
        let width = self.terminal.size().width as usize;
        let padding = width.saturating_sub(msg.len()) / 2;
        let spaces = " ".repeat(padding.saturating_sub(1));
        msg = format!("~{spaces}{msg}");
        msg.truncate(width);
        println!("{msg}\r");
    }

    fn draw_status_bar(&self) {
        let mut file_name = self
            .document
            .file_name
            .clone()
            .unwrap_or_else(|| "[No Name]".to_string());
        file_name.truncate(20);

        let modified = if self.document.is_dirty() {
            " (modified)"
        } else {
            ""
        };

        let mut status = format!("{} - {} lines{}", file_name, self.document.len(), modified);

        let line_indicator = format!(
            "{}/{}",
            self.cursor_position.y.saturating_add(1),
            self.document.len()
        );
        let len = status.len() + line_indicator.len();

        let width = self.terminal.size().width as usize;
        if len < width {
            status.push_str(&" ".repeat(width - len));
        }
        status = format!("{status}{line_indicator}");
        status.truncate(width);

        Terminal::set_bg_color(STATUS_BG_COLOR);
        Terminal::set_fg_color(STATUS_FG_COLOR);
        println!("{status}\r");
        Terminal::reset_fg_color();
        Terminal::reset_bg_color();
    }

    fn draw_message_bar(&self) {
        Terminal::clear_current_line();
        let message = &self.status_message;
        if message.time.elapsed() < Duration::from_secs(5) {
            let mut text = message.text.clone();
            text.truncate(self.terminal.size().width as usize);
            print!("{text}");
        }
    }

    fn process_keypress(&mut self) -> Result<(), io::Error> {
        let pressed_key = Terminal::read_key()?;
        match pressed_key {
            Key::Ctrl('q') => {
                if self.document.is_dirty() && self.quit_times > 0 {
                    self.status_message = StatusMessage::from(format!(
                        "WARNING! File has unsaved changes. Press Ctrl-Q {} more times to quit.",
                        self.quit_times
                    ));
                    self.quit_times -= 1;
                    return Ok(());
                }
                self.should_quit = true;
            }
            Key::Ctrl('s') => self.save(),
            Key::Ctrl('f') => self.search(),
            Key::Char(c) => {
                self.document.insert(&self.cursor_position, c);
                self.move_cursor(Key::Right);
            }
            Key::Delete => self.document.delete(&self.cursor_position),
            Key::Backspace => {
                if self.cursor_position.x > 0 || self.cursor_position.y > 0 {
                    self.move_cursor(Key::Left);
                    self.document.delete(&self.cursor_position);
                }
            }
            Key::Up
            | Key::Down
            | Key::Left
            | Key::Right
            | Key::PageUp
            | Key::PageDown
            | Key::Home
            | Key::End => self.move_cursor(pressed_key),
            _ => {}
        }
        self.scroll();
        if self.quit_times < QUIT_TIMES {
            self.quit_times = QUIT_TIMES;
            self.status_message = StatusMessage::from(String::new());
        }
        Ok(())
    }

    fn move_cursor(&mut self, key: Key) {
        let Position { mut x, mut y } = self.cursor_position;
        let height = self.document.len();
        let width = if let Some(row) = self.document.row(y) {
            row.len()
        } else {
            0
        };
        let terminal_height = self.terminal.size().height as usize;
        match key {
            Key::Up => y = y.saturating_sub(1),
            Key::Down => {
                if y < height {
                    y = y.saturating_add(1);
                }
            }
            Key::Left => {
                if x > 0 {
                    x -= 1;
                } else if y > 0 {
                    y -= 1;
                    x = if let Some(row) = self.document.row(y) {
                        row.len()
                    } else {
                        0
                    };
                }
            }
            Key::Right => {
                if x < width {
                    x += 1;
                } else if y < height {
                    y += 1;
                    x = 0;
                }
            }
            Key::PageUp => y = y.saturating_sub(terminal_height),
            Key::PageDown => {
                y = if y.saturating_add(terminal_height) < height {
                    y + terminal_height
                } else {
                    height
                }
            }
            Key::Home => x = 0,
            Key::End => x = width,
            _ => {}
        }

        // Prevent x from exceeding the current line width after y is changed.
        let width = if let Some(row) = self.document.row(y) {
            row.len()
        } else {
            0
        };
        if x > width {
            x = width;
        }

        self.cursor_position = Position { x, y }
    }

    /// Update the cursor offset based on the terminal size and
    /// the current cursor position within the document.
    fn scroll(&mut self) {
        let width = self.terminal.size().width as usize;
        let height = self.terminal.size().height as usize;
        let pos = &self.cursor_position;
        let offset = &mut self.offset;
        if pos.y < offset.y {
            offset.y = pos.y;
        } else if pos.y >= offset.y.saturating_add(height) {
            offset.y = pos.y.saturating_sub(height).saturating_add(1);
        }
        if pos.x < offset.x {
            offset.x = pos.x;
        } else if pos.x >= offset.x.saturating_add(width) {
            offset.x = pos.x.saturating_sub(width).saturating_add(1);
        }
    }

    fn save(&mut self) {
        if self.document.file_name.is_none() {
            let file_name = self.prompt("Save as: ", |_, _, _| {}).unwrap_or(None);
            if file_name.is_none() {
                self.status_message = StatusMessage::from("Save aborted.".to_string());
                return;
            }
            self.document.file_name = file_name;
        }
        self.status_message = if self.document.save().is_ok() {
            StatusMessage::from("File saved successfully.".to_string())
        } else {
            StatusMessage::from("Error writing file!".to_string())
        };
    }

    fn search(&mut self) {
        let mut direction = SearchDirection::Forward;
        let mut last_match_query: Option<String> = None;
        let old_position = self.cursor_position.clone();
        let query = self.prompt(
            "Search (ESC to cancel, Arrows to navigate): ",
            |editor, key, query| {
                let mut moved = false;
                match key {
                    Key::Right | Key::Down => {
                        direction = SearchDirection::Forward;
                        // Proceed the position to search the next match.
                        editor.move_cursor(Key::Right);
                        moved = true;
                    }
                    Key::Left | Key::Up => direction = SearchDirection::Backward,
                    _ => direction = SearchDirection::Forward,
                }
                let position = editor
                    .document
                    .find(query, &editor.cursor_position, direction);
                if let Some(position) = position {
                    last_match_query = Some(query.clone());
                    editor.cursor_position = position;
                    editor.scroll();
                } else if moved {
                    editor.move_cursor(Key::Left);
                }
            },
        );
        if let (Ok(Some(query)), Some(last_match_query)) = (query, last_match_query) {
            if query == last_match_query {
                return;
            } else {
                self.status_message = StatusMessage::from(format!("Not found: {query}"));
            }
        }
        self.cursor_position = old_position;
        self.scroll();
    }

    fn prompt<C>(&mut self, prompt: &str, mut callback: C) -> Result<Option<String>, io::Error>
    where
        C: FnMut(&mut Self, Key, &String),
    {
        let mut result = String::new();
        loop {
            self.status_message = StatusMessage::from(format!("{prompt}{result}"));
            self.refresh_screen()?;
            let key = Terminal::read_key()?;
            match key {
                Key::Esc => {
                    result.truncate(0);
                    break;
                }
                Key::Backspace => {
                    if !result.is_empty() {
                        result.truncate(result.len() - 1);
                    }
                }
                Key::Char('\n') => break,
                Key::Char(c) => {
                    if !c.is_control() {
                        result.push(c);
                    }
                }
                _ => {}
            }
            callback(self, key, &result);
        }
        self.status_message = StatusMessage::from(String::new());
        if result.is_empty() {
            Ok(None)
        } else {
            Ok(Some(result))
        }
    }
}

struct StatusMessage {
    text: String,
    time: Instant,
}

impl StatusMessage {
    fn from(text: String) -> Self {
        Self {
            text,
            time: Instant::now(),
        }
    }
}

fn die(err: &io::Error) {
    Terminal::clear_screen();
    panic!("{}", err);
}
