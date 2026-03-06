struct Board {
    board: [[Cell; 3]; 3],
}

impl Board {
    fn new() -> Self {
        Board {
            board: [[Cell::None; 3]; 3],
        }
    }
}
