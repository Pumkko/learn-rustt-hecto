use std::{
    io::stdout,
    sync::{Arc, Mutex},
};

use crate::game::terminal::Terminal;
use crossterm::{execute, style};

use super::{
    direction::Direction,
    snake_structs::{Snake, SnakePartPosition},
};

fn get_snake_next_end_position(
    snake_end_position: &SnakePartPosition,
    direction: Direction,
) -> SnakePartPosition {
    match direction {
        Direction::Right => SnakePartPosition {
            column: snake_end_position.column.saturating_add(1),
            row: snake_end_position.row,
        },
        Direction::Down => SnakePartPosition {
            column: snake_end_position.column,
            row: snake_end_position.row.saturating_add(1),
        },
        Direction::Left => SnakePartPosition {
            column: snake_end_position.column.saturating_sub(1),
            row: snake_end_position.row,
        },
        Direction::Up => SnakePartPosition {
            column: snake_end_position.column,
            row: snake_end_position.row.saturating_sub(1),
        },
    }
}

fn update_snake_direction_with_new_direction(
    arc_direction: &Arc<Mutex<Direction>>,
    snake: &mut Snake,
) {
    let direction = *arc_direction.lock().unwrap();

    let should_ignore_new_direction = direction.are_both_on_x_axis(snake.current_direction)
        || direction.are_both_on_y_axis(snake.current_direction);

    if !should_ignore_new_direction {
        snake.current_direction = direction;
    }
}

pub fn move_snake_towards_direction(arc_direction: &Arc<Mutex<Direction>>, snake: &mut Snake) {
    update_snake_direction_with_new_direction(arc_direction, snake);

    let snake_tail_position = snake.parts.pop_front().unwrap();
    let snake_head_position = snake.parts.back().unwrap();

    let mut stdout = stdout();
    Terminal::write_string_to(
        &mut stdout,
        snake_tail_position.column,
        snake_tail_position.row,
        " ",
    );

    let snake_new_end_position =
        get_snake_next_end_position(snake_head_position, snake.current_direction);

    Terminal::write_string_to(
        &mut stdout,
        snake_new_end_position.column,
        snake_new_end_position.row,
        "X",
    );
    snake.parts.push_back(snake_new_end_position);
}

pub fn render_default_snake(snake: &Snake) {
    let mut stdout = stdout();
    for _ in &snake.parts {
        execute!(stdout, style::Print("X")).unwrap();
    }
}
