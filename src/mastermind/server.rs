// Module for playign a web based game of Mastermind
use rocket::{self};

use super::cli::BaseGameArgs;

#[rocket::get("/")]
const fn hello_world_index() -> &'static str {
    "Hello World"
}

#[rocket::main]
async fn server_start() -> Result<(), rocket::Error> {
    #[allow(clippy::no_effect_underscore_binding)]
    rocket::build()
        .mount("/", rocket::routes![hello_world_index])
        .launch()
        .await?;
    Ok(())
}

/// # Panics
/// If error starting Rocket server
pub fn start_rocket(args: &BaseGameArgs) {
    dbg!(args);
    server_start().unwrap();
}
