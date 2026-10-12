use std::io::{Write, stdout};

use crossterm::{
    cursor::Show,
    event::{self, Event},
    execute,
    terminal::{Clear, ClearType, disable_raw_mode},
};

use crate::{tests::start_test, words::generate_word_list};

mod speed;
mod tests;
mod words;

fn main() {
    'typing_tests: loop {
        execute!(stdout(), Clear(ClearType::CurrentLine)).unwrap();
        let list = generate_word_list(10);
        let result = start_test(list);

        match result {
            tests::TypingTestResult::Finished {
                strokes,
                time,
                speed,
                accuracy,
            } => {
                execute!(stdout(), Clear(ClearType::CurrentLine)).unwrap();
                print!(
                    "\r{:.1} {} ({:.1} raw) | {} strokes | {:.1}% | {:.1}s | Press enter for a new test...\r",
                    speed.net,
                    speed.unit,
                    speed.raw,
                    strokes,
                    accuracy,
                    time.as_secs_f32(),
                );
                stdout().flush().unwrap();

                loop {
                    if let Ok(Event::Key(k)) = event::read() {
                        if k.code.is_enter() {
                            break;
                        }
                        if k.code.is_esc() {
                            break 'typing_tests;
                        }
                    }
                }
            }
            tests::TypingTestResult::Restart => continue,
            tests::TypingTestResult::Quit => break,
        }
    }
    println!();
    disable_raw_mode().unwrap();
    execute!(stdout(), Show).unwrap();
}
