use crate::definitions::Cell;
use crate::definitions::Player;

#[derive(Clone, Copy)]
pub struct Board {
    board: [[Cell; 3]; 3],
}

impl Board {
    const POSSIBLE_WINS: [[(usize, usize); 3]; 8] = [
        [(0, 0), (0, 1), (0, 2)],
        [(1, 0), (1, 1), (1, 2)],
        [(2, 0), (2, 1), (2, 2)],
        [(0, 0), (1, 0), (2, 0)],
        [(0, 1), (1, 1), (2, 1)],
        [(0, 2), (1, 2), (2, 2)],
        [(0, 0), (1, 1), (2, 2)],
        [(0, 2), (1, 1), (2, 0)],
    ];

    pub fn new() -> Self {
        Board {
            board: [[Cell::None; 3]; 3],
        }
    }

    pub fn set(&mut self, cell: Cell, row: usize, col: usize) {
        self.board[row][col] = cell;
    }

    pub fn is_full(&self) -> bool {
        self.board.iter().flatten().all(|&cell| cell != Cell::None)
    }

    pub fn winner(&self) -> Option<Player> {
        Self::POSSIBLE_WINS.iter().find_map(|line| {
            let [(r1, c1), (r2, c2), (r3, c3)] = *line;

            let a = self.board[r1][c1];
            let b = self.board[r2][c2];
            let c = self.board[r3][c3];

            if a == b && b == c {
                match a {
                    Cell::X => Some(Player::X),
                    Cell::O => Some(Player::O),
                    Cell::None => None,
                }
            } else {
                None
            }
        })
    }
}
