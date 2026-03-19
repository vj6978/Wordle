use crate::{Color, GameTile};
use colored::Colorize;
use std::fmt::Display;

pub fn show_game_board(game_state: &[Vec<GameTile>]) {
    println!();
    for row in game_state {
        for tile in row {
            let colored_char = match tile.color {
                Color::Green => tile.letter.to_string().bright_green(),
                Color::Red => tile.letter.to_string().bright_red(),
                Color::Yellow => tile.letter.to_string().bright_yellow(),
                Color::Purple => tile.letter.to_string().bright_purple(),
            };
            print!("{} ", colored_char);
        }
        println!();
    }
}

pub fn show_welcome_banner() {
    println!(
        "{}",
        r#"
  --- WORDLE INSTRUCTIONS ---
  - Guess the word in six tries.
  - Each guess must be a five-letter word.
  - Alphabets can occur more than once.
  - After each guess, the color of the tiles will change to
    show how close your guess was to the word.

  GREEN: Letter is in the word and in the correct spot.
  YELLOW: Letter is in the word but in the wrong spot.
  RED: Letter is not in the word in any spot.
    "#
    );
}
