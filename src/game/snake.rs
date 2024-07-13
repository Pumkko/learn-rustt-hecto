use std::collections::VecDeque;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Debug)]
pub struct SnakePartPosition {
    pub row: u16,
    pub column: u16,
}

#[derive(Debug)]
pub struct Snake {
    pub current_direction: Direction,
    pub parts: VecDeque<SnakePartPosition>,
}
