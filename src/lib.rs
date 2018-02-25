mod emoji;
mod moon;
mod weather_icon;
mod dripicon;
mod condition;

pub use condition::Condition;
pub use dripicon::DripIcon;
pub use emoji::Emoji;
pub use moon::{Moon, OutOfBounds, Style};
pub use weather_icon::WeatherIcon;

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
