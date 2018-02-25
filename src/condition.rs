use super::{Time, WeatherIcon};

type IconSet = (
    // Neutral
    Option<WeatherIcon>,
    // Day
    Option<WeatherIcon>,
    // Night
    Option<WeatherIcon>,
    // NightAlt
    Option<WeatherIcon>,
);

#[cfg_attr(rustfmt, rustfmt_skip)]
static WEATHER_ICONS: &'static [IconSet] = &[
    (Some(WeatherIcon::CloudyWindy),   Some(WeatherIcon::DayCloudyWindy),       Some(WeatherIcon::NightCloudyWindy),       Some(WeatherIcon::NightAltCloudyWindy)),
    (Some(WeatherIcon::Cloudy),        Some(WeatherIcon::DayCloudy),            Some(WeatherIcon::NightCloudy),            Some(WeatherIcon::NightAltCloudy)),
    (Some(WeatherIcon::Fog),           Some(WeatherIcon::DayFog),               Some(WeatherIcon::NightFog),               None),
    (Some(WeatherIcon::Hail),          Some(WeatherIcon::DayHail),              Some(WeatherIcon::NightHail),              Some(WeatherIcon::NightAltHail)),
    (Some(WeatherIcon::Lightning),     Some(WeatherIcon::DayLightning),         Some(WeatherIcon::NightLightning),         Some(WeatherIcon::NightAltLightning)),
    (Some(WeatherIcon::RainMix),       Some(WeatherIcon::DayRainMix),           Some(WeatherIcon::NightRainMix),           Some(WeatherIcon::NightAltRainMix)),
    (Some(WeatherIcon::RainWind),      Some(WeatherIcon::DayRainWind),          Some(WeatherIcon::NightRainWind),          Some(WeatherIcon::NightAltRainWind)),
    (Some(WeatherIcon::Rain),          Some(WeatherIcon::DayRain),              Some(WeatherIcon::NightRain),              Some(WeatherIcon::NightAltRain)),
    (Some(WeatherIcon::Showers),       Some(WeatherIcon::DayShowers),           Some(WeatherIcon::NightShowers),           Some(WeatherIcon::NightAltShowers)),
    (Some(WeatherIcon::Snow),          Some(WeatherIcon::DaySnow),              Some(WeatherIcon::NightSnow),              Some(WeatherIcon::NightAltSnow)),
    (Some(WeatherIcon::Sprinkle),      Some(WeatherIcon::DaySprinkle),          Some(WeatherIcon::NightSprinkle),          Some(WeatherIcon::NightAltSprinkle)),
    (None,                      Some(WeatherIcon::DaySunnyOvercast),     Some(WeatherIcon::NightPartlyCloudy),      Some(WeatherIcon::NightAltPartlyCloudy)),
    (None,                      Some(WeatherIcon::DaySunny),             Some(WeatherIcon::NightClear),             None),
    (Some(WeatherIcon::StormShowers),  Some(WeatherIcon::DayStormShowers),      Some(WeatherIcon::NightStormShowers),      Some(WeatherIcon::NightAltStormShowers)),
    (Some(WeatherIcon::Thunderstorm),  Some(WeatherIcon::DayThunderstorm),      Some(WeatherIcon::NightThunderstorm),      Some(WeatherIcon::NightAltThunderstorm)),
    (Some(WeatherIcon::SnowWind),      Some(WeatherIcon::DaySnowWind),          Some(WeatherIcon::NightSnowWind),          Some(WeatherIcon::NightAltSnowWind)),
    (None,                      Some(WeatherIcon::DaySleetStorm),        Some(WeatherIcon::NightSleetStorm),        Some(WeatherIcon::NightAltSleetStorm)),
    (None,                      Some(WeatherIcon::DaySnowThunderstorm),  Some(WeatherIcon::NightSnowThunderstorm),  Some(WeatherIcon::NightAltSnowThunderstorm)),
    (None,                      Some(WeatherIcon::DayCloudyHigh),        Some(WeatherIcon::NightCloudyHigh),        Some(WeatherIcon::NightAltCloudyHigh)),
    //(Some(WeatherIcon::Windy),         Some(WeatherIcon::DayWindy),             None,                               None),
    (Some(WeatherIcon::Sleet),         Some(WeatherIcon::DaySleet),             Some(WeatherIcon::NightSleet),             Some(WeatherIcon::NightAltSleet)),
    //(None,                      Some(WeatherIcon::DayHaze),              None,                               None),
    //(None,                      Some(WeatherIcon::DayLightWind),         None,                               None),
    //(None,                      None,                             Some(WeatherIcon::NightCloudyGusts),       None),
];

#[derive(Clone, Copy, Debug)]
pub enum Condition {
    CloudyWindy,
    Cloudy,
    Fog,
    Hail,
    Lightning,
    RainMix,
    RainWind,
    Rain,
    Showers,
    Snow,
    Sprinkle,
    PartlyCloudy,
    Fair,
    StormShowers,
    Thunderstorm,
    SnowWind,
    SleetStorm,
    SnowThunderstorm,
    CloudyHigh,
    //Windy,
    Sleet,
    //Haze,
    //LightWind,
    //CloudyGusts,
}

impl Condition {
    pub fn variant(&self, time: Time) -> Option<WeatherIcon> {
        use Time::*;
        match time {
            Neutral => WEATHER_ICONS[*self as usize].0,
            Day => WEATHER_ICONS[*self as usize].1,
            Night => WEATHER_ICONS[*self as usize].2,
            NightAlt => WEATHER_ICONS[*self as usize].3,
        }
    }

    pub fn neutral(&self) -> Option<WeatherIcon> {
        WEATHER_ICONS[*self as usize].0
    }

    pub fn day(&self) -> Option<WeatherIcon> {
        WEATHER_ICONS[*self as usize].1
    }

    pub fn night(&self) -> Option<WeatherIcon> {
        WEATHER_ICONS[*self as usize].2
    }

    pub fn night_alt(&self) -> Option<WeatherIcon> {
        WEATHER_ICONS[*self as usize].3
    }
}

impl Default for Condition {
    fn default() -> Self {
        Condition::Fair
    }
}
