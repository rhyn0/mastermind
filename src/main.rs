pub mod game;

fn main() {
    let game = game::GameState::default().start_game();
    let result = game.compare_answer("123");
    println!("{:?}", game.get_answer());
    println!("{result:?}");
}
