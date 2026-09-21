use std::ops::Range;

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
    /// Remove the character at `byte_idx`.
    ///
    /// The removed character is returned.
    pub fn remove_char(&mut self, byte_idx: usize) -> char {
        self.content.remove(byte_idx)
    }

    /// Remove the content specified by `range`.
    pub fn drain(&mut self, range: Range<usize>) {
        self.content.drain(range);
    }

    /// Truncate the content at `byte_idx`.
    pub fn truncate_line(&mut self, byte_idx: usize) {
        self.content.truncate(byte_idx);
    }

    /// Insert `ch` at `byte_idx`.
    pub fn insert_char(&mut self, byte_idx: usize, ch: char) {
        self.content.insert(byte_idx, ch);
    }

    /// Insert `string` at `byte_idx`.
    pub fn insert_str(&mut self, byte_idx: usize, string: &str) {
        self.content.insert_str(byte_idx, string);
    }

    /// Remove all content.
    pub fn clear(&mut self) {
        self.content = String::new();
    }
}

impl From<String> for UserInput {
    fn from(content: String) -> Self {
        UserInput { content }
    }
}

impl Default for UserInput {
    fn default() -> Self {
        UserInput {
            content: String::new(),
        }
    }
}
