use super::WeatherIcon;
use std::char;
use std::fmt;

const MOON_PHASES: f64 = 28.;

/// The moons are split into 28 icons, to correspond neatly with the 28 day moon cycle. There is a
/// primary set and alternate set. The primary set is meant to be interpreted as: where there are
/// pixels, that is the illuminated part of the moon. The alternate set is meant to be interpreted
/// as: where there are pixels, that is the shadowed part of the moon.
///
/// *From
/// [https://erikflowers.github.io/weather-icons/](https://erikflowers.github.io/weather-icons/)*
#[derive(Clone, Copy, Debug)]
pub enum Style {
    /// '\u{f0eb}', i.e. wi-moon-alt-new
    ///
    /// A full moon has no pixels.
    Alt = WeatherIcon::MoonAltNew as isize, // 0xf0eb,
    /// '\u{f095}', i.e. wi-moon-new
    ///
    /// A full moon is filled with pixels.
    Primary = WeatherIcon::MoonNew as isize, // 0xf095,
}

impl Default for Style {
    fn default() -> Self {
        Style::Primary
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Moon {
    pub phase: u32,
    pub style: Style,
    icon: char,
}

impl Moon {
    pub fn new(phase: f64, style: Style) -> Result<Self, OutOfBounds> {
        let phase: u32 = Self::moon_phase(phase)?;

        let icon = Self::moon_icon(phase, style);

        Ok(Self { phase, style, icon })
    }

    fn moon_phase(phase: f64) -> Result<u32, OutOfBounds> {
        if phase > 1. || phase < 0. {
            Err(OutOfBounds)
        } else {
            Ok(((phase * MOON_PHASES).round() % MOON_PHASES) as u32)
        }
    }

    fn moon_icon(phase: u32, style: Style) -> char {
        let icon: u32 = match style {
            Style::Primary => style as u32 + phase,
            Style::Alt => {
                if phase == 0 {
                    Style::Alt as u32
                } else {
                    WeatherIcon::MoonAltWaxingCrescent1 as u32 - 1 + phase
                }
            }
        };

        char::from_u32(icon).expect("couldn't convert moon phase to char")
    }
}

impl fmt::Display for Moon {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.icon)
    }
}

#[derive(Debug)]
pub struct OutOfBounds;

impl fmt::Display for OutOfBounds {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "moon phase is out of bounds")
    }
}

