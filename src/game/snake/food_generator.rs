use rand::{
    distributions::{Distribution, Uniform},
    rngs::ThreadRng,
};

use crate::game::{board::BoardBoundaries, terminal::Terminal};

use super::snake_structs::{Snake, SnakePartPosition};

pub struct FoodGenerator {
    between_col: Uniform<u16>,
    between_row: Uniform<u16>,
    board_boundaries: BoardBoundaries,
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
            board_boundaries,
            rng: rand::thread_rng(),
        }
    }

    /**
     * draw a random food item and return is location in a (column, row) tuple
     */
    pub fn draw_random_food(&mut self, snake: &Snake) -> (u16, u16) {
        let mut column = self.between_col.sample(&mut self.rng);
        let mut row = self.between_row.sample(&mut self.rng);

        let potential_existing_part = SnakePartPosition { row, column };
        let any_conflict = snake.parts.contains(&potential_existing_part);

        if any_conflict {
            (column, row) = self.draw_food_behind_snake(snake);
        }

        Terminal::write_string_to(column, row, "*").unwrap();
        (column, row)
    }

    fn draw_food_behind_snake(&mut self, snake: &Snake) -> (u16, u16) {
        // I don't want to loop and generate random numbers until it works
        // it can technically (though unlikely the player has to be very very good) fail
        // so if we have a conflict, let's just draw the food right behind the snake

        let snake_tail_option = snake.parts.front();

        if let Some(snake_tail) = snake_tail_option {
            let left_snake_col = snake_tail.column.saturating_sub(2);
            let right_snake_col = snake_tail.column.saturating_add(2);
            let above_snake_row = snake_tail.row.saturating_sub(2);
            let below_snake_row = snake_tail.row.saturating_add(2);

            if self
                .board_boundaries
                .is_point_within_boundaries(left_snake_col, snake_tail.row)
            {
                return (left_snake_col, snake_tail.row);
            } else if self
                .board_boundaries
                .is_point_within_boundaries(right_snake_col, snake_tail.row)
            {
                return (right_snake_col, snake_tail.row);
            } else if self
                .board_boundaries
                .is_point_within_boundaries(snake_tail.column, above_snake_row)
            {
                return (snake_tail.column, above_snake_row);
            } else if self
                .board_boundaries
                .is_point_within_boundaries(snake_tail.column, below_snake_row)
            {
                return (snake_tail.column, below_snake_row);
            }

            (0, 0)
        } else {
            panic!("snake has no tail");
        }
    }
}
