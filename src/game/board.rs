use std::{
    sync::{Arc, Mutex},
    thread::{self, JoinHandle},
};

use crossterm::event::{
    read,
    Event::{self, Key},
    KeyCode::{self, Char},
    KeyEvent, KeyModifiers,
};

use super::{
    snake::{direction::Direction, render_snake},
    terminal::Terminal,
};

pub struct Board {
    should_quit: Arc<Mutex<bool>>,
    direction: Arc<Mutex<Direction>>,
    snake_renderer_handle: Option<JoinHandle<()>>,
}

pub enum GameStatus {
    Quit,
    Lost,
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

impl Board {
    pub fn default() -> Self {
        Board {
            should_quit: Arc::new(Mutex::new(false)),
            direction: Arc::new(Mutex::new(Direction::Right)),
            snake_renderer_handle: None,
        }
    }

    pub fn run(&mut self) -> std::io::Result<()> {
        Terminal::initialize()?;

        self.start_new_game();
        self.repl()?;

        self.join_snake_renderer();
        Terminal::terminate()?;
        Ok(())
    }

    fn start_new_game(&mut self) {
        let mut direction_lock = self.direction.lock().unwrap();
        *direction_lock = Direction::Right;
        std::mem::drop(direction_lock);

        let snake_direction = self.direction.clone();
        let should_quit = self.should_quit.clone();
        let handle = thread::spawn(move || {
            let result = render_snake(&should_quit, &snake_direction);

            if let GameStatus::Lost = result {
                Terminal::print_you_lost().unwrap();
            }
        });

        self.snake_renderer_handle = Some(handle);
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

    fn join_snake_renderer(&mut self) {
        if self.snake_renderer_handle.is_some() {
            self.snake_renderer_handle
                .take()
                .unwrap()
                .join()
                .expect("Failed to join snake renderer thread");
        }
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
                Char('a') if *modifiers == KeyModifiers::CONTROL => {
                    self.join_snake_renderer();
                    Terminal::initialize().unwrap();
                    self.start_new_game();
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
