pub mod mastermind;

use mastermind::{
    cli::{BaseGameArgs, GameParams},
    game::GameState,
    server::start_rocket,
};

fn main() {
    let params = mastermind::cli::get_game_type();
    match params.game_type {
        GameParams::CLI(_) => play_cli_game(&params.game_type.get_args()),
        GameParams::Server(_) => {
            start_rocket(&params.game_type.get_args());
        }
    };
}

fn play_cli_game(params: &BaseGameArgs) {
    dbg!(params);
    let mut game = GameState::new_game(params);
    while game.available_turn() {
        // obtain guess
        let guess = mastermind::game::get_cli_guess(&game);
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
