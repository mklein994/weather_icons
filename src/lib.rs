mod moon;
mod icon;
mod condition;

pub use condition::Condition;
pub use icon::Icon;
pub use moon::{Moon, MoonBuilder, Style};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Time {
    Neutral,
    Day,
    Night,
    NightAlt,
}

impl Default for Time {
    fn default() -> Self {
        Time::Neutral
    }
}
