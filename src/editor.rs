use crossterm::event::{
    read,
    Event::{self, Key},
    KeyCode::{self, Char},
    KeyEvent, KeyModifiers,
};
use terminal::Terminal;

mod terminal;

pub struct Editor {
    should_quit: bool,
    terminal: Terminal,
}

impl Editor {
    pub fn default() -> std::io::Result<Self> {
        let terminal = Terminal::default()?;

        Ok(Editor {
            should_quit: false,
            terminal,
        })
    }

    pub fn run(&mut self) -> std::io::Result<()> {
        self.terminal.initialize()?;
        self.repl()?;
        self.terminal.terminate()?;
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
            match code {
                Char('q') if *modifiers == KeyModifiers::CONTROL => {
                    self.should_quit = true;
                }
                KeyCode::Up => self.terminal.cursor_move_up()?,
                KeyCode::Down => self.terminal.cursor_move_down()?,
                KeyCode::Left => self.terminal.cursor_move_left()?,
                KeyCode::Right => self.terminal.cursor_move_right()?,
                _ => (),
            }
        }

        Ok(())
    }
    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        if self.should_quit {
            self.terminal.clear_screen()?;
            print!("Goodbye.\r\n");
        }
        Ok(())
    }
}
