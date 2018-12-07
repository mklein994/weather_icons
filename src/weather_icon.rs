use super::dripicon::DripIcon;
use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WeatherIcon {
    DaySunny = 0xf00d,
    DayCloudy = 0xf002,
    DayCloudyGusts = 0xf000,
    DayCloudyWindy = 0xf001,
    DayFog = 0xf003,
    DayHail = 0xf004,
    DayHaze = 0xf0b6,
    DayLightning = 0xf005,
    DayRain = 0xf008,
    DayRainMix = 0xf006,
    DayRainWind = 0xf007,
    DayShowers = 0xf009,
    DaySleet = 0xf0b2,
    DaySleetStorm = 0xf068,
    DaySnow = 0xf00a,
    DaySnowThunderstorm = 0xf06b,
    DaySnowWind = 0xf065,
    DaySprinkle = 0xf00b,
    DayStormShowers = 0xf00e,
    DaySunnyOvercast = 0xf00c,
    DayThunderstorm = 0xf010,
    DayWindy = 0xf085,
    SolarEclipse = 0xf06e,
    Hot = 0xf072,
    DayCloudyHigh = 0xf07d,
    DayLightWind = 0xf0c4,
    NightClear = 0xf02e,
    NightAltCloudy = 0xf086,
    NightAltCloudyGusts = 0xf022,
    NightAltCloudyWindy = 0xf023,
    NightAltHail = 0xf024,
    NightAltLightning = 0xf025,
    NightAltRain = 0xf028,
    NightAltRainMix = 0xf026,
    NightAltRainWind = 0xf027,
    NightAltShowers = 0xf029,
    NightAltSleet = 0xf0b4,
    NightAltSleetStorm = 0xf06a,
    NightAltSnow = 0xf02a,
    NightAltSnowThunderstorm = 0xf06d,
    NightAltSnowWind = 0xf067,
    NightAltSprinkle = 0xf02b,
    NightAltStormShowers = 0xf02c,
    NightAltThunderstorm = 0xf02d,
    NightCloudy = 0xf031,
    NightCloudyGusts = 0xf02f,
    NightCloudyWindy = 0xf030,
    NightFog = 0xf04a,
    NightHail = 0xf032,
    NightLightning = 0xf033,
    NightPartlyCloudy = 0xf083,
    NightRain = 0xf036,
    NightRainMix = 0xf034,
    NightRainWind = 0xf035,
    NightShowers = 0xf037,
    NightSleet = 0xf0b3,
    NightSleetStorm = 0xf069,
    NightSnow = 0xf038,
    NightSnowThunderstorm = 0xf06c,
    NightSnowWind = 0xf066,
    NightSprinkle = 0xf039,
    NightStormShowers = 0xf03a,
    NightThunderstorm = 0xf03b,
    LunarEclipse = 0xf070,
    Stars = 0xf077,
    StormShowers = 0xf01d,
    Thunderstorm = 0xf01e,
    NightAltCloudyHigh = 0xf07e,
    NightCloudyHigh = 0xf080,
    NightAltPartlyCloudy = 0xf081,
    Cloud = 0xf041,
    Cloudy = 0xf013,
    CloudyGusts = 0xf011,
    CloudyWindy = 0xf012,
    Fog = 0xf014,
    Hail = 0xf015,
    Rain = 0xf019,
    RainMix = 0xf017,
    RainWind = 0xf018,
    Showers = 0xf01a,
    Sleet = 0xf0b5,
    Snow = 0xf01b,
    Sprinkle = 0xf01c,
    SnowWind = 0xf064,
    Smog = 0xf074,
    Smoke = 0xf062,
    Lightning = 0xf016,
    Raindrops = 0xf04e,
    Raindrop = 0xf078,
    Dust = 0xf063,
    SnowflakeCold = 0xf076,
    Windy = 0xf021,
    StrongWind = 0xf050,
    Sandstorm = 0xf082,
    Earthquake = 0xf0c6,
    Fire = 0xf0c7,
    Flood = 0xf07c,
    Meteor = 0xf071,
    Tsunami = 0xf0c5,
    Volcano = 0xf0c8,
    Hurricane = 0xf073,
    Tornado = 0xf056,
    SmallCraftAdvisory = 0xf0cc,
    GaleWarning = 0xf0cd,
    StormWarning = 0xf0ce,
    HurricaneWarning = 0xf0cf,
    WindDirection = 0xf0b1,
    Alien = 0xf075,
    Celsius = 0xf03c,
    Fahrenheit = 0xf045,
    Degrees = 0xf042,
    Thermometer = 0xf055,
    ThermometerExterior = 0xf053,
    ThermometerInternal = 0xf054,
    CloudDown = 0xf03d,
    CloudUp = 0xf040,
    CloudRefresh = 0xf03e,
    Horizon = 0xf047,
    HorizonAlt = 0xf046,
    Sunrise = 0xf051,
    Sunset = 0xf052,
    Moonrise = 0xf0c9,
    Moonset = 0xf0ca,
    Refresh = 0xf04c,
    RefreshAlt = 0xf04b,
    Umbrella = 0xf084,
    Barometer = 0xf079,
    Humidity = 0xf07a,
    Na = 0xf07b,
    Train = 0xf0cb,
    MoonNew = 0xf095,
    MoonWaxingCrescent1 = 0xf096,
    MoonWaxingCrescent2 = 0xf097,
    MoonWaxingCrescent3 = 0xf098,
    MoonWaxingCrescent4 = 0xf099,
    MoonWaxingCrescent5 = 0xf09a,
    MoonWaxingCrescent6 = 0xf09b,
    MoonFirstQuarter = 0xf09c,
    MoonWaxingGibbous1 = 0xf09d,
    MoonWaxingGibbous2 = 0xf09e,
    MoonWaxingGibbous3 = 0xf09f,
    MoonWaxingGibbous4 = 0xf0a0,
    MoonWaxingGibbous5 = 0xf0a1,
    MoonWaxingGibbous6 = 0xf0a2,
    MoonFull = 0xf0a3,
    MoonWaningGibbous1 = 0xf0a4,
    MoonWaningGibbous2 = 0xf0a5,
    MoonWaningGibbous3 = 0xf0a6,
    MoonWaningGibbous4 = 0xf0a7,
    MoonWaningGibbous5 = 0xf0a8,
    MoonWaningGibbous6 = 0xf0a9,
    MoonThirdQuarter = 0xf0aa,
    MoonWaningCrescent1 = 0xf0ab,
    MoonWaningCrescent2 = 0xf0ac,
    MoonWaningCrescent3 = 0xf0ad,
    MoonWaningCrescent4 = 0xf0ae,
    MoonWaningCrescent5 = 0xf0af,
    MoonWaningCrescent6 = 0xf0b0,
    MoonAltNew = 0xf0eb,
    MoonAltWaxingCrescent1 = 0xf0d0,
    MoonAltWaxingCrescent2 = 0xf0d1,
    MoonAltWaxingCrescent3 = 0xf0d2,
    MoonAltWaxingCrescent4 = 0xf0d3,
    MoonAltWaxingCrescent5 = 0xf0d4,
    MoonAltWaxingCrescent6 = 0xf0d5,
    MoonAltFirstQuarter = 0xf0d6,
    MoonAltWaxingGibbous1 = 0xf0d7,
    MoonAltWaxingGibbous2 = 0xf0d8,
    MoonAltWaxingGibbous3 = 0xf0d9,
    MoonAltWaxingGibbous4 = 0xf0da,
    MoonAltWaxingGibbous5 = 0xf0db,
    MoonAltWaxingGibbous6 = 0xf0dc,
    MoonAltFull = 0xf0dd,
    MoonAltWaningGibbous1 = 0xf0de,
    MoonAltWaningGibbous2 = 0xf0df,
    MoonAltWaningGibbous3 = 0xf0e0,
    MoonAltWaningGibbous4 = 0xf0e1,
    MoonAltWaningGibbous5 = 0xf0e2,
    MoonAltWaningGibbous6 = 0xf0e3,
    MoonAltThirdQuarter = 0xf0e4,
    MoonAltWaningCrescent1 = 0xf0e5,
    MoonAltWaningCrescent2 = 0xf0e6,
    MoonAltWaningCrescent3 = 0xf0e7,
    MoonAltWaningCrescent4 = 0xf0e8,
    MoonAltWaningCrescent5 = 0xf0e9,
    MoonAltWaningCrescent6 = 0xf0ea,
    /*
    Moon0 = 0xf095,
    Moon1 = 0xf096,
    Moon2 = 0xf097,
    Moon3 = 0xf098,
    Moon4 = 0xf099,
    Moon5 = 0xf09a,
    Moon6 = 0xf09b,
    Moon7 = 0xf09c,
    Moon8 = 0xf09d,
    Moon9 = 0xf09e,
    Moon10 = 0xf09f,
    Moon11 = 0xf0a0,
    Moon12 = 0xf0a1,
    Moon13 = 0xf0a2,
    Moon14 = 0xf0a3,
    Moon15 = 0xf0a4,
    Moon16 = 0xf0a5,
    Moon17 = 0xf0a6,
    Moon18 = 0xf0a7,
    Moon19 = 0xf0a8,
    Moon20 = 0xf0a9,
    Moon21 = 0xf0aa,
    Moon22 = 0xf0ab,
    Moon23 = 0xf0ac,
    Moon24 = 0xf0ad,
    Moon25 = 0xf0ae,
    Moon26 = 0xf0af,
    Moon27 = 0xf0b0,
    */
    Time1 = 0xf08a,
    Time2 = 0xf08b,
    Time3 = 0xf08c,
    Time4 = 0xf08d,
    Time5 = 0xf08e,
    Time6 = 0xf08f,
    Time7 = 0xf090,
    Time8 = 0xf091,
    Time9 = 0xf092,
    Time10 = 0xf093,
    Time11 = 0xf094,
    Time12 = 0xf089,
    DirectionUp = 0xf058,
    DirectionUpRight = 0xf057,
    DirectionRight = 0xf04d,
    DirectionDownRight = 0xf088,
    DirectionDown = 0xf044,
    DirectionDownLeft = 0xf043,
    DirectionLeft = 0xf048,
    DirectionUpLeft = 0xf087,
    WindBeaufort0 = 0xf0b7,
    WindBeaufort1 = 0xf0b8,
    WindBeaufort2 = 0xf0b9,
    WindBeaufort3 = 0xf0ba,
    WindBeaufort4 = 0xf0bb,
    WindBeaufort5 = 0xf0bc,
    WindBeaufort6 = 0xf0bd,
    WindBeaufort7 = 0xf0be,
    WindBeaufort8 = 0xf0bf,
    WindBeaufort9 = 0xf0c0,
    WindBeaufort10 = 0xf0c1,
    WindBeaufort11 = 0xf0c2,
    WindBeaufort12 = 0xf0c3,
}

