use crate::board::Board;
use crate::definitions::Player;

pub struct AutomatedPlayer {
    player: Player, // am I X or O?
}

impl AutomatedPlayer {
    pub fn new(self_player: Player) -> Self {
        AutomatedPlayer {
            player: self_player,
        }
    }

    pub fn choose_move(&self, board: &Board) -> usize {
        todo!()
    }
}
