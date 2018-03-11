use super::weather_icon::WeatherIcon;
use std::fmt;

#[derive(Debug, Copy, Clone)]
pub enum DripIcon {
    /// a
    Cloud = 0x61,
    /// b
    CloudDrizzle = 0x62,
    /// c
    CloudDrizzleLightning = 0x63,
    /// d
    CloudDrizzleLightningMoon = 0x64,
    /// e
    CloudDrizzleLightningSun = 0x65,
    /// f
    CloudDrizzleLightningSunAlt = 0x66,
    /// g
    CloudDrizzleMoon = 0x67,
    /// h
    CloudDrizzleSun = 0x68,
    /// i
    CloudDrizzleSunAlt = 0x69,
    /// j
    CloudFog = 0x6a,
    /// k
    CloudFogAlt = 0x6b,
    /// l
    CloudFogMoon = 0x6c,
    /// m
    CloudFogSun = 0x6d,
    /// n
    CloudFogSunAlt = 0x6e,
    /// o
    CloudHail = 0x6f,
    /// p
    CloudHailLightning = 0x70,
    /// q
    CloudHailLightningMoon = 0x71,
    /// r
    CloudHailLightningSun = 0x72,
    /// s
    CloudHailLightningSunAlt = 0x73,
    /// t
    CloudHailMoon = 0x74,
    /// u
    CloudHailSun = 0x75,
    /// v
    CloudHailSunAlt = 0x76,
    /// w
    CloudLightning = 0x77,
    /// x
    CloudLightningMoon = 0x78,
    /// y
    CloudLightningSun = 0x79,
    /// z
    CloudLightningSunAlt = 0x7a,
    /// A
    CloudMoon = 0x41,
    /// B
    CloudRain = 0x42,
    /// C
    CloudRainAlt = 0x43,
    /// D
    CloudRainAltMoon = 0x44,
    /// E
    CloudRainAltSun = 0x45,
    /// F
    CloudRainAltSunAlt = 0x46,
    /// G
    CloudRainLightning = 0x47,
    /// H
    CloudRainLightningMoon = 0x48,
    /// I
    CloudRainLightningSun = 0x49,
    /// J
    CloudRainLightningSunAlt = 0x4a,
    /// K
    CloudRainMoon = 0x4b,
    /// L
    CloudRainSun = 0x4c,
    /// M
    CloudRainSunAlt = 0x4d,
    /// N
    CloudSnow = 0x4e,
    /// O
    CloudSnowMoon = 0x4f,
    /// P
    CloudSnowSun = 0x50,
    /// Q
    CloudSnowSunAlt = 0x51,
    /// R
    CloudSun = 0x52,
    /// S
    CloudSunAlt = 0x53,
    /// T
    CloudWind = 0x54,
    /// U
    CloudWindAlt = 0x55,
    /// V
    CloudWindAltMoon = 0x56,
    /// W
    CloudWindAltSun = 0x57,
    /// X
    CloudWindAltSunAlt = 0x58,
    /// Y
    CloudWindMoon = 0x59,
    /// Z
    CloudWindSun = 0x5a,
    /// 0
    CloudWindSunAlt = 0x30,
    /// 1
    Clouds = 0x31,
    /// 2
    CloudsMoon = 0x32,
    /// 3
    CloudsSun = 0x33,
    /// 4
    CloudsSunAlt = 0x34,
    /// 5
    CompassEast = 0x35,
    /// 6
    CompassNorth = 0x36,
    /// 7
    CompassSouth = 0x37,
    /// 8
    CompassWest = 0x38,
    /// 9
    DegreesCelsius = 0x39,
    /// !
    DegreesFahrenheit = 0x21,
    /// "
    Drizzle = 0x22,
    /// \#
    Flag = 0x23,
    /// $
    Fog = 0x24,
    /// %
    Hail = 0x25,
    /// &
    Lightning = 0x26,
    /// '
    Moon25 = 0x27,
    /// (
    Moon50 = 0x28,
    /// )
    Moon75 = 0x29,
    /// \*
    Moon100 = 0x2a,
    /// \+
    MoonStars = 0x2b,
    /// ,
    Rain = 0x2c,
    /// \-
    Raindrop = 0x2d,
    /// .
    Snow = 0x2e,
    /// /
    Sun = 0x2f,
    /// :
    SunEclipse = 0x3a,
    /// ;
    SunLow = 0x3b,
    /// <
    SunLower = 0x3c,
    /// \=
    SunRise = 0x3d,
    /// >
    SunSet = 0x3e,
    /// ?
    Thermometer25 = 0x3f,
    /// @
    Thermometer50 = 0x40,
    /// [
    Thermometer75 = 0x5b,
    /// ]
    Thermometer100 = 0x5d,
    /// ^
    Tornado = 0x5e,
    /// _
    Umbrella = 0x5f,
    /// `
    UmbrellaDrizzle = 0x60,
    /// {
    Wet = 0x7b,
    /// |
    Wind = 0x7c,
}

