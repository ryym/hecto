use std::{cmp, fs, io};
use unicode_segmentation::UnicodeSegmentation;

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
}

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
}

impl From<&str> for Row {
    fn from(slice: &str) -> Self {
        let string = String::from(slice);
        let len = string.graphemes(true).count();
        Self { string, len }
    }
}
