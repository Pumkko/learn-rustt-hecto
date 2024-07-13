#![warn(clippy::all, clippy::pedantic)]

use game::board::Board;
mod game;

fn main() -> std::io::Result<()> {
    let mut editor = Board::default();
    editor.run()
}
