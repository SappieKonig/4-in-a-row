use std::time::{Duration, Instant};
use rayon::prelude::*;
use crate::games::connect4::bitboard::BitBoard;
use super::node::{Node, RootNode};

pub struct MCTS {
    exploration_constant: f32,
    num_threads: usize,
    time_limit: Duration,
    n_simulations: u32,
}

impl MCTS {
    pub fn new(exploration_constant: f32, num_threads: usize, time_limit_ms: u64, n_simulations: u32) -> Self {
        Self {
            exploration_constant,
            num_threads,
            time_limit: Duration::from_millis(time_limit_ms),
            n_simulations,
        }
    }

    pub fn search(&self, board: BitBoard, player_number: u8) -> usize {
        let mut root = RootNode::new(board);
        let start_time = Instant::now();

        // Create thread-local search trees and run them in parallel
        let thread_results: Vec<Node> = (0..self.num_threads)
            .into_par_iter()
            .map(|_| {
                let mut thread_root = root.create_thread_tree();
                while start_time.elapsed() < self.time_limit {
                    self.run_iteration(&mut thread_root, player_number);
                }
                thread_root
            })
            .collect();

        // Merge results back to root
        for thread_root in thread_results {
            root.merge_thread_results(&thread_root);
        }

        println!("Root visits: {}", root.get_total_visits());

        root.get_best_move()
    }

    fn run_iteration(&self, root: &mut Node, player_number: u8) {
        let mut current = &mut *root;
        let mut path_indices = Vec::new();
        
        // Selection
        while !current.is_terminal() && current.is_fully_expanded() {
            if current.children.is_empty() {
                break;
            }
            
            let child_idx = current.get_best_child_index(self.exploration_constant);
                
            path_indices.push(child_idx);
            current = &mut current.children[child_idx];
        }
    
        let result = if current.is_terminal() {
            current.result.unwrap() * self.n_simulations as i32
        } else if !current.is_fully_expanded() {
            let new_node = current.expand();
            if new_node.is_terminal() {
                new_node.result.unwrap() * self.n_simulations as i32
            } else {
                new_node.simulate(player_number, self.n_simulations)
            }
        } else {
            current.simulate(player_number, self.n_simulations)
        };
    
        // Backpropagation
        let mut current = root;
        current.update(result, self.n_simulations);
        
        for &idx in &path_indices {
            current = &mut current.children[idx];
            current.update(result, self.n_simulations);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mcts_finds_winning_moves() {
        let mcts = MCTS::new(1.414, 4, 1000);
        
        // Test horizontal win
        let mut board = BitBoard::new();
        board.make_move(0); // Player 1
        board.make_move(6); // Player 2
        board.make_move(1); // Player 1
        board.make_move(6); // Player 2
        board.make_move(2); // Player 1
        board.make_move(6); // Player 2
        
        let best_move = mcts.search(board, 1);
        assert_eq!(best_move, 3, "Failed to find horizontal winning move");

        // Test vertical win
        let mut board = BitBoard::new();
        board.make_move(0); // Player 1
        board.make_move(1); // Player 2
        board.make_move(0); // Player 1
        board.make_move(1); // Player 2
        board.make_move(0); // Player 1
        board.make_move(1); // Player 2
        
        let best_move = mcts.search(board, 1);
        assert_eq!(best_move, 0, "Failed to find vertical winning move");

        // Test diagonal win
        let mut board = BitBoard::new();
        board.make_move(0); // Player 1
        board.make_move(1); // Player 2
        board.make_move(1); // Player 1
        board.make_move(2); // Player 2
        board.make_move(2); // Player 1
        board.make_move(3); // Player 2
        board.make_move(2); // Player 1
        board.make_move(3); // Player 2
        board.make_move(3); // Player 1
        board.make_move(6); // Player 2
        
        let best_move = mcts.search(board, 1);
        assert_eq!(best_move, 3, "Failed to find diagonal winning move");
    }

    #[test]
    fn test_mcts_blocks_opponent_win() {
        let mcts = MCTS::new(1.414, 4, 1000);
        let mut board = BitBoard::new();
        board.make_move(5); // Player 1
        board.make_move(0); // Player 2
        board.make_move(6); // Player 1
        board.make_move(0); // Player 2
        board.make_move(6); // Player 1
        board.make_move(0); // Player 2
        
        let best_move = mcts.search(board, 1);
        assert_eq!(best_move, 0, "Failed to find blocking move");
    }
}