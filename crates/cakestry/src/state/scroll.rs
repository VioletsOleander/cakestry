use std::cmp;

#[derive(Default, Clone, Copy, Debug)]
pub struct Scroll {
    pub value: usize,
    pub limit: usize,
}

impl Scroll {
    pub fn up(self, value: usize) -> Self {
        Self {
            value: self.value.saturating_sub(value),
            limit: self.limit,
        }
    }

    pub fn down(self, value: usize) -> Self {
        if self.value < self.limit {
            Self {
                value: cmp::min(self.value.saturating_add(value), value),
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
