use std::fmt;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Cell(Option<Player>);

impl Cell {
    pub(crate) fn is_empty(&self) -> bool {
        self.0.is_none()
    }

    pub(crate) fn player(&self) -> Option<Player> {
        self.0
    }
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Cell(Some(p)) => write!(f, "{}", p)?,
            Cell(None) => write!(f, " ")?,
        }
        Ok(())
    }
}

impl Default for Cell {
    fn default() -> Self {
        Cell(None)
    }
}

impl From<Player> for Cell {
    fn from(player: Player) -> Self {
        Cell(Some(player))
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

pub enum GameOutcome {
    Win(Player),
    Draw,
}
