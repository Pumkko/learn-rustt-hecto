use std::{
    sync::mpsc::{self, Sender},
    thread::{self, JoinHandle},
};

use crossterm::event::{
    read,
    Event::{self, Key},
    KeyCode::{self, Char},
    KeyEvent, KeyModifiers,
};

use super::{
    snake::{direction::Direction, snake_renderer::render_snake, snake_structs::Snake},
    terminal::Terminal,
};

#[derive(Copy, Clone, Debug)]
pub struct BoardBoundaries {
    pub starting_col: u16,
    pub starting_row: u16,

    /**
     * Define the size of the Y axis of the boundaries
     * it will be excluded so for exemple a
     * `starting_col: 0`
     * `col_size: 5`
     * means (peusdo code) boundaries[col: 5, row: 0] would be outside the limit i.e it would hit the rectangle's border
     */
    pub col_size: u16,

    /**
     * Define the size of the X axis of the boundaries
     * it will be excluded so for exemple a
     * `starting_row: 0`
     * `row_size: 5`
     * means boundaries[col: 0, row: 5] would be outside the limit i.e it would hit the rectangle's border
     */
    pub row_size: u16,
}

impl BoardBoundaries {
    pub fn ending_col(self) -> u16 {
        self.starting_col.saturating_add(self.col_size)
    }

    pub fn ending_row(self) -> u16 {
        self.starting_row.saturating_add(self.row_size)
    }

    pub fn is_snake_outside_boundaries(self, snake: &Snake) -> bool {
        let snake_head = snake.parts.back().expect("Snake has no head !");

        let ending_col = self.ending_col();
        let ending_row = self.ending_row();

        snake_head.column <= self.starting_col
            || snake_head.column >= ending_col
            || snake_head.row <= self.starting_row
            || snake_head.row >= ending_row
    }
    pub fn is_point_within_boundaries(self, column: u16, row: u16) -> bool {
        let ending_col = self.ending_col();
        let ending_row = self.ending_row();

        column > self.starting_col
            && column < ending_col
            && row > self.starting_row
            && row < ending_row
    }
}

pub enum GameStatus {
    Quit,
    Lost,
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

pub struct Board {
    should_quit: bool,
    direction: Direction,
    should_quit_sender: Option<Sender<bool>>,
    direction_sender: Option<Sender<Direction>>,
    boundaries: BoardBoundaries,
    snake_renderer_handle: Option<JoinHandle<()>>,
}

impl Board {
    pub fn default() -> Self {
        Board {
            direction: Direction::default(),
            should_quit: false,
            snake_renderer_handle: None,
            should_quit_sender: None,
            direction_sender: None,
            boundaries: BoardBoundaries {
                starting_col: 0,
                starting_row: 0,
                col_size: 40,
                row_size: 10,
            },
        }
    }

    pub fn run(&mut self) -> std::io::Result<()> {
        Terminal::initialize()?;
        self.start_new_game();
        self.repl()?;

        self.join_snake_renderer();
        Terminal::terminate()?;
        Ok(())
    }

    fn start_new_game(&mut self) {
        Terminal::draw_rectangle(
            self.boundaries.starting_col,
            self.boundaries.starting_row,
            self.boundaries.col_size,
            self.boundaries.row_size,
        )
        .unwrap();

        let (should_quit_sender, should_quit_receiver) = mpsc::channel::<bool>();
        let (direction_sender, direction_receiver) = mpsc::channel::<Direction>();

        self.should_quit_sender = Some(should_quit_sender);
        self.direction_sender = Some(direction_sender);

        self.set_should_quit(false);
        self.set_new_direction(Direction::default());

        let boundaries = self.boundaries;
        let handle = thread::spawn(move || {
            let result = render_snake(boundaries, &should_quit_receiver, &direction_receiver);

            if let GameStatus::Lost = result {
                Terminal::clear_and_write_string_to(0, 0, "YOU LOST").unwrap();
            }
        });

        self.snake_renderer_handle = Some(handle);
    }

    fn repl(&mut self) -> Result<(), std::io::Error> {
        loop {
            let event = read()?;
            self.evaluate_event(&event);
            if self.should_quit {
                break;
            }
        }
        Ok(())
    }

    fn join_snake_renderer(&mut self) {
        if self.snake_renderer_handle.is_some() {
            self.snake_renderer_handle
                .take()
                .unwrap()
                .join()
                .expect("Failed to join snake renderer thread");
        }
    }

    fn set_new_direction(&mut self, new_direction: Direction) {
        self.direction = new_direction;

        match &self.direction_sender {
            Some(sender) => sender.send(self.direction).unwrap(),
            None => panic!("no sender for direction !"),
        }
    }

    fn set_should_quit(&mut self, should_quit: bool) {
        self.should_quit = should_quit;
        match &self.should_quit_sender {
            Some(sender) => sender.send(self.should_quit).unwrap(),
            None => panic!("no sender for should_quit !"),
        }
    }

    fn evaluate_event(&mut self, event: &Event) {
        if let Key(KeyEvent {
            code, modifiers, ..
        }) = event
        {
            let is_game_finished = match &self.snake_renderer_handle {
                None => false,
                Some(h) => h.is_finished(),
            };

            match code {
                Char('q') if *modifiers == KeyModifiers::CONTROL => self.set_should_quit(true),
                Char('a') if *modifiers == KeyModifiers::CONTROL && is_game_finished => {
                    self.join_snake_renderer();
                    Terminal::initialize().unwrap();
                    self.start_new_game();
                }
                KeyCode::Up | KeyCode::Left | KeyCode::Down | KeyCode::Right => {
                    if !is_game_finished {
                        let new_direction: Direction = code.into();
                        self.set_new_direction(new_direction);
                    }
                }
                _ => (),
            }
        }
    }
}
