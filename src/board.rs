use crate::definitions::Cell;
use crate::definitions::Player;
use std::fmt;

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
            board: [[Cell::default(); 3]; 3],
        }
    }

    pub fn set(&mut self, cell: Cell, row: usize, col: usize) {
        self.board[row][col] = cell;
    }

    pub fn is_full(&self) -> bool {
        self.board.iter().flatten().all(|&cell| cell.is_empty())
    }

    pub fn empty_squares(&self) -> Vec<(usize, usize)> {
        let mut squares = Vec::new();

        for r in 0..3 {
            for c in 0..3 {
                if self.board[r][c].is_empty() {
                    squares.push((r, c));
                }
            }
        }

        squares
    }

    pub fn winner(&self) -> Option<Player> {
        Self::POSSIBLE_WINS.iter().find_map(|line| {
            let [(r1, c1), (r2, c2), (r3, c3)] = *line;

            let a = self.board[r1][c1];
            let b = self.board[r2][c2];
            let c = self.board[r3][c3];

            if a == b && b == c { a.player() } else { None }
        })
    }
}

impl Default for Board {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for Board {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        // This formatting is non-standard but helps readability. This rustfmt annotation
        // lets us disable the formatter for this one particular statement.
        #[rustfmt::skip]
        let [[a, b, c],
             [d, e, f],
             [g, h, i]] = self.board;

        writeln!(fmt, " {} │ {} │ {}", a, b, c)?;
        writeln!(fmt, "───┼───┼───")?;
        writeln!(fmt, " {} │ {} │ {}", d, e, f)?;
        writeln!(fmt, "───┼───┼───")?;
        writeln!(fmt, " {} │ {} │ {}", g, h, i)?;

        Ok(())
    }
}
