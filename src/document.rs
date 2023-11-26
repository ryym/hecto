use std::{
    cmp, fs,
    io::{self, Write},
};
use unicode_segmentation::UnicodeSegmentation;

use crate::Position;

#[derive(Default)]
pub struct Document {
    rows: Vec<Row>,
    pub file_name: Option<String>,
    dirty: bool,
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
            dirty: false,
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

    pub fn insert(&mut self, at: &Position, c: char) {
        if at.y > self.len() {
            return;
        }
        self.dirty = true;
        if c == '\n' {
            self.insert_newline(at);
            return;
        }
        if at.y == self.len() {
            let mut row = Row::default();
            row.insert(0, c);
            self.rows.push(row);
        } else if let Some(row) = self.rows.get_mut(at.y) {
            row.insert(at.x, c);
        }
    }

    fn insert_newline(&mut self, at: &Position) {
        if at.y == self.len() {
            self.rows.push(Row::default());
            return;
        }
        let new_row = self.rows.get_mut(at.y).unwrap().cut(at.x);
        self.rows.insert(at.y + 1, new_row);
    }

    pub fn delete(&mut self, at: &Position) {
        if at.y >= self.len() {
            return;
        }
        self.dirty = true;
        if at.x == self.rows.get(at.y).unwrap().len() && at.y < self.len() - 1 {
            let next_row = self.rows.remove(at.y + 1);
            let row = self.rows.get_mut(at.y).unwrap();
            row.append(&next_row);
        } else {
            let row = self.rows.get_mut(at.y).unwrap();
            row.delete(at.x);
        }
    }

    pub fn save(&mut self) -> Result<(), io::Error> {
        if let Some(file_name) = &self.file_name {
            let mut file = fs::File::create(file_name)?;
            for row in &self.rows {
                file.write_all(row.as_bytes())?;
                file.write_all(b"\n")?;
            }
            self.dirty = false;
        }
        Ok(())
    }

    pub fn is_dirty(&self) -> bool {
        self.dirty
    }
}

#[derive(Default)]
pub struct Row {
    string: String,
    /// The length of the string in graphemes.
    len: usize,
}

impl Row {
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
            self.len += 1;
            return;
        }
        let mut result = String::new();
        for (i, grapheme) in self.string.graphemes(true).enumerate() {
            if i == at {
                result.push(c);
            }
            result.push_str(grapheme);
        }
        self.len += 1;
        self.string = result;
    }

    pub fn delete(&mut self, at: usize) {
        if at > self.len() {
            return;
        }
        let mut result = String::new();
        for (i, grapheme) in self.string.graphemes(true).enumerate() {
            if i != at {
                result.push_str(grapheme);
            }
        }
        self.len -= 1;
        self.string = result;
    }

    pub fn append(&mut self, new: &Self) {
        self.string = format!("{}{}", self.string, new.string);
        self.len += new.len;
    }

    pub fn cut(&mut self, at: usize) -> Self {
        let mut row = String::new();
        let mut splitted = String::new();
        for (i, grapheme) in self.string.graphemes(true).enumerate() {
            if i < at {
                row.push_str(grapheme);
            } else {
                splitted.push_str(grapheme);
            }
        }
        self.len = at;
        self.string = row;
        Self::from(splitted.as_str())
    }

    pub fn as_bytes(&self) -> &[u8] {
        self.string.as_bytes()
    }
}

impl From<&str> for Row {
    fn from(slice: &str) -> Self {
        let string = String::from(slice);
        let len = string.graphemes(true).count();
        Self { string, len }
    }
}
