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
use snake_renderer::render_default_snake;
use terminal::{Direction, Terminal};

mod snake_renderer;
mod terminal;

pub struct Editor {
    should_quit: Arc<Mutex<bool>>,
    direction: Arc<Mutex<Direction>>,
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
            render_default_snake(should_quit);
        });

        self.repl()?;

        handle.join().unwrap();
        Terminal::terminate()?;
        Ok(())
    }

    fn repl(&mut self) -> Result<(), std::io::Error> {
        loop {
            let event = read()?;
            self.evaluate_event(&event)?;
            self.refresh_screen()?;
            if *self.should_quit.clone().lock().unwrap() {
                break;
            }
        }
        Ok(())
    }

    fn move_cursor(&self, direction: Direction) -> std::io::Result<()> {
        let direction_clone = self.direction.clone();
        let mut direction_lock = direction_clone.lock().unwrap();

        if *direction_lock == direction {
            Ok(())
        } else {
            match direction {
                Direction::Up => {
                    *direction_lock = Direction::Up;
                    Terminal::cursor_move_up()
                }
                Direction::Down => {
                    *direction_lock = Direction::Down;
                    Terminal::cursor_move_down()
                }
                Direction::Left => {
                    *direction_lock = Direction::Left;
                    Terminal::cursor_move_left()
                }
                Direction::Right => {
                    *direction_lock = Direction::Right;
                    Terminal::cursor_move_right()
                }
            }
        }
    }

    fn evaluate_event(&mut self, event: &Event) -> std::io::Result<()> {
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
                    self.move_cursor(direction).unwrap();
                }
                _ => (),
            }
        }

        Ok(())
    }
    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        if *self.should_quit.clone().lock().unwrap() {
            Terminal::clear_screen()?;
            print!("Goodbye.\r\n");
        }
        Ok(())
    }
}
