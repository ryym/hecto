use std::io::{self, Write};
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
        print!("{}{}", termion::clear::All, termion::cursor::Goto(1, 1));
        if self.should_quit {
            println!("Goodbye.\r");
        } else {
            self.draw_rows();
            print!("{}", termion::cursor::Goto(1, 1));
        }
        io::stdout().flush()
    }

    fn draw_rows(&self) {
        for _ in 0..24 {
            println!("~\r");
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

fn die(err: &io::Error) {
    print!("{}", termion::clear::All);
    panic!("{}", err);
}

fn read_key() -> Result<Key, io::Error> {
    loop {
        if let Some(key) = io::stdin().lock().keys().next() {
            return key;
        }
    }
}