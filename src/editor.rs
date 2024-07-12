use std::{
    rc::Rc,
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
    should_quit: bool,
    direction: Arc<Mutex<Direction>>,
}

impl Editor {
    pub fn default() -> Self {
        Editor {
            should_quit: false,
            direction: Arc::new(Mutex::new(Direction::Right)),
        }
    }

    pub fn run(&mut self) -> std::io::Result<()> {
        Terminal::initialize()?;

        let direction = Arc::clone(&self.direction);
        thread::spawn(move || loop {
            thread::sleep(Duration::from_secs(1));
            let direction_lock = direction.lock().unwrap();
            Terminal::move_cursor(*direction_lock).unwrap();
        });

        self.repl()?;
        Terminal::terminate()?;
        Ok(())
    }

    fn repl(&mut self) -> Result<(), std::io::Error> {
        loop {
            let event = read()?;
            self.evaluate_event(&event)?;
            self.refresh_screen()?;
            if self.should_quit {
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
            let direction = Arc::clone(&self.direction);
            let mut direction_lock = direction.lock().unwrap();
            match code {
                Char('q') if *modifiers == KeyModifiers::CONTROL => {
                    self.should_quit = true;
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
        if self.should_quit {
            Terminal::clear_screen()?;
            print!("Goodbye.\r\n");
        }
        Ok(())
    }
}
