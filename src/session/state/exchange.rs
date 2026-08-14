/// A round of query and reply between user and assistant.
pub struct Exchange {
    query: String,
    reply: String,
}

impl Exchange {
    pub fn new(query: String, reply: String) -> Self {
        Exchange { query, reply }
    }

    pub fn query(&self) -> &str {
        &self.query
    }

    pub fn reply(&self) -> &str {
        &self.reply
    }

    pub fn set_reply(&mut self, reply: String) {
        self.reply = reply;
    }
}
