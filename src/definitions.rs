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
            Cell::X => write!(f, "X"),
            Cell::O => write!(f, "O"),
            Cell::None => write!(f, ""),
        }
    }
}

#[derive(Clone, Copy)]
pub enum Player {
    X,
    O,
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Player::X => write!(f, "X"),
            Player::O => write!(f, "O"),
        }
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

pub fn opposite(player: Player) -> Player {
    match player {
        Player::X => Player::O,
        Player::O => Player::X,
    }
}
