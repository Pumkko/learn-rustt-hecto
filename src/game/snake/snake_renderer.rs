use std::{
    io::stdout,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use crossterm::{queue, style};

use crate::game::terminal::Terminal;

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

pub fn render_snake_loop(arc_direction: &Arc<Mutex<Direction>>, snake: &mut Snake) {
    thread::sleep(Duration::from_millis(200));

    let snake_begin_position = snake.parts.pop_front().unwrap();
    let snake_end_position = snake.parts.back().unwrap();

    let mut stdout = stdout();
    Terminal::write_string_to(
        &mut stdout,
        snake_begin_position.column,
        snake_begin_position.row,
        " ",
    );

    let direction = *arc_direction.lock().unwrap();

    let should_ignore_new_direction = direction.are_both_on_x_axis(snake.current_direction)
        || direction.are_both_on_y_axis(snake.current_direction);

    if !should_ignore_new_direction {
        snake.current_direction = direction;
    }

    let snake_new_end_position = get_snake_next_end_position(snake_end_position, direction);

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
        queue!(stdout, style::Print("X")).unwrap();
    }
}
