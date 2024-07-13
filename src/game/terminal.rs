use std::io::{stdout, Stdout};

use crossterm::{
    cursor::{self},
    execute, queue,
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

    pub fn draw_rectangle(
        starting_col: u16,
        starting_row: u16,
        col_size: u16,
        row_size: u16,
    ) -> std::io::Result<()> {
        let mut stdout = stdout();

        let (terminal_col_size, terminal_row_size) = crossterm::terminal::size()?;

        if terminal_col_size < col_size {
            panic!("terminal col size is too big");
        }

        if terminal_row_size < row_size {
            panic!("terminal row size is too big");
        }

        let ending_col = starting_col.saturating_add(col_size);
        let ending_row = starting_row.saturating_add(row_size);

        queue!(stdout, cursor::MoveTo(starting_row, starting_col))?;
        for _ in starting_row..ending_row {
            queue!(stdout, style::Print("|"), cursor::MoveToNextLine(1))?;
        }

        queue!(stdout, cursor::MoveTo(ending_col, starting_row))?;
        for row in starting_row..=ending_row {
            queue!(stdout, style::Print("|"), cursor::MoveTo(ending_col, row))?;
        }

        queue!(stdout, cursor::MoveTo(starting_col, starting_row))?;
        for _ in starting_col..=ending_col {
            queue!(stdout, style::Print("-"))?;
        }

        queue!(stdout, cursor::MoveTo(starting_col, ending_row))?;
        for _ in starting_col..=ending_col {
            queue!(stdout, style::Print("-"))?;
        }

        Ok(())
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
