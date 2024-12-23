use std::time::{Duration, Instant};
use std::thread;
use rayon::prelude::*;
use crate::bitboard::BitBoard;
use super::node::{Node, RootNode};

pub struct MCTS {
    exploration_constant: f32,
    num_threads: usize,
    time_limit: Duration,
}

impl MCTS {
    pub fn new(exploration_constant: f32, num_threads: usize, time_limit_ms: u64) -> Self {
        Self {
            exploration_constant,
            num_threads,
            time_limit: Duration::from_millis(time_limit_ms),
        }
    }

    pub fn search(&self, board: BitBoard, player_number: u8) -> usize {
        let root = RootNode::new(board);
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

        // Select best move based on number of visits
        root.children
            .iter()
            .enumerate()
            .max_by_key(|(_, child)| child.visits)
            .map(|(i, _)| i)
            .unwrap_or(0)
    }

    fn run_iteration(&self, node: &mut Node, player_number: u8) {
        // Phase 1: Selection
        let mut selected = node;
        let mut path = vec![selected];

        // Select a path through the tree
        while selected.is_fully_expanded() && !selected.is_terminal() {
            selected = selected.children
                .iter_mut()
                .max_by(|a, b| {
                    a.ucb1(node.visits, self.exploration_constant)
                        .partial_cmp(&b.ucb1(node.visits, self.exploration_constant))
                        .unwrap()
                })
                .unwrap();
            path.push(selected);
        }

        // Phase 2: Expansion
        if !selected.is_terminal() {
            if let Some(new_node) = selected.expand() {
                selected = new_node;
                path.push(selected);
            }
        }

        // Phase 3: Simulation (Rollout)
        let result = self.simulate(selected.board, player_number);

        // Phase 4: Backpropagation
        for node in path {
            node.update(result);
        }
    }

    fn simulate(&self, mut board: BitBoard, player_number: u8) -> i32 {
        let mut current_player = player_number;
        let opponent = if player_number == 1 { 2 } else { 1 };

        while !board.is_full() {
            let valid_moves = board.get_valid_moves();
            if valid_moves.is_empty() {
                break;
            }

            let mov = valid_moves[fastrand::usize(..valid_moves.len())];
            if board.is_winning_move(mov) {
                return if current_player == player_number { 1 } else { -1 };
            }

            board.make_move(mov);
            current_player = if current_player == 1 { 2 } else { 1 };
        }

        0 // Draw
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mcts_basic() {
        let mcts = MCTS::new(1.414, 4, 1000); // 1 second, 4 threads
        let board = BitBoard::new();
        let best_move = mcts.search(board, 1);
        assert!(best_move < 7); // Valid column number
    }

    #[test]
    fn test_mcts_winning_move() {
        let mut board = BitBoard::new();

        // Set up a winning position
        board.make_move(0); // Player 1
        board.make_move(1); // Player 2
        board.make_move(2); // Player 1
        board.make_move(1); // Player 2
        board.make_move(3); // Player 1

        let mcts = MCTS::new(1.414, 4, 500); // 500ms, 4 threads
        let best_move = mcts.search(board, 1);

        // Should find the winning move
        assert_eq!(best_move, 4);
    }
}