use crate::board::Board;
use crate::definitions::{Cell, GameResult, Player};

pub struct Game {
    board: Board,
    current_turn: Player,
}

impl Game {
    pub fn new() -> Self {
        Game {
            board: Board::new(),
            current_turn: Player::X,
        }
    }

    pub fn make_move(&mut self, player: Player, row: usize, col: usize) {
        match player {
            Player::X => self.board.set(Cell::X, row, col),
            Player::O => self.board.set(Cell::O, row, col),
        }

        self.current_turn = self.current_turn.opposite();
    }

    pub fn board(&self) -> Board {
        self.board
    }

    pub fn current_player(&self) -> Player {
        self.current_turn
    }

    pub fn print_board(&self) {
        println!("{}", self.board);
    }

    pub fn over(&self) -> bool {
        // check that the board is not full and no one has won yet
        !self.board.is_full() && self.board.winner().is_none()
    }

    pub fn result(&self) -> Option<GameResult> {
        if let Some(player) = self.board.winner() {
            Some(GameResult::Win(player))
        } else if self.board.is_full() {
            Some(GameResult::Draw)
        } else {
            None
        }
    }
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}
