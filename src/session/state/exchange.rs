pub struct Exchange {
    query_lines: Vec<String>,
    reply_lines: Vec<String>,
}

impl Exchange {
    pub fn query_lines(&self) -> &Vec<String> {
        &self.query_lines
    }

    pub fn reply_lines(&self) -> &Vec<String> {
        &self.reply_lines
    }
}
