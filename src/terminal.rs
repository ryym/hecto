use crate::Position;
use std::io::{self, Stdout, Write};
use termion::{
    event::Key,
    input::TermRead,
    raw::{IntoRawMode, RawTerminal},
};

pub struct Size {
    pub width: u16,
    pub height: u16,
}

pub struct Terminal {
    _stdout: RawTerminal<Stdout>,
    size: Size,
}

impl Terminal {
    pub fn default() -> Result<Self, io::Error> {
        let size = termion::terminal_size()?;
        let stdout = io::stdout().into_raw_mode()?;
        Ok(Self {
            _stdout: stdout,
            size: Size {
                width: size.0,
                height: size.1,
            },
        })
    }

    pub fn flush() -> Result<(), io::Error> {
        io::stdout().flush()
    }

    pub fn clear_screen() {
        print!("{}", termion::clear::All);
    }

    pub fn clear_current_line() {
        print!("{}", termion::clear::CurrentLine);
    }

    pub fn cursor_hide() {
        print!("{}", termion::cursor::Hide);
    }

    pub fn cursor_show() {
        print!("{}", termion::cursor::Show);
    }

    pub fn cursor_position(pos: &Position) {
        // The cursor position in the terminal is 1-based.
        let x = u16::try_from(pos.x.saturating_add(1)).unwrap_or(u16::MAX);
        let y = u16::try_from(pos.y.saturating_add(1)).unwrap_or(u16::MAX);
        print!("{}", termion::cursor::Goto(x, y));
    }

    pub fn read_key() -> Result<Key, io::Error> {
        loop {
            if let Some(key) = io::stdin().lock().keys().next() {
                return key;
            }
        }
    }

    pub fn size(&self) -> &Size {
        &self.size
    }
}
