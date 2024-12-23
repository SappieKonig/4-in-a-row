use crate::config::{GRID_COLS, GRID_ROWS};
use crate::player::Player;

#[derive(Clone)]
pub struct Board {
    pub player_numbers: [[u8; GRID_COLS]; GRID_ROWS],
}

impl Board {
    pub fn new() -> Self {
        Self {
            cells: [[0; GRID_COLS]; GRID_ROWS],
        }
    }

    pub fn get_player_number(&self, row: usize, col: usize) -> u8 {
        self.cells[row][col]
    }

    pub fn can_play_column(&self, col: usize) -> bool {
        col < GRID_COLS && self.cells[0][col] == 0
    }

    pub fn make_move(&mut self, col: usize, player_number: u8) -> Option<(usize, usize)> {
        if !self.can_play_column(col) {
            return None;
        }

        // Find the lowest empty position in the column
        for row in (0..GRID_ROWS).rev() {
            if self.cells[row][col] == 0 {
                self.cells[row][col] = player_number;
                return Some((row, col));
            }
        }
        None
    }

    pub fn check_win(&self, row: usize, col: usize) -> bool {
        let player = self.cells[row][col];
        if player == 0 {
            return false;
        }

        // Check horizontal
        let mut count = 0;
        for c in 0..GRID_COLS {
            if self.cells[row][c] == player {
                count += 1;
                if count == 4 {
                    return true;
                }
            } else {
                count = 0;
            }
        }

        // Check vertical
        count = 0;
        for r in 0..GRID_ROWS {
            if self.cells[r][col] == player {
                count += 1;
                if count == 4 {
                    return true;
                }
            } else {
                count = 0;
            }
        }

        // Check diagonal (down-right)
        for r in 0..GRID_ROWS-3 {
            for c in 0..GRID_COLS-3 {
                if self.cells[r][c] == player &&
                   self.cells[r+1][c+1] == player &&
                   self.cells[r+2][c+2] == player &&
                   self.cells[r+3][c+3] == player {
                    return true;
                }
            }
        }

        // Check diagonal (up-right)
        for r in 3..GRID_ROWS {
            for c in 0..GRID_COLS-3 {
                if self.cells[r][c] == player &&
                   self.cells[r-1][c+1] == player &&
                   self.cells[r-2][c+2] == player &&
                   self.cells[r-3][c+3] == player {
                    return true;
                }
            }
        }

        false
    }

    pub fn is_full(&self) -> bool {
        for col in 0..GRID_COLS {
            if self.can_play_column(col) {
                return false;
            }
        }
        true
    }
}