use std::io::{Write, stdout};

use crossterm::{
    cursor::{Hide, Show},
    event::{self, Event, KeyCode},
    execute,
    style::{ContentStyle, Stylize},
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

    let mut test_chars = Vec::with_capacity(total_chars);
    for word in list {
        for ch in word.chars() {
            test_chars.push(ch);
        }

        // Add spaces between words
        test_chars.push(' ');
    }

    // Inputted chars
    let mut input = Vec::with_capacity(total_chars);

    display_test(&[], &test_chars);

    enable_raw_mode().unwrap();
    execute!(stdout(), Hide).unwrap();
    loop {
        if let Ok(Event::Key(k)) = event::read() {
            if k.code.is_esc() {
                disable_raw_mode().unwrap();
                execute!(stdout(), Show).unwrap();
                return;
            }

            if k.code == KeyCode::Backspace {
                input.pop();
            } else if let KeyCode::Char(ch) = k.code {
                input.push(ch);
            }
            display_test(&input, &test_chars);
        }
    }
}

fn display_test(input: &[char], expected: &[char]) {
    let pos = input.len();
    for (i, ch) in expected.iter().enumerate() {
        let mut style = ContentStyle::new();

        if i < pos {
            // Red for incorrect input
            if ch != &input[i] {
                style = style.red()
            }
        } else if i == pos {
            // Highlight the char under the cursor
            style = style.dim().underlined();
        } else {
            // Dim chars not typed yet
            style = style.dim();
        }

        print!("{}", style.apply(ch));
    }
    print!("\r");
    stdout().flush().unwrap();
}