impl ::std::error::Error for OutOfBounds {
    fn description(&self) -> &str {
        "The moon phase must be between 0 and 1"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_MOON_PHASES: [(f64, i32, WeatherIcon, WeatherIcon); 101] = [
        (0.00, 0, WeatherIcon::MoonNew, WeatherIcon::MoonAltNew),
        (0.01, 0, WeatherIcon::MoonNew, WeatherIcon::MoonAltNew),
        (
            0.02,
            1,
            WeatherIcon::MoonWaxingCrescent1,
            WeatherIcon::MoonAltWaxingCrescent1,
        ),
        (
            0.03,
            1,
            WeatherIcon::MoonWaxingCrescent1,
            WeatherIcon::MoonAltWaxingCrescent1,
        ),
        (
            0.04,
            1,
            WeatherIcon::MoonWaxingCrescent1,
            WeatherIcon::MoonAltWaxingCrescent1,
        ),
        (
            0.05,
            1,
            WeatherIcon::MoonWaxingCrescent1,
            WeatherIcon::MoonAltWaxingCrescent1,
        ),
        (
            0.06,
            2,
            WeatherIcon::MoonWaxingCrescent2,
            WeatherIcon::MoonAltWaxingCrescent2,
        ),
        (
            0.07,
            2,
            WeatherIcon::MoonWaxingCrescent2,
            WeatherIcon::MoonAltWaxingCrescent2,
        ),
        (
            0.08,
            2,
            WeatherIcon::MoonWaxingCrescent2,
            WeatherIcon::MoonAltWaxingCrescent2,
        ),
        (
            0.09,
            3,
            WeatherIcon::MoonWaxingCrescent3,
            WeatherIcon::MoonAltWaxingCrescent3,
        ),
        (
            0.10,
            3,
            WeatherIcon::MoonWaxingCrescent3,
            WeatherIcon::MoonAltWaxingCrescent3,
        ),
        (
            0.11,
            3,
            WeatherIcon::MoonWaxingCrescent3,
            WeatherIcon::MoonAltWaxingCrescent3,
        ),
        (
            0.12,
            3,
            WeatherIcon::MoonWaxingCrescent3,
            WeatherIcon::MoonAltWaxingCrescent3,
        ),
        (
            0.13,
            4,
            WeatherIcon::MoonWaxingCrescent4,
            WeatherIcon::MoonAltWaxingCrescent4,
        ),
        (
            0.14,
            4,
            WeatherIcon::MoonWaxingCrescent4,
            WeatherIcon::MoonAltWaxingCrescent4,
        ),
        (
            0.15,
            4,
            WeatherIcon::MoonWaxingCrescent4,
            WeatherIcon::MoonAltWaxingCrescent4,
        ),
        (
            0.16,
            4,
            WeatherIcon::MoonWaxingCrescent4,
            WeatherIcon::MoonAltWaxingCrescent4,
        ),
        (
            0.17,
            5,
            WeatherIcon::MoonWaxingCrescent5,
            WeatherIcon::MoonAltWaxingCrescent5,
        ),
        (
            0.18,
            5,
            WeatherIcon::MoonWaxingCrescent5,
            WeatherIcon::MoonAltWaxingCrescent5,
        ),
        (
            0.19,
            5,
            WeatherIcon::MoonWaxingCrescent5,
            WeatherIcon::MoonAltWaxingCrescent5,
        ),
        (
            0.20,
            6,
            WeatherIcon::MoonWaxingCrescent6,
            WeatherIcon::MoonAltWaxingCrescent6,
        ),
        (
            0.21,
            6,
            WeatherIcon::MoonWaxingCrescent6,
            WeatherIcon::MoonAltWaxingCrescent6,
        ),
        (
            0.22,
            6,
            WeatherIcon::MoonWaxingCrescent6,
            WeatherIcon::MoonAltWaxingCrescent6,
        ),
        (
            0.23,
            6,
            WeatherIcon::MoonWaxingCrescent6,
            WeatherIcon::MoonAltWaxingCrescent6,
        ),
        (
            0.24,
            7,
            WeatherIcon::MoonFirstQuarter,
            WeatherIcon::MoonAltFirstQuarter,
        ),
        (
            0.25,
            7,
            WeatherIcon::MoonFirstQuarter,
            WeatherIcon::MoonAltFirstQuarter,
        ),
        (
            0.26,
            7,
            WeatherIcon::MoonFirstQuarter,
            WeatherIcon::MoonAltFirstQuarter,
        ),
        (
            0.27,
            8,
            WeatherIcon::MoonWaxingGibbous1,
            WeatherIcon::MoonAltWaxingGibbous1,
        ),
        (
            0.28,
            8,
            WeatherIcon::MoonWaxingGibbous1,
            WeatherIcon::MoonAltWaxingGibbous1,
        ),
        (
            0.29,
            8,
            WeatherIcon::MoonWaxingGibbous1,
            WeatherIcon::MoonAltWaxingGibbous1,
        ),
        (
            0.30,
            8,
            WeatherIcon::MoonWaxingGibbous1,
            WeatherIcon::MoonAltWaxingGibbous1,
        ),
        (
            0.31,
            9,
            WeatherIcon::MoonWaxingGibbous2,
            WeatherIcon::MoonAltWaxingGibbous2,
        ),
        (
            0.32,
            9,
            WeatherIcon::MoonWaxingGibbous2,
            WeatherIcon::MoonAltWaxingGibbous2,
        ),
        (
            0.33,
            9,
            WeatherIcon::MoonWaxingGibbous2,
            WeatherIcon::MoonAltWaxingGibbous2,
        ),
        (
            0.34,
            10,
            WeatherIcon::MoonWaxingGibbous3,
            WeatherIcon::MoonAltWaxingGibbous3,
        ),
        (
            0.35,
            10,
            WeatherIcon::MoonWaxingGibbous3,
            WeatherIcon::MoonAltWaxingGibbous3,
        ),
        (
            0.36,
            10,
            WeatherIcon::MoonWaxingGibbous3,
            WeatherIcon::MoonAltWaxingGibbous3,
        ),
        (
            0.37,
            10,
            WeatherIcon::MoonWaxingGibbous3,
            WeatherIcon::MoonAltWaxingGibbous3,
        ),
        (
            0.38,
            11,
            WeatherIcon::MoonWaxingGibbous4,
            WeatherIcon::MoonAltWaxingGibbous4,
        ),
        (
            0.39,
            11,
            WeatherIcon::MoonWaxingGibbous4,
            WeatherIcon::MoonAltWaxingGibbous4,
        ),
        (
            0.40,
            11,
            WeatherIcon::MoonWaxingGibbous4,
            WeatherIcon::MoonAltWaxingGibbous4,
        ),
        (
            0.41,
            11,
            WeatherIcon::MoonWaxingGibbous4,
            WeatherIcon::MoonAltWaxingGibbous4,
        ),
        (
            0.42,
            12,
            WeatherIcon::MoonWaxingGibbous5,
            WeatherIcon::MoonAltWaxingGibbous5,
        ),
        (
            0.43,
            12,
            WeatherIcon::MoonWaxingGibbous5,
            WeatherIcon::MoonAltWaxingGibbous5,
        ),
        (
            0.44,
            12,
            WeatherIcon::MoonWaxingGibbous5,
            WeatherIcon::MoonAltWaxingGibbous5,
        ),
        (
            0.45,
            13,
            WeatherIcon::MoonWaxingGibbous6,
            WeatherIcon::MoonAltWaxingGibbous6,
        ),
        (
            0.46,
            13,
            WeatherIcon::MoonWaxingGibbous6,
            WeatherIcon::MoonAltWaxingGibbous6,
        ),
        (
            0.47,
            13,
            WeatherIcon::MoonWaxingGibbous6,
            WeatherIcon::MoonAltWaxingGibbous6,
        ),
        (
            0.48,
            13,
            WeatherIcon::MoonWaxingGibbous6,
            WeatherIcon::MoonAltWaxingGibbous6,
        ),
        (0.49, 14, WeatherIcon::MoonFull, WeatherIcon::MoonAltFull),
        (0.50, 14, WeatherIcon::MoonFull, WeatherIcon::MoonAltFull),
        (0.51, 14, WeatherIcon::MoonFull, WeatherIcon::MoonAltFull),
        (
            0.52,
            15,
            WeatherIcon::MoonWaningGibbous1,
            WeatherIcon::MoonAltWaningGibbous1,
        ),
        (
            0.53,
            15,
            WeatherIcon::MoonWaningGibbous1,
            WeatherIcon::MoonAltWaningGibbous1,
        ),
        (
            0.54,
            15,
            WeatherIcon::MoonWaningGibbous1,
            WeatherIcon::MoonAltWaningGibbous1,
        ),
        (
            0.55,
            15,
            WeatherIcon::MoonWaningGibbous1,
            WeatherIcon::MoonAltWaningGibbous1,
        ),
        (
            0.56,
            16,
            WeatherIcon::MoonWaningGibbous2,
            WeatherIcon::MoonAltWaningGibbous2,
        ),
        (
            0.57,
            16,
            WeatherIcon::MoonWaningGibbous2,
            WeatherIcon::MoonAltWaningGibbous2,
        ),
        (
            0.58,
            16,
            WeatherIcon::MoonWaningGibbous2,
            WeatherIcon::MoonAltWaningGibbous2,
        ),
        (
            0.59,
            17,
            WeatherIcon::MoonWaningGibbous3,
            WeatherIcon::MoonAltWaningGibbous3,
        ),
        (
            0.60,
            17,
            WeatherIcon::MoonWaningGibbous3,
            WeatherIcon::MoonAltWaningGibbous3,
        ),
        (
            0.61,
            17,
            WeatherIcon::MoonWaningGibbous3,
            WeatherIcon::MoonAltWaningGibbous3,
        ),
        (
            0.62,
            17,
            WeatherIcon::MoonWaningGibbous3,
            WeatherIcon::MoonAltWaningGibbous3,
        ),
        (
            0.63,
            18,
            WeatherIcon::MoonWaningGibbous4,
            WeatherIcon::MoonAltWaningGibbous4,
        ),
        (
            0.64,
            18,
            WeatherIcon::MoonWaningGibbous4,
            WeatherIcon::MoonAltWaningGibbous4,
        ),
        (
            0.65,
            18,
            WeatherIcon::MoonWaningGibbous4,
            WeatherIcon::MoonAltWaningGibbous4,
        ),
        (
            0.66,
            18,
            WeatherIcon::MoonWaningGibbous4,
            WeatherIcon::MoonAltWaningGibbous4,
        ),
        (
            0.67,
            19,
            WeatherIcon::MoonWaningGibbous5,
            WeatherIcon::MoonAltWaningGibbous5,
        ),
        (
            0.68,
            19,
            WeatherIcon::MoonWaningGibbous5,
            WeatherIcon::MoonAltWaningGibbous5,
        ),
        (
            0.69,
            19,
            WeatherIcon::MoonWaningGibbous5,
            WeatherIcon::MoonAltWaningGibbous5,
        ),
        (
            0.70,
            20,
            WeatherIcon::MoonWaningGibbous6,
            WeatherIcon::MoonAltWaningGibbous6,
        ),
        (
            0.71,
            20,
            WeatherIcon::MoonWaningGibbous6,
            WeatherIcon::MoonAltWaningGibbous6,
        ),
        (
            0.72,
            20,
            WeatherIcon::MoonWaningGibbous6,
            WeatherIcon::MoonAltWaningGibbous6,
        ),
        (
            0.73,
            20,
            WeatherIcon::MoonWaningGibbous6,
            WeatherIcon::MoonAltWaningGibbous6,
        ),
        (
            0.74,
            21,
            WeatherIcon::MoonThirdQuarter,
            WeatherIcon::MoonAltThirdQuarter,
        ),
        (
            0.75,
            21,
            WeatherIcon::MoonThirdQuarter,
            WeatherIcon::MoonAltThirdQuarter,
        ),
        (
            0.76,
            21,
            WeatherIcon::MoonThirdQuarter,
            WeatherIcon::MoonAltThirdQuarter,
        ),
        (
            0.77,
            22,
            WeatherIcon::MoonWaningCrescent1,
            WeatherIcon::MoonAltWaningCrescent1,
        ),
        (
            0.78,
            22,
            WeatherIcon::MoonWaningCrescent1,
            WeatherIcon::MoonAltWaningCrescent1,
        ),
        (
            0.79,
            22,
            WeatherIcon::MoonWaningCrescent1,
            WeatherIcon::MoonAltWaningCrescent1,
        ),
        (
            0.80,
            22,
            WeatherIcon::MoonWaningCrescent1,
            WeatherIcon::MoonAltWaningCrescent1,
        ),
        (
            0.81,
            23,
            WeatherIcon::MoonWaningCrescent2,
            WeatherIcon::MoonAltWaningCrescent2,
        ),
        (
            0.82,
            23,
            WeatherIcon::MoonWaningCrescent2,
            WeatherIcon::MoonAltWaningCrescent2,
        ),
        (
            0.83,
            23,
            WeatherIcon::MoonWaningCrescent2,
            WeatherIcon::MoonAltWaningCrescent2,
        ),
        (
            0.84,
            24,
            WeatherIcon::MoonWaningCrescent3,
            WeatherIcon::MoonAltWaningCrescent3,
        ),
        (
            0.85,
            24,
            WeatherIcon::MoonWaningCrescent3,
            WeatherIcon::MoonAltWaningCrescent3,
        ),
        (
            0.86,
            24,
            WeatherIcon::MoonWaningCrescent3,
            WeatherIcon::MoonAltWaningCrescent3,
        ),
        (
            0.87,
            24,
            WeatherIcon::MoonWaningCrescent3,
            WeatherIcon::MoonAltWaningCrescent3,
        ),
        (
            0.88,
            25,
            WeatherIcon::MoonWaningCrescent4,
            WeatherIcon::MoonAltWaningCrescent4,
        ),
        (
            0.89,
            25,
            WeatherIcon::MoonWaningCrescent4,
            WeatherIcon::MoonAltWaningCrescent4,
        ),
        (
            0.90,
            25,
            WeatherIcon::MoonWaningCrescent4,
            WeatherIcon::MoonAltWaningCrescent4,
        ),
        (
            0.91,
            25,
            WeatherIcon::MoonWaningCrescent4,
            WeatherIcon::MoonAltWaningCrescent4,
        ),
        (
            0.92,
            26,
            WeatherIcon::MoonWaningCrescent5,
            WeatherIcon::MoonAltWaningCrescent5,
        ),
        (
            0.93,
            26,
            WeatherIcon::MoonWaningCrescent5,
            WeatherIcon::MoonAltWaningCrescent5,
        ),
        (
            0.94,
            26,
            WeatherIcon::MoonWaningCrescent5,
            WeatherIcon::MoonAltWaningCrescent5,
        ),
        (
            0.95,
            27,
            WeatherIcon::MoonWaningCrescent6,
            WeatherIcon::MoonAltWaningCrescent6,
        ),
        (
            0.96,
            27,
            WeatherIcon::MoonWaningCrescent6,
            WeatherIcon::MoonAltWaningCrescent6,
        ),
        (
            0.97,
            27,
            WeatherIcon::MoonWaningCrescent6,
            WeatherIcon::MoonAltWaningCrescent6,
        ),
        (
            0.98,
            27,
            WeatherIcon::MoonWaningCrescent6,
            WeatherIcon::MoonAltWaningCrescent6,
        ),
        (0.99, 0, WeatherIcon::MoonNew, WeatherIcon::MoonAltNew),
        (1.00, 0, WeatherIcon::MoonNew, WeatherIcon::MoonAltNew),
    ];

    #[test]
    fn test_moon_phase() {
        for i in TEST_MOON_PHASES.iter() {
            let expected_primary = i.2;
            let expected_alt = i.3;

            let actual_primary = Moon::new(i.0, Style::Primary).unwrap();
            let actual_alt = Moon::new(i.0, Style::Alt).unwrap();

            assert_eq!(
                expected_primary as u32,
                actual_primary.icon as u32,
                "Expected primary: {:x} {:?}, actual: {} {:?}",
                expected_primary as u32,
                expected_primary,
                actual_primary.icon.escape_unicode(),
                actual_primary
            );
            assert_eq!(
                expected_alt as u32,
                actual_alt.icon as u32,
                "Expected alt: {:x} {:?}, actual: {} {:?}",
                expected_alt as u32,
                expected_alt,
                actual_alt.icon.escape_unicode(),
                actual_alt
            );
        }
    }

    #[test]
    #[should_panic]
    fn lunar_number_less_than_0_primary() {
        Moon::new(-1f64, Style::Primary).unwrap();
    }

    #[test]
    #[should_panic]
    fn lunar_number_less_than_0_alt() {
        Moon::new(-1f64, Style::Alt).unwrap();
    }

    #[test]
    #[should_panic]
    fn lunar_number_greater_than_1_primary() {
        Moon::new(2f64, Style::Primary).unwrap();
    }

    #[test]
    #[should_panic]
    fn lunar_number_greater_than_1_alt() {
        Moon::new(2f64, Style::Alt).unwrap();
    }
}
