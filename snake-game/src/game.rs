pub struct Game {
    pub snake: Vec<(u16, u16)>,
    pub food: (u16, u16),
    pub width: u16,
    pub height: u16,
    pub score: u16,
    pub is_paused: bool,
    pub is_live: bool
}

pub enum Direction {
    Up,
    Down,
    Left,
    Right
}
