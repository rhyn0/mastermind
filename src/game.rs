// File to hold all logic persisting to a game of Mastermind.
use itertools::Itertools;
use rand::{
    distributions::Alphanumeric,
    prelude::{thread_rng, Distribution},
};
use std::collections::HashSet;

const ASCII_ZERO: u8 = 48;
const ASCII_NINE: u8 = 57;
#[derive(Debug)]
#[allow(clippy::module_name_repetitions)]
pub struct GameState {
    pub max_guesses: u16,
    pub curr_guesses: u16,
    pub guess_length: u8,
    answer: String,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            max_guesses: 10,
            curr_guesses: 0,
            guess_length: 3,
            answer: String::new(),
        }
    }
}

impl GameState {
    const CORRECT_STRING: &str = "Fermi";
    const ALMOST_STRING: &str = "Pico";
    const FAIL_STRING: &str = "Bagels";
    #[must_use]
    pub fn new_game(max_guesses: u16, guess_length: u8) -> Self {
        Self {
            max_guesses,
            guess_length,
            ..Default::default()
        }
    }
    #[must_use]
    /// # Panics
    ///
    /// Will panic if i64 to usize conversion fails
    pub fn start_game(&self) -> Self {
        Self {
            answer: Alphanumeric
                .sample_iter(&mut thread_rng())
                .filter_map(|c| {
                    if (ASCII_ZERO..=ASCII_NINE).contains(&c) {
                        Some(char::from(c))
                    } else {
                        None
                    }
                })
                .take(self.guess_length.try_into().unwrap())
                .collect(),
            ..Default::default()
        }
    }
    #[must_use]
    pub fn compare_answer(&self, guess: &str) -> Vec<String> {
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
}
