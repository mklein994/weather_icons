use super::icon::Icon;
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

impl From<Icon> for DripIcon {
    fn from(icon: Icon) -> Self {
        use DripIcon::*;
        match icon {
            Icon::Cloud => Cloud,
            Icon::Showers => CloudDrizzle,
            Icon::StormShowers => CloudDrizzleLightning,
            Icon::NightAltStormShowers => CloudDrizzleLightningMoon,
            Icon::DayStormShowers => CloudDrizzleLightningSun,
            Icon::NightStormShowers => CloudDrizzleLightningSunAlt,
            Icon::NightAltShowers => CloudDrizzleMoon,
            Icon::DayShowers => CloudDrizzleSun,
            Icon::NightShowers => CloudDrizzleSunAlt,
            Icon::Fog => CloudFog,
            Icon::NightFog => CloudFogMoon,
            Icon::DayFog => CloudFogSun,
            Icon::Hail => CloudHail,
            Icon::NightAltHail => CloudHailMoon,
            Icon::DayHail => CloudHailSun,
            Icon::NightHail => CloudHailSunAlt,
            Icon::NightAltLightning => CloudLightningMoon,
            Icon::DayLightning => CloudLightningSun,
            Icon::NightLightning => CloudLightningSunAlt,
            Icon::NightAltCloudy => CloudMoon,
            Icon::Rain => CloudRain,
            Icon::Sprinkle => CloudRainAlt,
            Icon::NightAltSprinkle => CloudRainAltMoon,
            Icon::DaySprinkle => CloudRainAltSun,
            Icon::NightSprinkle => CloudRainAltSunAlt,
            Icon::Thunderstorm => CloudRainLightning,
            Icon::NightAltThunderstorm => CloudRainLightningMoon,
            Icon::DayThunderstorm => CloudRainLightningSun,
            Icon::NightThunderstorm => CloudRainLightningSunAlt,
            Icon::NightAltRain => CloudRainMoon,
            Icon::DayRain => CloudRainSun,
            Icon::NightRain => CloudRainSunAlt,
            Icon::Snow => CloudSnow,
            Icon::NightAltSnow => CloudSnowMoon,
            Icon::DaySnow => CloudSnowSun,
            Icon::NightSnow => CloudSnowSunAlt,
            Icon::DayCloudy => CloudSun,
            Icon::NightCloudy => CloudSunAlt,
            Icon::CloudyGusts => CloudWind,
            Icon::CloudyWindy => CloudWindAlt,
            Icon::NightAltCloudyWindy => CloudWindAltMoon,
            Icon::DayCloudyWindy => CloudWindAltSun,
            Icon::NightCloudyWindy => CloudWindAltSunAlt,
            Icon::NightAltCloudyGusts => CloudWindMoon,
            Icon::NightCloudyGusts => CloudWindSunAlt,
            Icon::Cloudy => Clouds,
            Icon::DirectionRight => CompassEast,
            Icon::DirectionUp => CompassNorth,
            Icon::DirectionDown => CompassSouth,
            Icon::DirectionLeft => CompassWest,
            Icon::Celsius => DegreesCelsius,
            Icon::Fahrenheit => DegreesFahrenheit,
            Icon::SmallCraftAdvisory => Flag,
            Icon::Windy => Fog,
            Icon::Lightning => Lightning,
            Icon::Raindrop => Raindrop,
            Icon::SnowflakeCold => Snow,
            Icon::DaySunny => Sun,
            Icon::SolarEclipse => SunEclipse,
            Icon::HorizonAlt => SunLow,
            Icon::Horizon => SunLower,
            Icon::Sunrise => SunRise,
            Icon::Sunset => SunSet,
            Icon::Thermometer => Thermometer100,
            Icon::Tornado => Tornado,
            Icon::Umbrella => Umbrella,
            Icon::Raindrops => Wet,
            Icon::StrongWind => Wind,
            Icon::NightPartlyCloudy => CloudsSunAlt,
            Icon::DaySunnyOvercast => CloudsSun,
            Icon::NightClear => Moon75,
            _ => unimplemented!(),
        }
    }
}
