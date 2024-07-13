use std::{
    sync::{Arc, Mutex},
    thread,
};

use crossterm::event::{
    read,
    Event::{self, Key},
    KeyCode::{self, Char},
    KeyEvent, KeyModifiers,
};
use snake_renderer::render_snake;
use terminal::Terminal;

mod snake_renderer;
mod terminal;

pub struct Editor {
    should_quit: Arc<Mutex<bool>>,
    direction: Arc<Mutex<Direction>>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl From<&KeyCode> for Direction {
    fn from(value: &KeyCode) -> Self {
        match value {
            KeyCode::Up => Direction::Up,
            KeyCode::Down => Direction::Down,
            KeyCode::Left => Direction::Left,
            KeyCode::Right => Direction::Right,
            _ => panic!("Can only map from up, left, down, right keycode"),
        }
    }
}

impl Editor {
    pub fn default() -> Self {
        Editor {
            should_quit: Arc::new(Mutex::new(false)),
            direction: Arc::new(Mutex::new(Direction::Right)),
        }
    }

    pub fn run(&mut self) -> std::io::Result<()> {
        Terminal::initialize()?;

        let direction = self.direction.clone();
        let should_quit = self.should_quit.clone();
        let handle = thread::spawn(move || {
            render_snake(should_quit, direction);
        });

        self.repl()?;

        handle.join().unwrap();
        Terminal::terminate()?;
        Ok(())
    }

    fn repl(&mut self) -> Result<(), std::io::Error> {
        loop {
            let event = read()?;
            self.evaluate_event(&event);
            if *self.should_quit.clone().lock().unwrap() {
                break;
            }
        }
        Ok(())
    }

    fn move_cursor(&self, direction: Direction) {
        let direction_clone = self.direction.clone();
        let mut direction_lock = direction_clone.lock().unwrap();

        *direction_lock = direction;
    }

    fn evaluate_event(&mut self, event: &Event) {
        if let Key(KeyEvent {
            code, modifiers, ..
        }) = event
        {
            match code {
                Char('q') if *modifiers == KeyModifiers::CONTROL => {
                    *self.should_quit.clone().lock().unwrap() = true;
                }
                KeyCode::Up | KeyCode::Left | KeyCode::Down | KeyCode::Right => {
                    let direction: Direction = code.into();
                    self.move_cursor(direction);
                }
                _ => (),
            }
        }
    }
}
