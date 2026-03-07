use crate::board::Board;
use crate::definitions::Player;
use rand::seq::IteratorRandom;

pub struct AutomatedPlayer {
    player: Player, // am I X or O?
}

impl AutomatedPlayer {
    pub fn new(self_player: Player) -> Self {
        AutomatedPlayer {
            player: self_player,
        }
    }

    pub fn choose_move(&self, board: &Board) -> (usize, usize) {
        let empty_squares = board.empty_squares();

        let (row, col) = empty_squares.iter().choose(&mut rand::rng()).unwrap();

        println!("{:?}", empty_squares);
        println!("Random choice: ({}, {})", *row, *col);

        (*row, *col)
    }
}
