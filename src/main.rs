#![warn(clippy::all, clippy::pedantic)]

mod document;
mod editor;
mod row;
mod terminal;

#[derive(Default, Clone)]
pub struct Position {
    x: usize,
    y: usize,
}

#[derive(PartialEq, Copy, Clone)]
pub enum SearchDirection {
    Forward,
    Backward,
}

use crate::editor::Editor;

fn main() {
    Editor::default().run();
}
