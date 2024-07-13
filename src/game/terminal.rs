use std::io::{stdout, Stdout};

use crossterm::{
    cursor::{self},
    execute,
    style::{self},
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
};

pub struct Terminal {}

impl Terminal {
    pub fn initialize() -> Result<(), std::io::Error> {
        enable_raw_mode()?;

        let mut stdout = stdout();

        execute!(
            stdout,
            cursor::MoveTo(0, 0),
            Clear(ClearType::All),
            cursor::Hide
        )
    }

    pub fn terminate() -> Result<(), std::io::Error> {
        let mut stdout = stdout();
        execute!(
            stdout,
            cursor::MoveTo(0, 0),
            Clear(ClearType::All),
            style::Print("Goodbye\r\n")
        )?;
        execute!(stdout, cursor::Show)?;
        disable_raw_mode()
    }

    pub fn print_you_lost() -> std::io::Result<()> {
        let mut stdout = stdout();

        execute!(
            stdout,
            cursor::MoveTo(0, 0),
            Clear(ClearType::All),
            style::Print("YOU LOST")
        )
    }

    pub fn write_string_to(stdout: &mut Stdout, col: u16, row: u16, content: &str) {
        execute!(stdout, cursor::MoveTo(col, row), style::Print(content)).unwrap();
    }
}
