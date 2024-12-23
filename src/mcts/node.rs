use crate::bitboard::BitBoard;

// Regular node for the tree exploration
pub struct Node {
    board: BitBoard,
    visits: u32,
    wins: i32,
    action: Option<usize>,
    children: Vec<Node>,
    untried_moves: Vec<usize>,
}

// Special root node that supports parallel access
pub struct RootNode {
    board: BitBoard,
    visits: std::sync::atomic::AtomicU32,
    wins: std::sync::atomic::AtomicI32,
    children: Vec<Node>,
    untried_moves: Vec<usize>,
}

impl Node {
    pub fn new(board: BitBoard, action: Option<usize>) -> Self {
        let untried_moves = board.get_valid_moves();
        Self {
            board,
            visits: 0,
            wins: 0,
            action,
            children: Vec::new(),
            untried_moves,
        }
    }
    
    pub fn ucb1(&self, parent_visits: u32, exploration_constant: f32) -> f32 {
        if self.visits == 0 {
            return f32::INFINITY;
        }
        
        let win_ratio = self.wins as f32 / self.visits as f32;
        let exploration = exploration_constant * ((parent_visits as f32).ln() / self.visits as f32).sqrt();
        
        win_ratio + exploration
    }
    
    pub fn expand(&mut self) -> Option<&mut Node> {
        let move_index = fastrand::usize(..self.untried_moves.len());
        let action = self.untried_moves.swap_remove(move_index);
        
        let mut new_board = self.board;
        if new_board.make_move(action) {
            let new_node = Node::new(new_board, Some(action));
            self.children.push(new_node);
            self.children.last_mut()
        } else {
            None
        }
    }
    
    pub fn update(&mut self, result: i32) {
        self.visits += 1;
        self.wins += result;
    }
}

impl RootNode {
    pub fn new(board: BitBoard) -> Self {
        let untried_moves = board.get_valid_moves();
        Self {
            board,
            visits: std::sync::atomic::AtomicU32::new(0),
            wins: std::sync::atomic::AtomicI32::new(0),
            children: Vec::new(),
            untried_moves,
        }
    }

    // Creates a new tree copy for a thread to work with
    pub fn create_thread_tree(&self) -> Node {
        Node::new(self.board, None)
    }

    // Merge results from a thread's exploration back to root
    pub fn merge_thread_results(&self, thread_node: &Node) {
        self.visits.fetch_add(thread_node.visits, std::sync::atomic::Ordering::Relaxed);
        self.wins.fetch_add(thread_node.wins, std::sync::atomic::Ordering::Relaxed);
    }
}