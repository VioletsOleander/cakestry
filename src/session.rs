pub mod state;

#[derive(Default)]
pub struct Session {
    state: state::State,
}

impl Session {
    pub fn state(&self) -> &state::State {
        &self.state
    }

    pub fn state_mut(&mut self) -> &mut state::State {
        &mut self.state
    }
}