impl fmt::Display for WeatherIcon {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            ::std::char::from_u32(*self as u32).expect("Could not format into char")
        )
    }
}

impl Default for WeatherIcon {
    fn default() -> Self {
        WeatherIcon::Na
    }
}

impl From<DripIcon> for WeatherIcon {
    fn from(icon: DripIcon) -> Self {
        use crate::WeatherIcon::*;
        match icon {
            DripIcon::Cloud => Cloud,
            DripIcon::CloudDrizzle => Showers,
            DripIcon::CloudDrizzleLightning => StormShowers,
            DripIcon::CloudDrizzleLightningMoon => NightAltStormShowers,
            DripIcon::CloudDrizzleLightningSun => DayStormShowers,
            DripIcon::CloudDrizzleLightningSunAlt => NightStormShowers,
            DripIcon::CloudDrizzleMoon => NightAltShowers,
            DripIcon::CloudDrizzleSun => DayShowers,
            DripIcon::CloudDrizzleSunAlt => NightShowers,
            DripIcon::CloudFog => Fog,
            DripIcon::CloudFogMoon => NightFog,
            DripIcon::CloudFogSun => DayFog,
            DripIcon::CloudHail => Hail,
            DripIcon::CloudHailMoon => NightAltHail,
            DripIcon::CloudHailSun => DayHail,
            DripIcon::CloudHailSunAlt => NightHail,
            DripIcon::CloudLightningMoon => NightAltLightning,
            DripIcon::CloudLightningSun => DayLightning,
            DripIcon::CloudLightningSunAlt => NightLightning,
            DripIcon::CloudMoon => NightAltCloudy,
            DripIcon::CloudRain => Rain,
            DripIcon::CloudRainAlt => Sprinkle,
            DripIcon::CloudRainAltMoon => NightAltSprinkle,
            DripIcon::CloudRainAltSun => DaySprinkle,
            DripIcon::CloudRainAltSunAlt => NightSprinkle,
            DripIcon::CloudRainLightning => Thunderstorm,
            DripIcon::CloudRainLightningMoon => NightAltThunderstorm,
            DripIcon::CloudRainLightningSun => DayThunderstorm,
            DripIcon::CloudRainLightningSunAlt => NightThunderstorm,
            DripIcon::CloudRainMoon => NightAltRain,
            DripIcon::CloudRainSun => DayRain,
            DripIcon::CloudRainSunAlt => NightRain,
            DripIcon::CloudSnow => Snow,
            DripIcon::CloudSnowMoon => NightAltSnow,
            DripIcon::CloudSnowSun => DaySnow,
            DripIcon::CloudSnowSunAlt => NightSnow,
            DripIcon::CloudSun => DayCloudy,
            DripIcon::CloudSunAlt => NightCloudy,
            DripIcon::CloudWind => CloudyGusts,
            DripIcon::CloudWindAlt => CloudyWindy,
            DripIcon::CloudWindAltMoon => NightAltCloudyWindy,
            DripIcon::CloudWindAltSun => DayCloudyWindy,
            DripIcon::CloudWindAltSunAlt => NightCloudyWindy,
            DripIcon::CloudWindMoon => NightAltCloudyGusts,
            DripIcon::CloudWindSunAlt => NightCloudyGusts,
            DripIcon::Clouds => Cloudy,
            DripIcon::CompassEast => DirectionRight,
            DripIcon::CompassNorth => DirectionUp,
            DripIcon::CompassSouth => DirectionDown,
            DripIcon::CompassWest => DirectionLeft,
            DripIcon::DegreesCelsius => Celsius,
            DripIcon::DegreesFahrenheit => Fahrenheit,
            DripIcon::Flag => SmallCraftAdvisory,
            DripIcon::Fog => Windy,
            DripIcon::Lightning => Lightning,
            DripIcon::Raindrop => Raindrop,
            DripIcon::Snow => SnowflakeCold,
            DripIcon::Sun => DaySunny,
            DripIcon::SunEclipse => SolarEclipse,
            DripIcon::SunLow => HorizonAlt,
            DripIcon::SunLower => Horizon,
            DripIcon::SunRise => Sunrise,
            DripIcon::SunSet => Sunset,
            DripIcon::Thermometer100 => Thermometer,
            DripIcon::Tornado => Tornado,
            DripIcon::Umbrella => Umbrella,
            DripIcon::Wet => Raindrops,
            DripIcon::Wind => StrongWind,
            _ => unimplemented!(),
        }
    }
}
