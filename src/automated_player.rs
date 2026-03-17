use crate::board::Board;
use crate::definitions::{Cell, Player, opposite};
use std::cmp;

pub struct AutomatedPlayer {
    player: Player, // am I X or O?
}

impl AutomatedPlayer {
    pub fn new(self_player: Player) -> Self {
        AutomatedPlayer {
            player: self_player,
        }
    }

    fn evaluate(board: Board, player: Player, depth: usize) -> isize {
        match board.winner() {
            Some(winner) => {
                // defining it like this to allow for easy modification in the future
                if winner == player {
                    10 - (depth as isize)
                } else {
                    -10 + (depth as isize)
                }
            }
            // None means no one won, i.e. a draw.
            None => 0,
        }
    }

    fn negamax_impl(&self, board: Board, player: Player, depth: usize) -> isize {
        if board.is_full() || board.winner().is_some() {
            return AutomatedPlayer::evaluate(board, player, depth);
        }

        let mut best_score = isize::MIN;

        let cell_type = match player {
            Player::X => Cell::X,
            Player::O => Cell::O,
        };

        for (row, col) in board.empty_squares() {
            let mut new_board = board;
            new_board.set(cell_type, row, col);

            let score = -self.negamax_impl(new_board, opposite(player), depth + 1);

            best_score = cmp::max(best_score, score);
        }

        best_score
    }

    fn negamax(&self, board: Board, player: Player) -> isize {
        self.negamax_impl(board, player, 0)
    }

    pub fn choose_move(&self, board: &Board) -> (usize, usize) {
        let empty_squares = board.empty_squares();

        let mut best_score = isize::MIN;
        let mut best_move = (0, 0);

        let cell_type = match self.player {
            Player::X => Cell::X,
            Player::O => Cell::O,
        };

        for (row, col) in empty_squares {
            let mut wip_board = *board;
            wip_board.set(cell_type, row, col);

            let score = -self.negamax(wip_board, opposite(self.player));

            if score > best_score {
                best_score = score;
                best_move = (row, col);
            }
        }

        best_move
    }
}
