use crate::{
    document::{Document, Row},
    terminal::Terminal,
    Position,
};
use std::{env, io};
use termion::event::Key;

const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct Editor {
    should_quit: bool,
    terminal: Terminal,
    cursor_position: Position,
    offset: Position,
    document: Document,
}

impl Editor {
    pub fn default() -> Self {
        let terminal = Terminal::default().expect("failed to initialize Terminal");

        let args: Vec<String> = env::args().collect();
        let document = if args.len() > 1 {
            let file_name = &args[1];
            Document::open(file_name).unwrap_or_default()
        } else {
            Document::default()
        };

        Self {
            should_quit: false,
            terminal,
            cursor_position: Position::default(),
            offset: Position::default(),
            document,
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
        for terminal_row in 0..height - 1 {
            Terminal::clear_current_line();
            let row_index = terminal_row as usize + self.offset.y;
            if let Some(row) = self.document.row(row_index) {
                self.draw_row(row)
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

    fn process_keypress(&mut self) -> Result<(), io::Error> {
        let pressed_key = Terminal::read_key()?;
        match pressed_key {
            Key::Ctrl('q') => self.should_quit = true,
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
        match key {
            Key::Up => y = y.saturating_sub(1),
            Key::Down => {
                if y < height {
                    y = y.saturating_add(1);
                }
            }
            Key::Left => x = x.saturating_sub(1),
            Key::Right => {
                if x < width {
                    x = x.saturating_add(1);
                }
            }
            Key::PageUp => y = 0,
            Key::PageDown => y = height,
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
}

fn die(err: &io::Error) {
    Terminal::clear_screen();
    panic!("{}", err);
}
