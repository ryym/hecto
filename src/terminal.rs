use crate::Position;
use std::io::{self, Stdout, Write};
use termion::{
    color,
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
                height: size.1.saturating_sub(2),
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

    pub fn set_bg_color(color: color::Rgb) {
        print!("{}", color::Bg(color));
    }

    pub fn reset_bg_color() {
        print!("{}", color::Bg(color::Reset));
    }

    pub fn set_fg_color(color: color::Rgb) {
        print!("{}", color::Fg(color));
    }

    pub fn reset_fg_color() {
        print!("{}", color::Fg(color::Reset));
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
