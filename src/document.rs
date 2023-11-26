use std::{cmp, fs, io};
use unicode_segmentation::UnicodeSegmentation;

use crate::Position;

#[derive(Default)]
pub struct Document {
    rows: Vec<Row>,
    file_name: Option<String>,
}

impl Document {
    pub fn open(file_name: &str) -> Result<Self, io::Error> {
        let contents = fs::read_to_string(file_name)?;
        let mut rows = Vec::new();
        for line in contents.lines() {
            rows.push(Row::from(line));
        }
        Ok(Self {
            rows,
            file_name: Some(file_name.to_string()),
        })
    }

    pub fn row(&self, index: usize) -> Option<&Row> {
        self.rows.get(index)
    }

    pub fn len(&self) -> usize {
        self.rows.len()
    }

    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }

    pub fn file_name(&self) -> Option<&String> {
        self.file_name.as_ref()
    }

    pub fn insert(&mut self, at: &Position, c: char) {
        if at.y == self.len() {
            let mut row = Row::default();
            row.insert(0, c);
            self.rows.push(row);
        } else if let Some(row) = self.rows.get_mut(at.y) {
            row.insert(at.x, c);
        }
    }

    pub fn delete(&mut self, at: &Position) {
        if at.y >= self.len() {
            return;
        }
        if at.x == self.rows.get(at.y).unwrap().len() && at.y < self.len() - 1 {
            let next_row = self.rows.remove(at.y + 1);
            let row = self.rows.get_mut(at.y).unwrap();
            row.append(&next_row);
        } else {
            let row = self.rows.get_mut(at.y).unwrap();
            row.delete(at.x);
        }
    }
}

#[derive(Default)]
pub struct Row {
    string: String,
    /// The length of the string in graphemes.
    len: usize,
}

impl Row {
    fn update_string(&mut self, string: String) {
        self.len = string.graphemes(true).count();
        self.string = string;
    }

    pub fn render(&self, start: usize, end: usize) -> String {
        let end = cmp::min(end, self.string.len());
        let start = cmp::min(start, end);
        self.string
            .graphemes(true)
            .skip(start)
            .take(end - start)
            .collect()
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn insert(&mut self, at: usize, c: char) {
        if at >= self.len() {
            self.string.push(c);
        } else {
            let mut result: String = self.string.graphemes(true).take(at).collect();
            let reminder: String = self.string.graphemes(true).skip(at).collect();
            result.push(c);
            result.push_str(&reminder);
            self.update_string(result);
        }
    }

    pub fn delete(&mut self, at: usize) {
        if at < self.len() {
            let mut result: String = self.string.graphemes(true).take(at).collect();
            let reminder: String = self.string.graphemes(true).skip(at + 1).collect();
            result.push_str(&reminder);
            self.update_string(result);
        }
    }

    pub fn append(&mut self, new: &Self) {
        self.update_string(format!("{}{}", self.string, new.string));
    }
}

impl From<&str> for Row {
    fn from(slice: &str) -> Self {
        let string = String::from(slice);
        let len = string.graphemes(true).count();
        Self { string, len }
    }
}
