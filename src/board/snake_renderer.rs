use std::{
    collections::VecDeque,
    io::stdout,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use crossterm::{execute, queue, style};

use super::{terminal::Terminal, Direction};

#[derive(Debug)]
struct SnakePartPosition {
    pub row: u16,
    pub column: u16,
}

#[derive(Debug)]
struct Snake {
    pub current_direction: Direction,
    pub parts: VecDeque<SnakePartPosition>,
}

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
    return direction == Direction::Left || direction == Direction::Right;
}

fn is_y_direction(direction: Direction) -> bool {
    return direction == Direction::Up || direction == Direction::Down;
}

fn are_direction_x_axis(d1: Direction, d2: Direction) -> bool {
    is_x_direction(d1) && is_x_direction(d2)
}

fn are_direction_y_axis(d1: Direction, d2: Direction) -> bool {
    is_y_direction(d1) && is_y_direction(d2)
}

pub fn render_snake(should_quit: Arc<Mutex<bool>>, direction: Arc<Mutex<Direction>>) {
    let mut snake = get_default_snake(&direction);

    let mut stdout = stdout();
    for _ in &snake.parts {
        queue!(stdout, style::Print("X")).unwrap();
    }

    loop {
        thread::sleep(Duration::from_millis(200));

        if *should_quit.lock().unwrap() {
            return;
        }

        let front = snake.parts.pop_front().unwrap();
        let back = snake.parts.back().unwrap();

        Terminal::write_string_to(&mut stdout, front.column, front.row, " ");

        let direction_lock = *direction.lock().unwrap();

        let should_ignore_new_direction =
            are_direction_x_axis(direction_lock, snake.current_direction)
                || are_direction_y_axis(direction_lock, snake.current_direction);

        if !should_ignore_new_direction {
            snake.current_direction = direction_lock;
        }

        let new_position = match snake.current_direction {
            Direction::Right => SnakePartPosition {
                column: back.column.saturating_add(1),
                row: back.row,
            },
            Direction::Down => SnakePartPosition {
                column: back.column,
                row: back.row.saturating_add(1),
            },
            Direction::Left => SnakePartPosition {
                column: back.column.saturating_sub(1),
                row: back.row,
            },
            Direction::Up => SnakePartPosition {
                column: back.column,
                row: back.row.saturating_sub(1),
            },
        };

        Terminal::write_string_to(&mut stdout, new_position.column, new_position.row, "X");
        snake.parts.push_back(new_position);
    }
}
