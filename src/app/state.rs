// use crate::terminal::{KeyCode, KeyEvent, KeyModifiers};

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
}

impl State {
    pub fn set_wait(&mut self, wait: bool) {
        self.wait = wait;
    }

    pub fn set_mode(&mut self, mode: Mode) {
        self.mode = mode;
    }

    // pub fn handle_key(&mut self, key: KeyEvent) {
    //     match key.modifiers {
    //         KeyModifiers::CONTROL => (),
    //         KeyModifiers::SHIFT => match key.code {
    //             KeyCode::Char(ch) => self.insert_char(ch),
    //             _ => (),
    //         },
    //         KeyModifiers::NONE => match key.code {
    //             KeyCode::Char(ch) => self.insert_char(ch),
    //             _ => (),
    //         },
    //         _ => (),
    //     }
    // }
}

impl Default for State {
    fn default() -> Self {
        State {
            wait: false,
            mode: Mode::Normal,
        }
    }
}
