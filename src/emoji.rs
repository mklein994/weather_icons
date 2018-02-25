use std::fmt;

#[derive(Debug, Copy, Clone)]
pub enum Emoji {
    /// Cityscape with a clear sky.
    Cityscape = 0x1f3d9,
    /// Cityscape in fog.
    Foggy = 0x1f301,
    /// Cityscape at night.
    NightWithStars = 0x1f303,
    /// Cityscape at dusk.
    CityscapeAtDusk = 0x1f306,
    /// Cityscape at sunset.
    Sunset = 0x1f307,
    /// 1/8
    NewMoon = 0x1f311,
    /// 2/8
    WaxingCrescentMoon = 0x1f312,
    /// 3/8
    FirstQuarterMoon = 0x1f313,
    /// 4/8
    WaxingGibbousMoon = 0x1f314,
    /// 5/8
    FullMoon = 0x1f315,
    /// 6/8
    WaningGibbousMoon = 0x1f316,
    /// 7/8
    LastQuarterMoon = 0x1f317,
    /// 8/8
    WaningCrescentMoon = 0x1f318,
    CrescentMoon = 0x1f319,
    Thermometer = 0x1f321,
    Sun = 0x2600,
    Cloud = 0x2601,
    SunBehindCloud = 0x26c5,
    CloudWithLightningAndRain = 0x26c8,
    SunBehindSmallCloud = 0x1f324,
    SunBehindLargeCloud = 0x1f325,
    SunBehindRainCloud = 0x1f326,
    CloudWithRain = 0x1f327,
    CloudWithSnow = 0x1f328,
    CloudWithLightning = 0x1f329,
    Tornado = 0x1f32a,
    Fog = 0x1f32b,
    Snowflake = 0x2744,
    Fire = 0x1f525,
    Droplet = 0x1f4a7,
    WaterWave = 0x1f30a,
}

impl fmt::Display for Emoji {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use std::char;
        write!(
            f,
            "{}",
            char::from_u32(*self as u32).expect("Couldn't convert emoji into an icon")
        )
    }
}
