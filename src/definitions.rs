use std::fmt;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Cell {
    X,
    O,
    None,
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Cell::X => write!(f, "X")?,
            Cell::O => write!(f, "O")?,
            Cell::None => write!(f, " ")?,
        }
        Ok(())
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Player {
    X,
    O,
}

impl Player {
    pub fn opposite(self) -> Player {
        match self {
            Player::X => Player::O,
            Player::O => Player::X,
        }
    }

    pub fn cell(self) -> Cell {
        match self {
            Player::X => Cell::X,
            Player::O => Cell::O,
        }
    }
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Player::X => write!(f, "X")?,
            Player::O => write!(f, "O")?,
        }
        Ok(())
    }
}

pub enum PlayerType {
    Human,
    AI,
}

pub enum GameResult {
    Win(Player),
    Draw,
}
