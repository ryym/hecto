use std::io::{self, Read};

fn main() {
    println!("Hello, world!");

    for b in io::stdin().bytes() {
        let c = b.unwrap() as char;
        print!("{}", c);
        if c == 'q' {
            break;
        }
    }
}
