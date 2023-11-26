#![warn(clippy::all, clippy::pedantic)]

mod document;
mod editor;
mod row;
mod terminal;

#[derive(Default)]
pub struct Position {
    x: usize,
    y: usize,
}

use crate::editor::Editor;

fn main() {
    Editor::default().run();
}
