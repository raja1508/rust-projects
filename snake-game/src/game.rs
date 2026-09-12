pub struct Game {
    pub snake: Vec<(u16, u16)>,
    pub food: (u16, u16),
    pub move: (u16, u16)
    pub width: u16,
    pub height: u16,
    pub score: u16
}

pub enum Direction {
    Up,
    Down,
    Left,
    Right
}
