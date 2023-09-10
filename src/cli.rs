use std::io::Write;

/// Module contains logic around accepting parameters from CLI.
use clap::{error::ErrorKind, Args, CommandFactory, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct ProgramParams {
    #[command(subcommand)]
    pub game_type: GameParams,
}

#[derive(Subcommand, Debug)]
pub enum GameParams {
    CLI(BaseGameArgs),
}

#[derive(Args, Debug, Copy, Clone)]
pub struct BaseGameArgs {
    #[arg(short, long, help = "Get game parameters interactively")]
    pub interactive: bool,
    /// How many guesses until player loses game
    #[arg(short, long, value_name = "NUM_GUESS", default_value_t = 10)]
    pub guess_max: u16,
    /// Length of item to guess
    #[arg(short, long, value_name = "ANSWER_LEN", default_value_t = 3)]
    pub length_answer: u8,
    #[arg(short, long, value_name = "MAX_LETTER", default_value_t = 'J')]
    pub max_letter: char,
    /// Turn debugging information on
    #[arg(short, action = clap::ArgAction::Count, help = "How verbose to be, -v quieter than -vv. Max of 4")]
    pub verbose: u8,
    /// Game generation seed
    #[arg(
        short,
        long,
        help = "Seed value for random generator",
        value_name = "SEED"
    )]
    pub seed_val: Option<u64>,
}

impl BaseGameArgs {
    #[must_use]
    pub fn get_max_letter(&self) -> u8 {
        // Previously asserted that it is ASCII uppercase, so can't panic
        self.max_letter
            .try_into()
            .map_or_else(|_| unreachable!(), |x| x)
    }
    #[must_use]
    pub fn get_interactive_args(self) -> Self {
        Self {
            interactive: false,
            guess_max: Self::interact_guess_max(self.guess_max),
            length_answer: Self::interact_answer_length(self.length_answer),
            max_letter: Self::interact_max_letter(self.max_letter),
            verbose: self.verbose,
            seed_val: Self::interact_seed_val(self.seed_val),
        }
    }
    fn ask_receiv_arg(prev_val: &str, arg_name: &str) -> String {
        let mut line = String::new();
        print!("Value for {arg_name} [{prev_val}]: ");
        std::io::stdout().flush().unwrap();

        std::io::stdin().read_line(&mut line).unwrap();
        line
    }
    fn interact_guess_max(prev_max: u16) -> u16 {
        loop {
            let response = Self::ask_receiv_arg(&prev_max.to_string(), "Max Number of Guesses");
            if response.len() == 1 {
                // length 1 means just the newline char
                // meaning accept the current value
                return prev_max;
            }
            match response.trim().parse::<u16>() {
                Ok(x) => {
                    if x > 0 {
                        return x;
                    }
                }
                Err(_) => println!("Max number of guesses must be a positive integer."),
            }
        }
    }
    fn interact_answer_length(prev_len: u8) -> u8 {
        loop {
            let response = Self::ask_receiv_arg(&prev_len.to_string(), "Length of Answer");
            if response.len() == 1 {
                // length 1 means just the newline char
                // meaning accept the current value
                return prev_len;
            }
            match response.trim().parse::<u8>() {
                Ok(x) => {
                    if (0..=26).contains(&x) {
                        return x;
                    }
                    println!(
                        "Don't explode your brain, keep the answer length in the range of [0, 26]."
                    );
                }
                Err(_) => println!("Max number of guesses must be a positive integer."),
            }
        }
    }
    fn interact_max_letter(prev_char: char) -> char {
        loop {
            let response =
                Self::ask_receiv_arg(&prev_char.to_string(), "Maximum character in guess space");
            if response.len() == 1 {
                // length 1 means just the newline char
                // meaning accept the current value
                return prev_char;
            }
            match response.trim().chars().next() {
                Some(c) => { if c.is_ascii_uppercase() { return c;} println!("Only accepting ASCII uppercase for guess space");},
                None => println!("Try pressing 'Enter' instead to accept the current character {prev_char} instead."),
            };
        }
    }
    fn interact_seed_val(prev_seed: Option<u64>) -> Option<u64> {
        loop {
            let response = Self::ask_receiv_arg(
                &prev_seed.map_or_else(|| String::from("Randomized"), |x| x.to_string()),
                "Seed Value for Random Generator",
            );
            if response.len() == 1 {
                // length 1 means just the newline char
                // meaning accept the current value
                return prev_seed;
            }
            match response.trim().parse::<u64>() {
                Ok(x) => {
                    return Some(x);
                }
                Err(_) => {
                    println!(
                        "Seed value must be a positive integer, sorry to the negative lovers."
                    );
                }
            };
        }
    }
}

#[must_use]
pub fn get_args() -> GameParams {
    let mut params = ProgramParams::parse();
    match &params.game_type {
        GameParams::CLI(args) => {
            let new_args = if args.interactive {
                args.get_interactive_args()
            } else {
                *args
            };
            if !new_args.max_letter.is_ascii_uppercase() {
                let mut cmd = ProgramParams::command();
                cmd.error(
                    ErrorKind::ArgumentConflict,
                    "Maximum letter to use for Mastermind Code must be ASCII Uppercase.",
                )
                .exit()
            }
            params.game_type = GameParams::CLI(new_args);
        }
    }
    params.game_type
}
