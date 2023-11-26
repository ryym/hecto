use std::cmp;
use unicode_segmentation::UnicodeSegmentation;

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