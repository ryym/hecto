#![warn(clippy::all, clippy::pedantic)]

mod editor;
mod terminal;

pub struct Position {
    x: usize,
    y: usize,
}

use crate::editor::Editor;

fn main() {
    Editor::default().run();
}
