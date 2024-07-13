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
    pub fn new(
        direction: Direction,
        snake_starting_col: u16,
        snake_starting_row: u16,
        size: u16,
    ) -> Self {
        let mut snake = Snake {
            current_direction: direction,
            parts: VecDeque::new(),
        };

        for i in snake_starting_col..(size + snake_starting_col) {
            snake.parts.push_back(SnakePartPosition {
                column: i,
                row: snake_starting_row,
            });
        }

        snake
    }
}
