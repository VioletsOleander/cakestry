pub struct Exchange {
    pub prompt: String,
    pub response: String,
}

impl Exchange {
    pub fn new(prompt: String, response: String) -> Self {
        Self { prompt, response }
    }
}
