use std::io::{Read, Write, stdin, stdout};

use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

fn main() {
    enable_raw_mode().unwrap();
    for b in stdin().bytes() {
        let b = b.unwrap();
        let c = b as char;

        print!("{}\r", c);
        stdout().flush().unwrap();

        if c == 'q' {
            disable_raw_mode().unwrap();
            break;
        }
    }
}
