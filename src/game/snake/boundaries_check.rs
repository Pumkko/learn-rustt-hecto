use crate::game::board::BoardBoundaries;

use super::snake_structs::Snake;

pub fn is_snake_biting_itself(snake: &Snake) -> bool {
    let snake_head = snake.parts.back().expect("snake has no head !");

    let parts_where_head_is = snake.parts.iter().filter(|p| *p == snake_head).count();
    parts_where_head_is > 1
}

pub fn is_snake_outside_boundaries(snake: &Snake, boundaries: BoardBoundaries) -> bool {
    let snake_head = snake.parts.back().expect("Snake has no head !");

    let ending_col = boundaries.ending_col();
    let ending_row = boundaries.ending_row();

    snake_head.column <= boundaries.starting_col
        || snake_head.column >= ending_col
        || snake_head.row <= boundaries.starting_row
        || snake_head.row >= ending_row
}
