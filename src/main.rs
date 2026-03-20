mod display;
mod input_validator;

use crate::input_validator::assert_input_length;
use display::show_game_board;
use display::show_welcome_banner;
use input_validator::assert_input_alphabets_only;
use rand::seq::IndexedRandom;
use random_word::Lang;
use std::collections::HashSet;
use std::io;

type Board = Vec<Vec<GameTile>>;

#[derive(Clone)]
enum Color {
    Green,
    Red,
    Yellow,
    Purple,
}

#[derive(Clone)]
struct GameTile {
    letter: char,
    color: Color,
}

struct Game {
    lives: i32,
    board: Board,
    word_to_guess: String,
    word_to_guess_characters: Vec<char>,
    character_reference_table: HashSet<char>,
}

impl Game {
    fn new(word_to_gues: Option<String>) -> Self {
        let lives = 6;
        let mut rng = rand::rng();
        let words = random_word::all_len(5, Lang::En).expect("Failed to get words");
        let word_to_guess = match word_to_gues {
            Some(word) => word,
            None => words
                .choose(&mut rng)
                .expect("Failed to choose a word")
                .to_uppercase()
        };

        let word_to_guess_characters = word_to_guess.chars().collect();
        let character_reference_table: HashSet<char> = word_to_guess.chars().collect();

        let tile: GameTile = GameTile {
            letter: '_',
            color: Color::Purple,
        };

        let board: Board = vec![vec![tile; 5]; 6];

        Game {
            lives,
            board,
            word_to_guess,
            word_to_guess_characters,
            character_reference_table,
        }
    }

    fn assert_guess(&mut self, index: usize, player_input: &str) -> bool {
        let player_input_characters = player_input.chars();

        if player_input == self.word_to_guess {
            for (position, character) in player_input_characters.enumerate() {
                self.board[index][position] = GameTile {
                    letter: character,
                    color: Color::Green,
                }
            }
            true
        } else {
            for (position, character) in player_input_characters.enumerate() {
                self.board[index][position] =
                    if character == self.word_to_guess_characters[position] {
                        GameTile {
                            letter: character,
                            color: Color::Green,
                        }
                    } else if self.character_reference_table.contains(&character) {
                        GameTile {
                            letter: character,
                            color: Color::Yellow,
                        }
                    } else {
                        GameTile {
                            letter: character,
                            color: Color::Red,
                        }
                    }
            }
            false
        }
    }

    fn start_game_loop(mut self: Game) {
        let lives = 6;

        show_welcome_banner();
        show_game_board(&self.board);

        for index in 0..lives {
            let mut player_input = String::new();

            println!("\nWhat is your guess:");

            loop {
                player_input.clear();

                io::stdin()
                    .read_line(&mut player_input)
                    .expect("Error reading user input!");

                player_input = player_input.trim().to_uppercase();

                if player_input.eq("QUIT") {
                    std::process::exit(0);
                }

                if !assert_input_alphabets_only(&player_input)
                    || !assert_input_length(&player_input)
                {
                    println!("Try again");
                } else {
                    break;
                }
            }

            if self.assert_guess(index, &player_input) {
                println!("Congrats! You won!");
                return;
            }

            show_game_board(&self.board);
        }

        println!("\nBetter luck next time!");
        println!("The word to guess was: {}!", self.word_to_guess);
    }
}

fn main() {
    let new_game = Game::new(Option::None);
    new_game.start_game_loop();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_create_new_game() {
        let word_to_guess = "NORTH";
        let game = Game::new(Option::Some(String::from(word_to_guess)));
        assert_eq!(game.lives, 6);
        assert_eq!(game.board[0].iter().len(), 5);
        assert_eq!(game.word_to_guess, word_to_guess);
    }
}