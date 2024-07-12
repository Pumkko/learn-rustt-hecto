use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use crossterm::event::{
    read,
    Event::{self, Key},
    KeyCode::{self, Char},
    KeyEvent, KeyModifiers,
};
use terminal::{Direction, Terminal};

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
        let handle = thread::spawn(move || loop {
            thread::sleep(Duration::from_secs(1));

            if *should_quit.lock().unwrap() {
                return;
            }

            let direction_lock = direction.lock().unwrap();
            Terminal::move_cursor(*direction_lock).unwrap();
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
    fn evaluate_event(&mut self, event: &Event) -> std::io::Result<()> {
        if let Key(KeyEvent {
            code, modifiers, ..
        }) = event
        {
            let direction = self.direction.clone();
            let mut direction_lock = direction.lock().unwrap();
            match code {
                Char('q') if *modifiers == KeyModifiers::CONTROL => {
                    *self.should_quit.clone().lock().unwrap() = true;
                }
                KeyCode::Up => {
                    *direction_lock = Direction::Up;
                    Terminal::cursor_move_up()?
                }
                KeyCode::Down => {
                    *direction_lock = Direction::Down;
                    Terminal::cursor_move_down()?
                }
                KeyCode::Left => {
                    *direction_lock = Direction::Left;
                    Terminal::cursor_move_left()?
                }
                KeyCode::Right => {
                    *direction_lock = Direction::Right;
                    Terminal::cursor_move_right()?
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
