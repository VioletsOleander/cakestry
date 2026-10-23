mod paragraph;
mod prompt;
mod separator;

pub use paragraph::Paragraph;
pub use prompt::Prompt;
pub use separator::Separator;

pub trait Scroll {
    fn scroll(self, scroll: u16) -> Self;
}
