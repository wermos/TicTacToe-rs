struct Game {
    board: Board,
    turn: Turn,
}

impl Game {
    fn new() -> Self {
        Game {
            board: Board::new(),
            turn: Turn::X,
        }
    }
}
