pub struct Exchange {
    query: String,
    reply: String,
}

impl Exchange {
    pub fn new(query: String, reply: String) -> Self {
        Self { query, reply }
    }

    pub fn query(&self) -> &str {
        &self.query
    }

    pub fn reply(&self) -> &str {
        &self.reply
    }
}
