# MasterMind

Mastermind used to be a huge game in a lot of video games I played in my childhood. Many of my college classes even featured it as a learning project for something to learn simplistics of certain languages for.

I wanted to re create it again using Rust as a way to get used to some of the new packages in Rust.

For people who might not know what Mastermind the game is, some have called it 'Bagels' and it does have some similarities to Wordle.

## Code

Code is linted and formatted with enforcement using [pre-commit](https://pre-commit.com/). The main hooks are just used from [https://github.com/doublify/pre-commit-rust](https://github.com/doublify/pre-commit-rust) which are wrappers around `cargo fmt` and `cargo clippy`.

## How to Play

The game revolves around guessing a specific set of characters with limited guesses. The engine responds with 'Pico', 'Fermi' or 'Bagels' to represent the accuracy of a player's guess against the answer the machine holds. Other versions of this game will use colored balls.

In this version those words hold the following meaning:

- Pico: character matches but not in the right position
- Fermi: character matches and is in the right position
- Bagels: no character matches anywhere

For simplicity sake, the first pass will only use the character set of ASCII digits. But the length of the number to guess and the number of guesses will be customizable.
