#![warn(clippy::all, clippy::pedantic)]

mod board;
use board::Editor;

fn main() -> std::io::Result<()> {
    let mut editor = Editor::default();
    editor.run()
}
