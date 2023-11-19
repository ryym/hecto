use std::io;
use termion::{event::Key, input::TermRead, raw::IntoRawMode};

pub(crate) struct Editor {}

impl Editor {
    pub fn default() -> Self {
        Editor {}
    }

    pub fn run(&self) {
        let _stdout = io::stdout().into_raw_mode().unwrap();

        for key in io::stdin().keys() {
            match key {
                Ok(key) => match key {
                    Key::Char(c) => {
                        if c.is_control() {
                            println!("{:?}\r", c as u8)
                        } else {
                            println!("{:?} ({})\r", c as u8, c)
                        }
                    }
                    Key::Ctrl('q') => break,
                    _ => println!("{:?}\r", key),
                },
                Err(e) => panic!("{}", e),
            }
        }
    }
}
