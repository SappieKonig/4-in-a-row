use rand::Rng;
use crate::games::connect4::board::Board;
use crate::games::connect4::bitboard::BitBoard;
use crate::mcts::mcts::MCTS;

pub trait Bot {
    fn make_move(&self, board: &Board, player_number: u8) -> Option<usize>;
}

pub struct RandomBot;

impl Bot for RandomBot {
    fn make_move(&self, board: &Board, _player_number: u8) -> Option<usize> {
        let mut valid_cols = Vec::new();
        for col in 0..crate::config::GRID_COLS {
            if board.can_play_column(col) {
                valid_cols.push(col);
            }
        }

        if valid_cols.is_empty() {
            None
        } else {
            Some(valid_cols[rand::thread_rng().gen_range(0..valid_cols.len())])
        }
    }
}

pub struct MctsBot {
    mcts: MCTS,
}

impl MctsBot {
    pub fn new(simulation_time_ms: u64) -> Self {
        Self {
            mcts: MCTS::new(1.414, 4, simulation_time_ms, 10), // Example parameters
        }
    }
}

impl Bot for MctsBot {
    fn make_move(&self, board: &Board, player_number: u8) -> Option<usize> {
        let bitboard = BitBoard::from_board(board, player_number);
        Some(self.mcts.search(bitboard, player_number))
    }
}