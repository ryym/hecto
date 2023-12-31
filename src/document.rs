use crate::{row::Row, Position, SearchDirection};
use std::{
    fs,
    io::{self, Write},
};

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

    pub fn find(&self, query: &str, at: &Position, direction: SearchDirection) -> Option<Position> {
        if at.y >= self.rows.len() {
            return None;
        }
        match direction {
            SearchDirection::Forward => {
                let mut x_start = at.x;
                let rows_to_end = self.rows.iter().enumerate().skip(at.y);
                for (y, row) in rows_to_end {
                    if let Some(x) = row.find(query, x_start, direction) {
                        return Some(Position { x, y });
                    }
                    x_start = 0;
                }
            }
            SearchDirection::Backward => {
                let mut x_start = at.x;
                let rows_to_start = self
                    .rows
                    .iter()
                    .enumerate()
                    .take(at.y.saturating_add(1))
                    .rev();
                for (y, row) in rows_to_start {
                    if let Some(x) = row.find(query, x_start, direction) {
                        return Some(Position { x, y });
                    }
                    let prev_row = self.rows.get(y.saturating_sub(1));
                    x_start = prev_row.map_or(0, Row::len);
                }
            }
        }
        None
    }
}
