use std::io::stdout;

use crossterm::{
    cursor::Show,
    execute,
    terminal::{Clear, ClearType, disable_raw_mode},
};

use crate::{tests::start_test, words::generate_word_list};

mod tests;
mod words;

fn main() {
    loop {
        execute!(stdout(), Clear(ClearType::CurrentLine)).unwrap();
        let list = generate_word_list(10);
        if !start_test(list) {
            break;
        }
    }
    println!();
    disable_raw_mode().unwrap();
    execute!(stdout(), Show).unwrap();
}
