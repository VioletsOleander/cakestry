/// A round of query and reply between user and assistant.
pub struct Exchange {
    query_lines: Vec<String>,
    reply_lines: Vec<String>,
}

impl Exchange {
    pub fn new(query_lines: Vec<String>, reply_lines: Vec<String>) -> Self {
        Exchange {
            query_lines,
            reply_lines,
        }
    }

    pub fn query_lines(&self) -> &Vec<String> {
        &self.query_lines
    }

    pub fn reply_lines(&self) -> &Vec<String> {
        &self.reply_lines
    }
}
