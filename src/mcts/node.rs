use crate::games::connect4::bitboard::BitBoard;
use std::collections::HashMap;

// Regular node for the tree exploration
#[derive(Clone)]  // Explicit derive
pub struct Node {
    pub board: BitBoard,
    visits: u32,
    wins: i32,
    action: Option<usize>,
    pub children: Vec<Node>,
    untried_moves: Vec<usize>,
    pub result: Option<i32>,
}

// Special root node that supports parallel access
pub struct RootNode {
    board: BitBoard,
    move_to_visits: HashMap<usize, u32>,
}

impl Node {
    pub fn new(board: BitBoard, action: Option<usize>, result: Option<i32>) -> Self {
        let untried_moves = board.get_valid_moves();
        let children = Vec::new();
        Self {
            board,
            visits: 0,
            wins: 0,
            action,
            children,
            untried_moves,
            result,
        }
    }

    pub fn get_win_ratio(&self) -> f32 {
        self.wins as f32 / (self.visits + 1) as f32
    }

    pub fn get_best_child_index(&self, exploration_constant: f32) -> usize {
        self.children.iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| {
                self.ucb1(a, exploration_constant).partial_cmp(&self.ucb1(b, exploration_constant)).unwrap()
            })
            .map(|(i, _)| i)
            .unwrap()
    }
    
    pub fn ucb1(&self, child: &Node, exploration_constant: f32) -> f32 {
        let win_ratio = child.get_win_ratio();
        let exploration = exploration_constant * ((self.visits as f32).ln() / child.visits as f32).sqrt();
        
        if self.board.get_current_player() == 1 {
            win_ratio + exploration
        } else {
            -win_ratio + exploration
        }
    }
    
    pub fn expand(&mut self) -> &mut Node {
        let move_index = fastrand::usize(..self.untried_moves.len());
        let action = self.untried_moves.swap_remove(move_index);
        
        let mut new_board = self.board;
        let result = new_board.make_move(action);
        
        self.children.push(Node::new(new_board, Some(action), result));
        self.children.last_mut().unwrap()
    }
    
    pub fn update(&mut self, wins: i32, n_simulations: u32) {
        self.visits += n_simulations;
        self.wins += wins;
    }

    pub fn is_fully_expanded(&self) -> bool {
        self.untried_moves.is_empty()
    }

    pub fn is_terminal(&self) -> bool {
        self.result.is_some()
    }

    pub fn simulate(&self, player_number: u8, n_simulations: u32) -> i32 {
        let mut wins = 0;
        for _ in 0..n_simulations {
            let result = self._simulate(self.board, player_number);
            wins += result;
        }
        wins
    }

    fn _simulate(&self, mut board: BitBoard, player_number: u8) -> i32 {
        let mut current_player = player_number;

        loop {
            let valid_moves = board.get_valid_moves();

            let mov = valid_moves[fastrand::usize(..valid_moves.len())];
            let result = board.make_move(mov);
            if result.is_some() {
                return result.unwrap();
            }
        }
    }
}

impl RootNode {
    pub fn new(board: BitBoard) -> Self {
        let untried_moves = board.get_valid_moves();
        let mut move_to_visits = HashMap::new();
        for m in untried_moves {
            move_to_visits.insert(m, 0);
        }
        Self {
            board,
            move_to_visits,
        }
    }

    pub fn create_thread_tree(&self) -> Node {
        Node::new(self.board, None, None)
    }

    pub fn merge_thread_results(&mut self, thread_node: &Node) {
        for child in &thread_node.children {
            if let Some(action) = child.action {
                self.move_to_visits.entry(action).and_modify(|v| *v += child.visits);
            }
        }
    }

    pub fn get_best_move(&self) -> usize {
        *self.move_to_visits.iter()
            .max_by_key(|(_, &visits)| visits)
            .unwrap()
            .0
    }
    
    pub fn get_total_visits(&self) -> u32 {
        self.move_to_visits.values().sum()
    }
}