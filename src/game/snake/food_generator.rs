use rand::{
    distributions::{Distribution, Uniform},
    rngs::ThreadRng,
};

use crate::game::{board::BoardBoundaries, terminal::Terminal};

pub struct FoodGenerator {
    between_col: Uniform<u16>,
    between_row: Uniform<u16>,
    rng: ThreadRng,
}

impl FoodGenerator {
    pub fn new(board_boundaries: BoardBoundaries) -> Self {
        let col_start = board_boundaries.starting_col + 2;
        let col_end = board_boundaries.ending_col() - 2;

        let row_start = board_boundaries.starting_row + 2;
        let row_end = board_boundaries.ending_row() - 2;

        assert!(col_start < col_end, "board is not wide enough");
        assert!(row_start < row_end, "board is not long enough");

        FoodGenerator {
            between_col: Uniform::from(col_start..col_end),
            between_row: Uniform::from(row_start..row_end),
            rng: rand::thread_rng(),
        }
    }

    /**
     * draw a random food item and return is location in a (col, row) tuple
     */
    pub fn draw_random_food(&mut self) -> (u16, u16) {
        let col = self.between_col.sample(&mut self.rng);
        let row = self.between_row.sample(&mut self.rng);

        Terminal::write_string_to(col, row, "*").unwrap();
        (col, row)
    }
}
