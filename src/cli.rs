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

#[derive(Args, Debug)]
pub struct BaseGameArgs {
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
}

#[must_use]
pub fn get_args() -> GameParams {
    let params = ProgramParams::parse();
    match &params.game_type {
        GameParams::CLI(args) => {
            if !args.max_letter.is_ascii_uppercase() {
                let mut cmd = ProgramParams::command();
                cmd.error(
                    ErrorKind::ArgumentConflict,
                    "Maximum letter to use for Mastermind Code must be ASCII Uppercase.",
                )
                .exit()
            }
        }
    }
    params.game_type
}
