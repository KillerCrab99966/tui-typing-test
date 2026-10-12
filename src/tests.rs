use std::{
    io::{Write, stdout},
    time::{Duration, Instant},
};

use crossterm::{
    cursor::Hide,
    event::{self, Event, KeyCode},
    execute,
    style::{ContentStyle, Stylize},
    terminal::enable_raw_mode,
};

use crate::speed::{TypingSpeedUnit::WordsPerMinute, TypingTestSpeed};

pub enum TypingTestResult {
    Finished {
        strokes: u32,
        time: Duration,
        speed: TypingTestSpeed,
        accuracy: f32,
    },
    Restart,
    Quit,
}

impl TypingTestResult {
    pub fn new(strokes: u32, errors: u32, time: Duration) -> Self {
        let speed = TypingTestSpeed::new(strokes, errors, time, WordsPerMinute);
        Self::Finished {
            strokes,
            time,
            speed,
            accuracy: (strokes - errors) as f32 / strokes as f32 * 100.0,
        }
    }
}

pub fn start_test(list: Vec<&str>) -> TypingTestResult {
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
    let mut strokes: u32 = 0;
    let mut errors: u32 = 0;

    enable_raw_mode().unwrap();
    execute!(stdout(), Hide).unwrap();
    display_test(&[], &test_chars);

    let mut started = false;
    let mut start_time = Instant::now();
    loop {
        if let Ok(Event::Key(k)) = event::read() {
            if k.code.is_esc() {
                return TypingTestResult::Quit;
            }

            // New test on enter
            if k.code.is_enter() {
                return TypingTestResult::Restart;
            }

            if k.code == KeyCode::Backspace {
                input.pop();
            } else if let KeyCode::Char(ch) = k.code {
                // Start the timer
                if !started {
                    started = true;
                    start_time = Instant::now();
                }

                strokes += 1;
                if input.len() < test_chars.len() && ch != test_chars[input.len()] {
                    errors += 1;
                }

                input.push(ch);
            }
            display_test(&input, &test_chars);

            // End test if all the characters have been typed
            // and the last is correct, or more characters than
            // in the test have been typed.
            if (input.len() == test_chars.len() && input.last() == test_chars.last())
                || input.len() > test_chars.len()
            {
                return TypingTestResult::new(strokes, errors, start_time.elapsed());
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
