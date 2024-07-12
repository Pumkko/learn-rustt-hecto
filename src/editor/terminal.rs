use std::io::stdout;

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
        Self::clear_screen()?;

        let mut stdout = stdout();
        queue!(
            stdout,
            cursor::Hide,
            cursor::MoveTo(5, 6),
            style::Print("*")
        )?;
        execute!(stdout, cursor::MoveTo(1, 0))
    }

    pub fn terminate() -> Result<(), std::io::Error> {
        let mut stdout = stdout();
        queue!(stdout, cursor::Show,)?;
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

    pub fn move_cursor(direction: Direction) -> std::io::Result<()> {
        match direction {
            Direction::Down => Self::cursor_move_down(),
            Direction::Up => Self::cursor_move_up(),
            Direction::Left => Self::cursor_move_left(),
            Direction::Right => Self::cursor_move_right(),
        }
    }
}
