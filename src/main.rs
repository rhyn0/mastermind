pub mod cli;
pub mod game;

use std::io::Write;

use clap::Parser;
use cli::ProgramParams;
use game::GameState;

fn main() {
    let args = ProgramParams::parse();
    play_cli_game(&args);
}

fn play_cli_game(params: &ProgramParams) {
    let mut game = GameState::new_game(params.guess_max, params.length_answer).start_game();
    while game.available_turn() {
        // obtain guess
        let mut guess = String::new();
        let mut stdout_lock = std::io::stdout().lock();
        write!(stdout_lock, "Enter in guess: ").unwrap();
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut guess).unwrap();

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
