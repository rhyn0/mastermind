/// Module contains logic around accepting parameters from CLI.
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct ProgramParams {
    /// How many guesses until player loses game
    #[arg(short, long, value_name = "NUM_GUESS", default_value_t = 10)]
    pub guess_max: u16,
    /// Length of item to guess
    #[arg(short, long, value_name = "ANSWER_LEN", default_value_t = 3)]
    pub length_answer: u8,
    /// Turn debugging information on
    #[arg(short, action = clap::ArgAction::Count, help = "How verbose to be, -v quieter than -vv. Max of 4")]
    pub verbose: u8,
}
