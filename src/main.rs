use std::io::{Write, stdout};

use crossterm::{
    cursor::{Hide, Show},
    event::{self, Event, KeyCode},
    execute,
    style::Stylize,
    terminal::{disable_raw_mode, enable_raw_mode},
};

use crate::words::generate_word_list;

mod words;

fn main() {
    let list = generate_word_list(10);
    start_test(list);
}

fn start_test(list: Vec<&str>) {
    // Split the list into chars
    let total_chars = list.join(" ").len();

    let mut chars = Vec::with_capacity(total_chars);
    for word in list {
        for ch in word.chars() {
            chars.push(ch);
        }

        // Add spaces between words
        chars.push(' ');
    }

    // Position of cursor
    let mut pos = 0;

    write_chars(&chars, pos);

    enable_raw_mode().unwrap();
    execute!(stdout(), Hide).unwrap();
    loop {
        if let Ok(Event::Key(k)) = event::read() {
            if k.code.is_esc() {
                disable_raw_mode().unwrap();
                execute!(stdout(), Show).unwrap();
                return;
            }

            // print!("{}\r", c);
            // stdout().flush().unwrap();
            if k.code == KeyCode::Backspace {
                pos -= 1;
            } else if let KeyCode::Char(_) = k.code {
                pos += 1;
            }
            write_chars(&chars, pos);
        }
    }
}

fn write_chars(chars: &[char], pos: usize) {
    for (i, ch) in chars.iter().enumerate() {
        // Highlight the char under the cursor
        if i == pos {
            print!("{}", (*ch).underlined());
        } else {
            print!("{}", ch);
        }
    }
    print!("\r");
    stdout().flush().unwrap();
}
