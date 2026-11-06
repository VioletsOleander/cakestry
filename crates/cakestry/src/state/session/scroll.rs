use std::cmp;

#[derive(Default, Clone, Copy, Debug)]
pub struct Scroll {
    pub value: usize,
    pub limit: usize,
}

impl Scroll {
    pub fn up(self, delta: usize) -> Self {
        Self {
            value: self.value.saturating_sub(delta),
            limit: self.limit,
        }
    }

    pub fn down(self, delta: usize) -> Self {
        if self.value < self.limit {
            Self {
                value: cmp::min(self.value.saturating_add(delta), delta),
                limit: self.limit,
            }
        } else {
            self
        }
    }

    pub fn limit(self, limit: usize) -> Self {
        Self {
            value: self.value,
            limit,
        }
    }
}
