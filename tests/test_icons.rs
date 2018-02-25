extern crate weather_icons;

use weather_icons::Condition;

#[test]
fn check_icon_condition_mapping_day() {
    use weather_icons::WeatherIcon::*;

    assert_eq!(Condition::CloudyWindy.day(), Some(DayCloudyWindy));
    assert_eq!(Condition::Cloudy.day(), Some(DayCloudy));
    assert_eq!(Condition::Fog.day(), Some(DayFog));
    assert_eq!(Condition::Hail.day(), Some(DayHail));
    assert_eq!(Condition::Lightning.day(), Some(DayLightning));
    assert_eq!(Condition::RainMix.day(), Some(DayRainMix));
    assert_eq!(Condition::RainWind.day(), Some(DayRainWind));
    assert_eq!(Condition::Rain.day(), Some(DayRain));
    assert_eq!(Condition::Showers.day(), Some(DayShowers));
    assert_eq!(Condition::Snow.day(), Some(DaySnow));
    assert_eq!(Condition::Sprinkle.day(), Some(DaySprinkle));
    assert_eq!(Condition::PartlyCloudy.day(), Some(DaySunnyOvercast));
    assert_eq!(Condition::Fair.day(), Some(DaySunny));
    assert_eq!(Condition::StormShowers.day(), Some(DayStormShowers));
    assert_eq!(Condition::Thunderstorm.day(), Some(DayThunderstorm));
    assert_eq!(Condition::SnowWind.day(), Some(DaySnowWind));
    assert_eq!(Condition::SleetStorm.day(), Some(DaySleetStorm));
    assert_eq!(Condition::SnowThunderstorm.day(), Some(DaySnowThunderstorm));
    assert_eq!(Condition::CloudyHigh.day(), Some(DayCloudyHigh));
    //Windy,
    assert_eq!(Condition::Sleet.day(), Some(DaySleet));
    //Haze,
    //LightWind,
    //CloudyGusts,
}

#[test]
fn check_icon_condition_mapping_night() {
    use weather_icons::WeatherIcon::*;

    assert_eq!(Condition::CloudyWindy.night(), Some(NightCloudyWindy));
    assert_eq!(Condition::Cloudy.night(), Some(NightCloudy));
    assert_eq!(Condition::Fog.night(), Some(NightFog));
    assert_eq!(Condition::Hail.night(), Some(NightHail));
    assert_eq!(Condition::Lightning.night(), Some(NightLightning));
    assert_eq!(Condition::RainMix.night(), Some(NightRainMix));
    assert_eq!(Condition::RainWind.night(), Some(NightRainWind));
    assert_eq!(Condition::Rain.night(), Some(NightRain));
    assert_eq!(Condition::Showers.night(), Some(NightShowers));
    assert_eq!(Condition::Snow.night(), Some(NightSnow));
    assert_eq!(Condition::Sprinkle.night(), Some(NightSprinkle));
    assert_eq!(Condition::PartlyCloudy.night(), Some(NightPartlyCloudy));
    assert_eq!(Condition::Fair.night(), Some(NightClear));
    assert_eq!(Condition::StormShowers.night(), Some(NightStormShowers));
    assert_eq!(Condition::Thunderstorm.night(), Some(NightThunderstorm));
    assert_eq!(Condition::SnowWind.night(), Some(NightSnowWind));
    assert_eq!(Condition::SleetStorm.night(), Some(NightSleetStorm));
    assert_eq!(
        Condition::SnowThunderstorm.night(),
        Some(NightSnowThunderstorm)
    );
    assert_eq!(Condition::CloudyHigh.night(), Some(NightCloudyHigh));
    //Windy,
    assert_eq!(Condition::Sleet.night(), Some(NightSleet));
    //Haze,
    //LightWind,
    //CloudyGusts,
}
