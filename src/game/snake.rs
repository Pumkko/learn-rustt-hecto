pub mod direction;
mod snake_renderer;
mod snake_structs;

use std::sync::{Arc, Mutex};

use direction::Direction;
use snake_renderer::{render_default_snake, render_snake_loop};
use snake_structs::Snake;

const SNAKE_INITIAL_SIZE: u16 = 8;

pub fn render_snake(arc_should_quit: &Arc<Mutex<bool>>, arc_direction: &Arc<Mutex<Direction>>) {
    // From what i understand the following line does that :
    // direction.lock().unwrap() returns a MutexGuard<Direction>
    // But then we immediately dereference it by calling *
    // because Direction implements copy and clone, a copy of the locked direction is made and the lock is released
    let direction = *arc_direction.lock().unwrap();

    let mut snake = Snake::new(direction, SNAKE_INITIAL_SIZE);

    render_default_snake(&snake);
    loop {
        if *arc_should_quit.lock().unwrap() {
            return;
        }
        render_snake_loop(arc_direction, &mut snake);
    }
}
