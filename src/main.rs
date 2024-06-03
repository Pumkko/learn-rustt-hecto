#![warn(clippy::all, clippy::pedantic)]

mod editor;
use editor::Editor;

fn main() -> std::io::Result<()> {
    let mut editor = Editor::default()?;
    editor.run()
}
