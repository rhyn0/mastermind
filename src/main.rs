pub mod cli;
pub mod game;

use cli::{BaseGameArgs, GameParams};
use game::GameState;

fn main() {
    let game_args = cli::get_args();
    match game_args {
        GameParams::CLI(cli_args) => play_cli_game(&cli_args),
    };
}

fn play_cli_game(params: &BaseGameArgs) {
    let mut game = GameState::new_game(params);
    while game.available_turn() {
        // obtain guess
        let guess = game::get_cli_guess(&game);
        let guess_result = game.compare_answer(&guess);
        if game.is_guess_correct(&guess_result) {
            println!(
                "Congratulations!! Correct answer was {guess} and you got it in {} tries.",
                &game.curr_guesses
            );
            return;
        }
        for val in guess_result {
            print!("{val} ");
        }
        println!();
    }
    println!(
        "Tough luck, you were unable to guess the answer in {} tries.",
        game.max_guesses
    );
    println!("The answer for this game was {}.", game.get_answer());
}
