use std::io::stdout;

use crossterm::{
    cursor::{self, MoveToNextLine},
    execute, queue, style,
    terminal::{self, disable_raw_mode, enable_raw_mode, Clear, ClearType},
};

pub struct Terminal {}

impl Terminal {
    pub fn initialize() -> Result<(), std::io::Error> {
        enable_raw_mode()?;
        Self::clear_screen()?;
        let (_, rows) = terminal::size()?;

        let mut stdout = stdout();

        for _ in 0..rows {
            queue!(stdout, style::Print("~"), MoveToNextLine(1))?;
        }

        execute!(stdout, cursor::MoveTo(1, 0))
    }

    pub fn terminate() -> Result<(), std::io::Error> {
        disable_raw_mode()
    }

    pub fn clear_screen() -> Result<(), std::io::Error> {
        let mut stdout = stdout();
        execute!(stdout, cursor::MoveTo(0, 0), Clear(ClearType::All))
    }

    pub fn cursor_move_up() -> std::io::Result<()> {
        execute!(stdout(), cursor::MoveUp(1))
    }

    pub fn cursor_move_down() -> std::io::Result<()> {
        execute!(stdout(), cursor::MoveDown(1))
    }

    pub fn cursor_move_right() -> std::io::Result<()> {
        execute!(stdout(), cursor::MoveRight(1))
    }
    
    pub fn cursor_move_left() -> std::io::Result<()> {
        execute!(stdout(), cursor::MoveLeft(1))
    }
}
