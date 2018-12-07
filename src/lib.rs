mod condition;
mod dripicon;
mod emoji;
mod moon;
mod weather_icon;

pub use crate::condition::Condition;
pub use crate::dripicon::DripIcon;
pub use crate::emoji::Emoji;
pub use crate::moon::{Moon, OutOfBounds, Style};
pub use crate::weather_icon::WeatherIcon;

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
