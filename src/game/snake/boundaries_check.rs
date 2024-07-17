use super::snake_structs::Snake;

pub fn is_snake_biting_itself(snake: &Snake) -> bool {
    let snake_head = snake.parts.back().expect("snake has no head !");

    let parts_where_head_is = snake.parts.iter().filter(|p| *p == snake_head).count();
    parts_where_head_is > 1
}

pub fn is_snake_eating_food(snake: &Snake, food_col: u16, food_row: u16) -> bool {
    let snake_head = snake.parts.back().expect("Snake has no head !");

    snake_head.column == food_col && snake_head.row == food_row
}
