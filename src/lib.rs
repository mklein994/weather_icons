mod moon;
mod weather_icon;
mod dripicon;
mod condition;

pub use condition::Condition;
pub use dripicon::DripIcon;
pub use weather_icon::WeatherIcon;
pub use moon::{Moon, OutOfBounds, Style};

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
