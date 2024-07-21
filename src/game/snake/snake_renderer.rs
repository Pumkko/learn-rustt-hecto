use std::{sync::mpsc::Receiver, thread, time::Duration};

use crate::game::{
    board::{BoardBoundaries, GameStatus},
    terminal::Terminal,
};

use super::{
    boundaries_check::{is_snake_biting_itself, is_snake_eating_food},
    direction::Direction,
    food_generator::FoodGenerator,
    snake_structs::{Snake, SnakePartPosition},
};

const SNAKE_INITIAL_SIZE: u16 = 4;

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

fn update_snake_direction_with_new_direction(direction: Direction, snake: &mut Snake) {
    let should_ignore_new_direction = direction.are_both_on_x_axis(snake.current_direction)
        || direction.are_both_on_y_axis(snake.current_direction);

    if !should_ignore_new_direction {
        snake.current_direction = direction;
    }
}

fn move_snake_towards_direction(
    direction: Direction,
    snake: &mut Snake,
    food_column: u16,
    food_row: u16,
) {
    update_snake_direction_with_new_direction(direction, snake);

    let snake_tail_position = snake.parts.pop_front().unwrap();
    let snake_head_position = snake.parts.back().unwrap();

    if snake_tail_position.column != food_column || snake_tail_position.row != food_row {
        Terminal::write_string_to(snake_tail_position.column, snake_tail_position.row, " ")
            .unwrap();
    }

    let snake_new_end_position =
        get_snake_next_end_position(snake_head_position, snake.current_direction);

    Terminal::write_string_to(
        snake_new_end_position.column,
        snake_new_end_position.row,
        "X",
    )
    .unwrap();
    snake.parts.push_back(snake_new_end_position);
}

fn render_default_snake(snake: &Snake) {
    if let Some(t) = snake.parts.front() {
        let snake_len = snake.parts.len();
        let snake_str = "X".repeat(snake_len);
        Terminal::write_string_to(t.column, t.row, &snake_str).unwrap();
    }
}

fn grow_snake(snake: &mut Snake) {
    let snake_tail = snake.parts.front();

    if let Some(tail_position) = snake_tail {
        let new_tail = match snake.current_direction {
            Direction::Down => SnakePartPosition {
                column: tail_position.column,
                row: tail_position.row.saturating_sub(1),
            },
            Direction::Up => SnakePartPosition {
                column: tail_position.column,
                row: tail_position.row.saturating_add(1),
            },
            Direction::Left => SnakePartPosition {
                column: tail_position.column.saturating_add(1),
                row: tail_position.row,
            },
            Direction::Right => SnakePartPosition {
                column: tail_position.column.saturating_sub(1),
                row: tail_position.row,
            },
        };

        Terminal::write_string_to(new_tail.column, new_tail.row, "X").unwrap();
        snake.parts.push_front(new_tail);
    }
}

pub fn render_snake(
    board_boundaries: BoardBoundaries,
    should_quit_receiver: &Receiver<bool>,
    direction_receiver: &Receiver<Direction>,
) -> GameStatus {
    let mut direction = direction_receiver
        .recv()
        .expect("failed to read initial direction");

    let mut snake = Snake::new(
        direction,
        board_boundaries.starting_col + 1,
        board_boundaries.starting_row + 1,
        SNAKE_INITIAL_SIZE,
    );
    render_default_snake(&snake);

    let mut food_generator = FoodGenerator::new(board_boundaries);
    let (mut food_col, mut food_row) = food_generator.draw_random_food(&snake);

    loop {
        thread::sleep(Duration::from_millis(100));

        let should_quit = match should_quit_receiver.try_recv() {
            Ok(sq) => sq,
            Err(e) => match e {
                std::sync::mpsc::TryRecvError::Disconnected => {
                    panic!("receive disconnected should_quit")
                }
                std::sync::mpsc::TryRecvError::Empty => false,
            },
        };

        direction = match direction_receiver.try_recv() {
            Ok(new_direction) => new_direction,
            Err(e) => match e {
                std::sync::mpsc::TryRecvError::Disconnected => {
                    panic!("receive disconnected should_quit")
                }
                std::sync::mpsc::TryRecvError::Empty => direction,
            },
        };

        move_snake_towards_direction(direction, &mut snake, food_col, food_row);
        if is_snake_biting_itself(&snake) || board_boundaries.is_snake_outside_boundaries(&snake) {
            return GameStatus::Lost;
        } else if is_snake_eating_food(&snake, food_col, food_row) {
            grow_snake(&mut snake);
            (food_col, food_row) = food_generator.draw_random_food(&snake);
        } else if should_quit {
            return GameStatus::Quit;
        }
    }
}
