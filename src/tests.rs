use std::io::{Write, stdout};

use crossterm::{
    cursor::Hide,
    event::{self, Event, KeyCode},
    execute,
    style::{ContentStyle, Stylize},
    terminal::enable_raw_mode,
};

/// Returns `true` for a new test and `false` to quit.
pub fn start_test(list: Vec<&str>) -> bool {
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
    // Remove the trailing space
    test_chars.pop();

    // Inputted chars
    let mut input = Vec::with_capacity(total_chars);

    enable_raw_mode().unwrap();
    execute!(stdout(), Hide).unwrap();
    display_test(&[], &test_chars);

    loop {
        if let Ok(Event::Key(k)) = event::read() {
            if k.code.is_esc() {
                return false;
            }

            // New test on enter
            if k.code.is_enter() {
                return true;
            }

            if k.code == KeyCode::Backspace {
                input.pop();
            } else if let KeyCode::Char(ch) = k.code {
                input.push(ch);
            }
            display_test(&input, &test_chars);

            // End test if all the characters have been typed
            // and the last is correct, or more characters than
            // in the test have been typed.
            if (input.len() == test_chars.len() && input.last() == test_chars.last())
                || input.len() > test_chars.len()
            {
                return true;
            }
        }
    }
}

fn display_test(input: &[char], expected: &[char]) {
    print!("\r");

    let pos = input.len();
    for (i, ch) in expected.iter().enumerate() {
        let mut style = ContentStyle::new();

        if i < pos {
            // Red for incorrect input
            if ch != &input[i] {
                style = style.red().underline_red().underlined();
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
