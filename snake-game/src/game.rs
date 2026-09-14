use std::collections::VecDeque;

pub struct Game {
    pub snake: VecDeque<(u16, u16)>,
    pub food: (u16, u16),
    pub width: u16,
    pub height: u16,
    pub score: u16,
    pub is_paused: bool,
    pub is_live: bool,
    pub last_trail: Option<(u16, u16)>,
    pub wall: Vec<(u16, u16)>,
    pub changing_direction: Option<Direction>,
    pub moving_direction: Direction
}

#[derive(Clone, Copy, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right
}
