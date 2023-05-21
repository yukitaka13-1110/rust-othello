mod game;
mod board;
use crate::game::OthelloGame;

fn main() {
    loop {
        let mut game = OthelloGame::new();
        game.configure();
        game.start();
        game.results();
        if !game.continue_or_not() {
            break;
        }
    }
}
