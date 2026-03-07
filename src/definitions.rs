#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Cell {
    X,
    O,
    None,
}

#[derive(Clone, Copy)]
pub enum Player {
    X,
    O,
}

pub enum PlayerType {
    Human,
    AI,
}

// #[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
