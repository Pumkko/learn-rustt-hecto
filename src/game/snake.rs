pub mod direction;
mod snake_renderer;
mod snake_structs;

use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use direction::Direction;
use snake_renderer::{move_snake_towards_direction, render_default_snake};
use snake_structs::Snake;

use super::board::GameStatus;

const SNAKE_INITIAL_SIZE: u16 = 8;

fn is_snake_biting_itself(snake: &Snake) -> bool {
    let snake_head = snake.parts.back().expect("snake has no head !");

    let parts_where_head_is = snake.parts.iter().filter(|p| *p == snake_head).count();
    parts_where_head_is > 1
}

pub fn render_snake(
    arc_should_quit: &Arc<Mutex<bool>>,
    arc_direction: &Arc<Mutex<Direction>>,
) -> GameStatus {
    // From what i understand the following line does that :
    // direction.lock().unwrap() returns a MutexGuard<Direction>
    // But then we immediately dereference it by calling *
    // because Direction implements copy and clone, a copy of the locked direction is made and the lock is released
    let direction = *arc_direction.lock().unwrap();

    let mut snake = Snake::new(direction, SNAKE_INITIAL_SIZE);

    render_default_snake(&snake);
    loop {
        thread::sleep(Duration::from_millis(200));

        if *arc_should_quit.lock().unwrap() {
            return GameStatus::Quit;
        }
        move_snake_towards_direction(arc_direction, &mut snake);
        if is_snake_biting_itself(&snake) {
            return GameStatus::Lost;
        }
    }
}
