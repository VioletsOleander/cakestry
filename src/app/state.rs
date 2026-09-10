pub struct State {
    wait: bool,
    mode: Mode,
}

#[derive(Clone, Copy)]
pub enum Mode {
    Normal,
    Command,
}

impl State {
    pub fn wait(&self) -> bool {
        self.wait
    }

    pub fn mode(&self) -> Mode {
        self.mode
    }

    pub fn set_wait(&mut self, wait: bool) {
        self.wait = wait;
    }

    pub fn set_mode(&mut self, mode: Mode) {
        self.mode = mode;
    }
}

impl Default for State {
    fn default() -> Self {
        State {
            wait: false,
            mode: Mode::Normal,
        }
    }
}
