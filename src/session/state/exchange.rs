/// A round of query and reply between user and assistant.
pub struct Exchange {
    query: String,
    reply: String,
}

impl Exchange {
    pub fn new(query: String, reply: String) -> Self {
        Exchange { query, reply }
    }

    pub fn query(&self) -> &String {
        &self.query
    }

    pub fn reply(&self) -> &String {
        &self.reply
    }

    pub fn set_reply(&mut self, reply: String) {
        self.reply = reply;
    }

    pub fn push_to_reply(&mut self, delta: &str) {
        self.reply.push_str(delta);
    }
}
