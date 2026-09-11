use std::ops::{Index, Range};

/// A line of text edited by the user.
pub struct UserInput {
    content: String,
}

impl UserInput {
    /// Return the contained content.
    pub fn content(&self) -> &String {
        &self.content
    }

    /// Return the char indexed by `byte_idx`.
    pub fn char(&self, byte_idx: usize) -> char {
        self.content[byte_idx..]
            .chars()
            .next()
            .expect("Given byte_idx should be a valid start index of a character")
    }
}

impl UserInput {
    /// Remove the character at `byte_idx`, in the line index by `line_idx`
    ///
    /// The removed character is returned.
    pub fn remove_char(&mut self, line_idx: usize, byte_idx: usize) -> char {
        self.lines[line_idx].remove(byte_idx)
    }

    /// Remove the content specified by `range` in line index by `line_idx`.
    pub fn drain_line(&mut self, line_idx: usize, range: Range<usize>) {
        self.lines[line_idx].drain(range);
    }

    /// Truncate the line indexed by `line_idx` at `byte_idx`.
    pub fn truncate_line(&mut self, line_idx: usize, byte_idx: usize) {
        self.lines[line_idx].truncate(byte_idx);
    }

    /// Remove the line indexed by `line_idx`.
    ///
    /// The removed line is returned.
    pub fn remove_line(&mut self, line_idx: usize) -> String {
        self.lines.remove(line_idx)
    }

    /// Insert `ch` into the line indexed by `line_idx`, at `byte_idx`.
    pub fn insert_char(&mut self, line_idx: usize, byte_idx: usize, ch: char) {
        self.lines[line_idx].insert(byte_idx, ch);
    }

    /// Insert `string` into the line indexed by `line_idx`, at `byte_idx`.
    pub fn insert_str(&mut self, line_idx: usize, byte_idx: usize, string: &str) {
        self.lines[line_idx].insert_str(byte_idx, string);
    }

    /// Insert `line` at `line_idx`.
    pub fn insert_line(&mut self, line_idx: usize, line: String) {
        self.lines.insert(line_idx, line);
    }

    /// Remove all content.
    pub fn clear(&mut self) {
        self.lines = vec![String::new()];
    }
}

impl From<Vec<String>> for UserInput {
    fn from(lines: Vec<String>) -> Self {
        UserInput { lines }
    }
}

impl Index<usize> for UserInput {
    type Output = String;

    fn index(&self, line_idx: usize) -> &Self::Output {
        &self.lines[line_idx]
    }
}

impl Default for UserInput {
    fn default() -> Self {
        UserInput {
            content: String::new(),
        }
    }
}
