use std::io::stdout;

use crossterm::{
    cursor, execute,
    terminal::{self, disable_raw_mode, enable_raw_mode, Clear, ClearType},
};

pub struct Terminal {}

impl Terminal {
    pub fn initialize() -> Result<(), std::io::Error> {
        enable_raw_mode()?;
        Self::clear_screen()?;
        let (_, rows) = terminal::size()?;

        for _ in 0..rows {
            print!("~\r\n");
        }

        execute!(stdout(), cursor::MoveTo(1, 0))
    }

    pub fn terminate() -> Result<(), std::io::Error> {
        disable_raw_mode()
    }

    pub fn clear_screen() -> Result<(), std::io::Error> {
        let mut stdout = stdout();
        execute!(stdout, cursor::MoveTo(0, 0))?;
        execute!(stdout, Clear(ClearType::All))
    }
}
