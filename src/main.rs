use crate::game::Game;
use crate::input::get_numeric_input;

mod game;
mod input;

fn main() {
    println!("Welcome to the memory game!");

    let size: (usize, usize) = get_game_size();

    let mut game: Game = Game::new(size.0, size.1);
    // game.print_cheat_sheet();
    game.play();
}

fn get_game_size() -> (usize, usize) {
    let mut x;
    let mut y;
    loop {
        x = get_numeric_input("Provide horizontal size for the game:");
        y = get_numeric_input("Provide vertical size for the game:");
        if x * y % 2 != 0 {
            println!("Game must contain even number of tiles, current size is odd, try again!")
        } else {
            break;
        }
    }
    (x, y)
}
