use std::collections::VecDeque;

use super::direction::Direction;

#[derive(Debug, PartialEq, Eq)]
pub struct SnakePartPosition {
    pub row: u16,
    pub column: u16,
}

#[derive(Debug)]
pub struct Snake {
    pub current_direction: Direction,
    pub parts: VecDeque<SnakePartPosition>,
}

impl Snake {
    pub fn new(direction: Direction, size: u16) -> Self {
        let mut snake = Snake {
            current_direction: direction,
            parts: VecDeque::new(),
        };

        for i in 0..size {
            snake
                .parts
                .push_back(SnakePartPosition { column: i, row: 0 });
        }

        snake
    }
}
