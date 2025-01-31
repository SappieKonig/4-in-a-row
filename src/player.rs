#[derive(Copy, Clone, PartialEq)]
pub enum Player {
    Human,
    RandomBot,
    MctsBot,
}

impl Player {
    pub fn is_bot(&self) -> bool {
        matches!(self, Player::RandomBot | Player::MctsBot)
    }
}