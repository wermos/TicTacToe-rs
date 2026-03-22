use crate::board::Board;
use crate::definitions::Player;
use std::cmp;
use std::time::Instant;

pub struct AutomatedPlayer {
    player: Player, // am I X or O?
}

struct SearchStats {
    pub nodes: usize,
}

impl AutomatedPlayer {
    const INF: isize = 100;

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

    fn negamax_impl(
        &self,
        board: Board,
        player: Player,
        depth: usize,
        mut alpha: isize,
        beta: isize,
        stats: &mut SearchStats,
    ) -> isize {
        stats.nodes += 1;

        if board.is_full() || board.winner().is_some() {
            return AutomatedPlayer::evaluate(board, player, depth);
        }

        let mut best_score = -AutomatedPlayer::INF;

        for (row, col) in board.empty_squares() {
            let mut new_board = board;
            new_board.set(player.cell(), row, col);

            let score = -self.negamax_impl(
                new_board,
                player.opposite(),
                depth + 1,
                -beta,
                -alpha,
                stats,
            );

            best_score = cmp::max(best_score, score);
            alpha = cmp::max(alpha, score);

            if alpha >= beta {
                break;
            }
        }

        best_score
    }

    fn negamax(&self, board: Board, player: Player, stats: &mut SearchStats) -> isize {
        self.negamax_impl(
            board,
            player,
            0,
            -AutomatedPlayer::INF,
            AutomatedPlayer::INF,
            stats,
        )
    }

    pub fn choose_move(&self, board: &Board) -> (usize, usize) {
        let start = Instant::now();

        let mut stats = SearchStats { nodes: 0 };

        let empty_squares = board.empty_squares();

        let mut best_score = isize::MIN;
        let mut best_move = (0, 0);

        for (row, col) in empty_squares {
            let mut wip_board = *board;
            wip_board.set(self.player.cell(), row, col);

            let score = -self.negamax(wip_board, self.player.opposite(), &mut stats);

            if score > best_score {
                best_score = score;
                best_move = (row, col);
            }
        }

        let duration = start.elapsed();
        let seconds = duration.as_secs_f64();

        println!("Nodes searched: {}", stats.nodes);
        println!("Time: {:.3} ms", seconds * 1000.0);
        println!("Nodes/sec: {:.2}M", stats.nodes as f64 / seconds / 1e6);

        best_move
    }
}
