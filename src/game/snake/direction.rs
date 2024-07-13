/**
 * The direction towards which the snake is moving
 */
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    pub fn is_x_direction(self) -> bool {
        self == Direction::Left || self == Direction::Right
    }

    pub fn is_y_direction(self) -> bool {
        self == Direction::Up || self == Direction::Down
    }

    pub fn are_both_on_x_axis(self, other: Direction) -> bool {
        self.is_x_direction() && other.is_x_direction()
    }

    pub fn are_both_on_y_axis(self, other: Direction) -> bool {
        self.is_y_direction() && other.is_y_direction()
    }
}
