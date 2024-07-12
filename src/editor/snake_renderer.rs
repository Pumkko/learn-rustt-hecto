use std::{
    io::stdout,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use crossterm::{cursor, execute, queue, style};

pub fn render_default_snake(should_quit: Arc<Mutex<bool>>) {
    let snake_size: u16 = 4;
    let mut snake_column_end: u16 = 0;
    let mut snake_column_start: u16 = 0;
    let mut snake_row_end: u16 = 0;
    let mut snake_row_start: u16 = 0;

    loop {
        thread::sleep(Duration::from_millis(500));

        if *should_quit.lock().unwrap() {
            return;
        }

        let mut stdout = stdout();
        execute!(stdout, crossterm::terminal::BeginSynchronizedUpdate).unwrap();
        if snake_column_start > 0 {
            queue!(
                stdout,
                cursor::MoveTo(snake_column_start, snake_row_start),
                style::Print(" "),
                cursor::MoveTo(snake_column_end + 1, snake_row_end),
                style::Print("X")
            )
            .unwrap();

            snake_column_end += 1;
            snake_column_start += 1;
        } else {
            for n in 0..snake_size {
                queue!(stdout, style::Print("X")).unwrap();
            }

            snake_column_end += snake_size;
            snake_column_start += 1;
        }

        execute!(stdout, crossterm::terminal::EndSynchronizedUpdate).unwrap();
    }
}
