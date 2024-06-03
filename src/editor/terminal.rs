use std::io::stdout;

use crossterm::{
    cursor::{self, MoveToNextLine},
    execute, queue, style,
    terminal::{self, disable_raw_mode, enable_raw_mode, Clear, ClearType},
};

struct ScreenSize {
    columns: u16,
    rows: u16,
}

pub struct Terminal {
    screen_size: ScreenSize,
}

use std::io::Result as TerminalResult;

impl Terminal {
    pub fn default() -> TerminalResult<Self> {
        let (columns, rows) = terminal::size()?;

        Ok(Terminal {
            screen_size: ScreenSize { columns, rows },
        })
    }

    pub fn initialize(&self) -> Result<(), std::io::Error> {
        enable_raw_mode()?;
        self.clear_screen()?;

        let mut stdout = stdout();

        for _ in 0..self.screen_size.rows {
            queue!(stdout, style::Print("~"), MoveToNextLine(1))?;
        }

        execute!(stdout, cursor::MoveTo(1, 0))
    }

    pub fn terminate(&self) -> Result<(), std::io::Error> {
        disable_raw_mode()
    }

    pub fn clear_screen(&self) -> Result<(), std::io::Error> {
        let mut stdout = stdout();
        execute!(stdout, cursor::MoveTo(0, 0), Clear(ClearType::All))
    }

    pub fn cursor_move_up(&self) -> std::io::Result<()> {
        execute!(stdout(), cursor::MoveUp(1))
    }

    pub fn cursor_move_down(&self) -> std::io::Result<()> {
        execute!(stdout(), cursor::MoveDown(1))
    }

    pub fn cursor_move_right(&self) -> std::io::Result<()> {
        execute!(stdout(), cursor::MoveRight(1))
    }

    pub fn cursor_move_left(&self) -> std::io::Result<()> {
        execute!(stdout(), cursor::MoveLeft(1))
    }
}
