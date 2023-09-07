pub mod cli;
pub mod game;

use clap::Parser;
use cli::ProgramParams;
use game::GameState;

fn main() {
    let args = ProgramParams::parse();
    let verbosity = args.verbose;
    dbg!(verbosity);
    let game = GameState::new_game(args.guess_max, args.length_answer).start_game();
    let result = game.compare_answer("123");
    println!("{:?}", game.get_answer());
    println!("{result:?}");
}
