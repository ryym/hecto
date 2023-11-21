use std::io;
use termion::{event::Key, input::TermRead, raw::IntoRawMode};

pub(crate) struct Editor {
    should_quit: bool,
}

impl Editor {
    pub fn default() -> Self {
        Self { should_quit: false }
    }

    pub fn run(&mut self) {
        let _stdout = io::stdout().into_raw_mode().unwrap();

        loop {
            if let Err(err) = self.process_keypress() {
                panic!("{}", err);
            }
            if self.should_quit {
                break;
            }
        }
    }

    fn process_keypress(&mut self) -> Result<(), io::Error> {
        let pressed_key = read_key()?;
        match pressed_key {
            Key::Ctrl('q') => self.should_quit = true,
            _ => {}
        }
        Ok(())
    }
}

fn read_key() -> Result<Key, io::Error> {
    loop {
        if let Some(key) = io::stdin().lock().keys().next() {
            return key;
        }
    }
}
