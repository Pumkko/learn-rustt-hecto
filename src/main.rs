#![warn(clippy::all, clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]

use game::board::Board;
mod game;

fn main() -> std::io::Result<()> {
    let mut editor = Board::default();
    editor.run()
}
