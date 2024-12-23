#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Player {
    Human,
    RandomBot,
    MctsBot,
    Empty,
}

impl Player {
    pub fn is_bot(&self) -> bool {
        matches!(self, Player::RandomBot | Player::MctsBot)
    }
}