use std::io::{stdout, Stdout};

use crossterm::{
    cursor::{self},
    event::KeyCode,
    execute, queue,
    style::{self, style, StyledContent, Stylize},
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
};

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

pub struct Terminal {}

impl Terminal {
    pub fn initialize() -> Result<(), std::io::Error> {
        enable_raw_mode()?;

        let mut stdout = stdout();

        queue!(
            stdout,
            cursor::MoveTo(0, 0),
            Clear(ClearType::All),
            cursor::Hide
        )
    }

    pub fn terminate() -> Result<(), std::io::Error> {
        let mut stdout = stdout();
        queue!(
            stdout,
            cursor::MoveTo(0, 0),
            Clear(ClearType::All),
            style::Print("Goodbye\r\n")
        )?;
        queue!(stdout, cursor::Show)?;
        disable_raw_mode()
    }

    pub fn write_string_to(stdout: &mut Stdout, col: u16, row: u16, content: &str) {
        execute!(stdout, cursor::MoveTo(col, row), style::Print(content)).unwrap();
    }
}
