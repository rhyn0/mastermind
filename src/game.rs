// File to hold all logic persisting to a game of Mastermind.
use itertools::Itertools;
use rand::{
    distributions::Alphanumeric,
    prelude::{thread_rng, Distribution},
};
use std::{collections::HashSet, fmt::Debug, io::Write};

const ASCII_A: u8 = 65;
#[derive(Debug, Clone)]
#[allow(clippy::module_name_repetitions)]
pub struct GameState {
    pub max_guesses: u16,
    pub curr_guesses: u16,
    pub guess_length: u8,
    pub letter_max: char,
    answer: String,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            max_guesses: 10,
            curr_guesses: 0,
            guess_length: 3,
            letter_max: 'J',
            answer: String::new(),
        }
    }
}

impl GameState {
    const CORRECT_STRING: &str = "Fermi";
    const ALMOST_STRING: &str = "Pico";
    const FAIL_STRING: &str = "Bagels";
    #[must_use]
    pub fn new_game(max_guesses: u16, guess_length: u8, letter_max: u8) -> Self {
        Self {
            max_guesses,
            guess_length,
            answer: Alphanumeric
                .sample_iter(&mut thread_rng())
                .filter_map(|c| {
                    if (ASCII_A..=letter_max).contains(&c) {
                        Some(char::from(c))
                    } else {
                        None
                    }
                })
                .take(guess_length.into())
                .collect(),
            ..Default::default()
        }
    }
    #[must_use]
    pub fn compare_answer(&mut self, guess: &str) -> Vec<String> {
        // TODO: change logic and probably break into two functions
        // Code for inexact is check all other indices that weren't exact matches
        self.curr_guesses += 1;
        let correct_idx: HashSet<usize> = self
            .answer
            .chars()
            .zip(guess.chars())
            .enumerate()
            .filter_map(|(idx, (c1, c2))| if c1 == c2 { Some(idx) } else { None })
            .collect();
        let semi_correct_all: HashSet<usize> = self
            .answer
            .chars()
            .cartesian_product(guess.char_indices())
            .filter_map(|(c1, (idx, c2))| if c1 == c2 { Some(idx) } else { None })
            .collect::<HashSet<usize>>();
        let semi_correct_idx: HashSet<usize> =
            semi_correct_all.difference(&correct_idx).copied().collect();
        if correct_idx.is_empty() && semi_correct_idx.is_empty() {
            return vec![Self::FAIL_STRING.to_owned()];
        }
        correct_idx
            .iter()
            .map(|_| Self::CORRECT_STRING.to_owned())
            .chain(
                semi_correct_idx
                    .iter()
                    .map(|_| Self::ALMOST_STRING.to_owned()),
            )
            .collect_vec()
    }
    #[must_use]
    pub fn get_answer(&self) -> &str {
        // useful for end game, if player fails
        self.answer.as_str()
    }

    /// Return whether player can make another guess
    #[must_use]
    pub const fn available_turn(&self) -> bool {
        self.curr_guesses < self.max_guesses
    }

    /// Return whether a result is indicative of being the answer
    #[must_use]
    pub fn is_guess_correct(&self, result: &[String]) -> bool {
        if result.len() > self.guess_length.into() {
            return false;
        }
        result.iter().filter(|&r| r == Self::CORRECT_STRING).count() == self.guess_length.into()
    }
}

/// # Panics
///
/// Panics when unable to write to stdout with print!
#[must_use]
pub fn get_cli_guess(game: &GameState) -> String {
    print!(
        "Enter in guess - valid characters [A-{}]: ",
        game.letter_max
    );
    std::io::stdout().flush().unwrap();
    let mut guess_string = String::new();
    while let Ok(n) = std::io::stdin().read_line(&mut guess_string) {
        // since it is line, there is newline char at end
        if n - 1 == game.guess_length.into() {
            break;
        }
        if n > game.guess_length.into() {
            println!("Your guess is too long, needs to be {}", game.guess_length);
        } else {
            println!("Your guess is too short, needs to be {}", game.guess_length);
        }
        guess_string.clear();
        print!(
            "Enter in guess - valid characters [A-{}]: ",
            game.letter_max
        );
        std::io::stdout().flush().unwrap();
    }
    guess_string.trim().to_owned()
}
