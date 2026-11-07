use std::cmp;

#[derive(Default, Clone, Copy, Debug)]
pub struct Scroll {
    pub value: usize,
    pub limit: usize,
}

impl Scroll {
    pub fn up(&mut self, delta: usize) {
        self.value = self.value.saturating_sub(delta);
    }

    pub fn down(&mut self, delta: usize) {
        self.value = cmp::min(self.value.saturating_add(delta), self.limit);
    }

    pub fn set_limit(&mut self, limit: usize) {
        self.limit = limit;
    }
}