impl fmt::Display for DripIcon {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", ::std::char::from_u32(*self as u32).unwrap())
    }
}

impl Default for DripIcon {
    fn default() -> Self {
        DripIcon::Cloud
    }
}

impl From<WeatherIcon> for DripIcon {
    fn from(icon: WeatherIcon) -> Self {
        use DripIcon::*;
        match icon {
            WeatherIcon::Cloud => Cloud,
            WeatherIcon::Showers => CloudDrizzle,
            WeatherIcon::StormShowers => CloudDrizzleLightning,
            WeatherIcon::NightAltStormShowers => CloudDrizzleLightningMoon,
            WeatherIcon::DayStormShowers => CloudDrizzleLightningSun,
            WeatherIcon::NightStormShowers => CloudDrizzleLightningSunAlt,
            WeatherIcon::NightAltShowers => CloudDrizzleMoon,
            WeatherIcon::DayShowers => CloudDrizzleSun,
            WeatherIcon::NightShowers => CloudDrizzleSunAlt,
            WeatherIcon::Fog => CloudFog,
            WeatherIcon::NightFog => CloudFogMoon,
            WeatherIcon::DayFog => CloudFogSun,
            WeatherIcon::Hail => CloudHail,
            WeatherIcon::NightAltHail => CloudHailMoon,
            WeatherIcon::DayHail => CloudHailSun,
            WeatherIcon::NightHail => CloudHailSunAlt,
            WeatherIcon::NightAltLightning => CloudLightningMoon,
            WeatherIcon::DayLightning => CloudLightningSun,
            WeatherIcon::NightLightning => CloudLightningSunAlt,
            WeatherIcon::NightAltCloudy => CloudMoon,
            WeatherIcon::Rain => CloudRain,
            WeatherIcon::Sprinkle => CloudRainAlt,
            WeatherIcon::NightAltSprinkle => CloudRainAltMoon,
            WeatherIcon::DaySprinkle => CloudRainAltSun,
            WeatherIcon::NightSprinkle => CloudRainAltSunAlt,
            WeatherIcon::Thunderstorm => CloudRainLightning,
            WeatherIcon::NightAltThunderstorm => CloudRainLightningMoon,
            WeatherIcon::DayThunderstorm => CloudRainLightningSun,
            WeatherIcon::NightThunderstorm => CloudRainLightningSunAlt,
            WeatherIcon::NightAltRain => CloudRainMoon,
            WeatherIcon::DayRain => CloudRainSun,
            WeatherIcon::NightRain => CloudRainSunAlt,
            WeatherIcon::Snow => CloudSnow,
            WeatherIcon::NightAltSnow => CloudSnowMoon,
            WeatherIcon::DaySnow => CloudSnowSun,
            WeatherIcon::NightSnow => CloudSnowSunAlt,
            WeatherIcon::DayCloudy => CloudSun,
            WeatherIcon::NightCloudy => CloudSunAlt,
            WeatherIcon::CloudyGusts => CloudWind,
            WeatherIcon::CloudyWindy => CloudWindAlt,
            WeatherIcon::NightAltCloudyWindy => CloudWindAltMoon,
            WeatherIcon::DayCloudyWindy => CloudWindAltSun,
            WeatherIcon::NightCloudyWindy => CloudWindAltSunAlt,
            WeatherIcon::NightAltCloudyGusts => CloudWindMoon,
            WeatherIcon::NightCloudyGusts => CloudWindSunAlt,
            WeatherIcon::Cloudy => Clouds,
            WeatherIcon::DirectionRight => CompassEast,
            WeatherIcon::DirectionUp => CompassNorth,
            WeatherIcon::DirectionDown => CompassSouth,
            WeatherIcon::DirectionLeft => CompassWest,
            WeatherIcon::Celsius => DegreesCelsius,
            WeatherIcon::Fahrenheit => DegreesFahrenheit,
            WeatherIcon::SmallCraftAdvisory => Flag,
            WeatherIcon::Windy => Fog,
            WeatherIcon::Lightning => Lightning,
            WeatherIcon::Raindrop => Raindrop,
            WeatherIcon::SnowflakeCold => Snow,
            WeatherIcon::DaySunny => Sun,
            WeatherIcon::SolarEclipse => SunEclipse,
            WeatherIcon::HorizonAlt => SunLow,
            WeatherIcon::Horizon => SunLower,
            WeatherIcon::Sunrise => SunRise,
            WeatherIcon::Sunset => SunSet,
            WeatherIcon::Thermometer => Thermometer100,
            WeatherIcon::Tornado => Tornado,
            WeatherIcon::Umbrella => Umbrella,
            WeatherIcon::Raindrops => Wet,
            WeatherIcon::StrongWind => Wind,
            WeatherIcon::NightPartlyCloudy => CloudsSunAlt,
            WeatherIcon::DaySunnyOvercast => CloudsSun,
            WeatherIcon::NightClear => Moon75,
            _ => unimplemented!(),
        }
    }
}
