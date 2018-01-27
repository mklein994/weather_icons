use super::{Icon, Time};

/// `Neutral`, `Day`, `Night`, `NightAlt`
type IconSet = (Option<Icon>, Option<Icon>, Option<Icon>, Option<Icon>);

#[cfg_attr(rustfmt, rustfmt_skip)]
static WEATHER_ICONS: &'static [IconSet] = &[
    (Some(Icon::CloudyWindy),   Some(Icon::DayCloudyWindy),       Some(Icon::NightCloudyWindy),       Some(Icon::NightAltCloudyWindy)),
    (Some(Icon::Cloudy),        Some(Icon::DayCloudy),            Some(Icon::NightCloudy),            Some(Icon::NightAltCloudy)),
    (Some(Icon::Fog),           Some(Icon::DayFog),               Some(Icon::NightFog),               None),
    (Some(Icon::Hail),          Some(Icon::DayHail),              Some(Icon::NightHail),              Some(Icon::NightAltHail)),
    (Some(Icon::Lightning),     Some(Icon::DayLightning),         Some(Icon::NightLightning),         Some(Icon::NightAltLightning)),
    (Some(Icon::RainMix),       Some(Icon::DayRainMix),           Some(Icon::NightRainMix),           Some(Icon::NightAltRainMix)),
    (Some(Icon::RainWind),      Some(Icon::DayRainWind),          Some(Icon::NightRainWind),          Some(Icon::NightAltRainWind)),
    (Some(Icon::Rain),          Some(Icon::DayRain),              Some(Icon::NightRain),              Some(Icon::NightAltRain)),
    (Some(Icon::Showers),       Some(Icon::DayShowers),           Some(Icon::NightShowers),           Some(Icon::NightAltShowers)),
    (Some(Icon::Snow),          Some(Icon::DaySnow),              Some(Icon::NightSnow),              Some(Icon::NightAltSnow)),
    (Some(Icon::Sprinkle),      Some(Icon::DaySprinkle),          Some(Icon::NightSprinkle),          Some(Icon::NightAltSprinkle)),
    (None,                      Some(Icon::DaySunnyOvercast),     Some(Icon::NightPartlyCloudy),      Some(Icon::NightAltPartlyCloudy)),
    (None,                      Some(Icon::DaySunny),             Some(Icon::NightClear),             None),
    (Some(Icon::StormShowers),  Some(Icon::DayStormShowers),      Some(Icon::NightStormShowers),      Some(Icon::NightAltStormShowers)),
    (Some(Icon::Thunderstorm),  Some(Icon::DayThunderstorm),      Some(Icon::NightThunderstorm),      Some(Icon::NightAltThunderstorm)),
    (Some(Icon::SnowWind),      Some(Icon::DaySnowWind),          Some(Icon::NightSnowWind),          Some(Icon::NightAltSnowWind)),
    (None,                      Some(Icon::DaySleetStorm),        Some(Icon::NightSleetStorm),        Some(Icon::NightAltSleetStorm)),
    (None,                      Some(Icon::DaySnowThunderstorm),  Some(Icon::NightSnowThunderstorm),  Some(Icon::NightAltSnowThunderstorm)),
    (None,                      Some(Icon::DayCloudyHigh),        Some(Icon::NightCloudyHigh),        Some(Icon::NightAltCloudyHigh)),
    //(Some(Icon::Windy),         Some(Icon::DayWindy),             None,                               None),
    (Some(Icon::Sleet),         Some(Icon::DaySleet),             Some(Icon::NightSleet),             Some(Icon::NightAltSleet)),
    //(None,                      Some(Icon::DayHaze),              None,                               None),
    //(None,                      Some(Icon::DayLightWind),         None,                               None),
    //(None,                      None,                             Some(Icon::NightCloudyGusts),       None),
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
    pub fn variant(&self, time: Time) -> Option<Icon> {
        use Time::*;
        match time {
            Neutral => WEATHER_ICONS[*self as usize].0,
            Day => WEATHER_ICONS[*self as usize].1,
            Night => WEATHER_ICONS[*self as usize].2,
            NightAlt => WEATHER_ICONS[*self as usize].3,
        }
    }

    pub fn neutral(&self) -> Option<Icon> {
        WEATHER_ICONS[*self as usize].0
    }

    pub fn day(&self) -> Option<Icon> {
        WEATHER_ICONS[*self as usize].1
    }

    pub fn night(&self) -> Option<Icon> {
        WEATHER_ICONS[*self as usize].2
    }

    pub fn night_alt(&self) -> Option<Icon> {
        WEATHER_ICONS[*self as usize].3
    }
}

impl Default for Condition {
    fn default() -> Self {
        Condition::Fair
    }
}
