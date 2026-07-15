#[derive(Default)]
pub struct State {
    messages: Vec<String>,
}

impl State {
    pub fn messages(&self) -> &[String] {
        &self.messages
    }

    pub fn add_message(&mut self, message: String) -> () {
        self.messages.push(message);
    }
}
