use std::{
    collections::VecDeque,
    io::stdout,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use crossterm::{queue, style};

use super::{
    snake::{Direction, Snake, SnakePartPosition},
    terminal::Terminal,
};

const SNAKE_INITIAL_SIZE: u16 = 8;

fn get_default_snake(direction: &Arc<Mutex<Direction>>) -> Snake {
    let direction_lock = direction.lock().unwrap();

    let mut snake = Snake {
        current_direction: *direction_lock,
        parts: VecDeque::new(),
    };

    for i in 0..SNAKE_INITIAL_SIZE {
        snake
            .parts
            .push_back(SnakePartPosition { column: i, row: 0 });
    }

    snake
}

fn is_x_direction(direction: Direction) -> bool {
    direction == Direction::Left || direction == Direction::Right
}

fn is_y_direction(direction: Direction) -> bool {
    direction == Direction::Up || direction == Direction::Down
}

fn are_direction_x_axis(d1: Direction, d2: Direction) -> bool {
    is_x_direction(d1) && is_x_direction(d2)
}

fn are_direction_y_axis(d1: Direction, d2: Direction) -> bool {
    is_y_direction(d1) && is_y_direction(d2)
}

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

fn render_snake_loop(direction: &Arc<Mutex<Direction>>, snake: &mut Snake) {
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

    let direction_lock = *direction.lock().unwrap();

    let should_ignore_new_direction = are_direction_x_axis(direction_lock, snake.current_direction)
        || are_direction_y_axis(direction_lock, snake.current_direction);

    if !should_ignore_new_direction {
        snake.current_direction = direction_lock;
    }

    let snake_new_end_position = get_snake_next_end_position(snake_end_position, direction_lock);

    Terminal::write_string_to(
        &mut stdout,
        snake_new_end_position.column,
        snake_new_end_position.row,
        "X",
    );
    snake.parts.push_back(snake_new_end_position);
}

fn render_default_snake(snake: &Snake) {
    let mut stdout = stdout();
    for _ in &snake.parts {
        queue!(stdout, style::Print("X")).unwrap();
    }
}

pub fn render_snake(should_quit: &Arc<Mutex<bool>>, direction: &Arc<Mutex<Direction>>) {
    let mut snake = get_default_snake(direction);

    render_default_snake(&snake);
    loop {
        if *should_quit.lock().unwrap() {
            println!("Returning from loop");
            return;
        }
        render_snake_loop(direction, &mut snake);
    }
}
