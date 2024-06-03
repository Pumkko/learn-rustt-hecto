#![warn(clippy::all, clippy::pedantic)]

mod editor;
use editor::Editor;

fn main() {
    let mut editor = Editor::default();
    if let Err(e) = editor.run() {
        println!("{e}");
    }
}
