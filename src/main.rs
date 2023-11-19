use std::io::{self, Read, Write};
use termion::raw::IntoRawMode;

fn main() {
    let mut stdout = io::stdout().into_raw_mode().unwrap();

    for b in io::stdin().bytes() {
        match b {
            Ok(b) => {
                let c = b as char;
                if c.is_control() {
                    println!("{:?} {:#b} \r", b, b)
                } else {
                    println!("{:?} {:#b} ({})\r", b, b, c)
                }
                stdout.flush().unwrap();
                if b == to_ctrl_byte('q') {
                    break;
                }
            }
            Err(e) => panic!("{}", e),
        }
    }
}

fn to_ctrl_byte(c: char) -> u8 {
    let byte = c as u8;
    byte & 0b001_1111
}
