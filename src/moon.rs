use super::Icon;
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
    Alt = Icon::MoonAltNew as isize, // 0xf0eb,
    /// '\u{f095}', i.e. wi-moon-new
    ///
    /// A full moon is filled with pixels.
    Primary = Icon::MoonNew as isize, // 0xf095,
}

impl Default for Style {
    fn default() -> Self {
        Style::Primary
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Moon(u32);

impl fmt::Display for Moon {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            ::std::char::from_u32(self.0).expect("Failed to convert moon phase into char")
        )
    }
}

impl Moon {
    pub fn new() -> MoonBuilder {
        Default::default()
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct MoonBuilder {
    phase: u32,
    style: Style,
}

impl MoonBuilder {
    pub fn phase(&mut self, phase: f64) -> Result<&mut Self, OutOfBounds> {
        if phase > 1. || phase < 0. {
            return Err(OutOfBounds);
        }
        self.phase = (phase * MOON_PHASES).round() as u32;
        Ok(self)
    }

    pub fn style(&mut self, style: Style) -> &mut Self {
        self.style = style;
        self
    }

    pub fn build(self) -> Moon {
        Moon(self.style as u32 + self.phase)
    }
}

#[derive(Debug)]
pub struct OutOfBounds;

impl fmt::Display for OutOfBounds {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Out of bounds")
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

    const TEST_MOON_PHASES: [(f64, i32, Icon, Icon); 101] = [
        (0.00, 0, Icon::MoonNew, Icon::MoonAltNew),
        (0.01, 0, Icon::MoonNew, Icon::MoonAltNew),
        (
            0.02,
            1,
            Icon::MoonWaxingCrescent1,
            Icon::MoonAltWaxingCrescent1,
        ),
        (
            0.03,
            1,
            Icon::MoonWaxingCrescent1,
            Icon::MoonAltWaxingCrescent1,
        ),
        (
            0.04,
            1,
            Icon::MoonWaxingCrescent1,
            Icon::MoonAltWaxingCrescent1,
        ),
        (
            0.05,
            1,
            Icon::MoonWaxingCrescent1,
            Icon::MoonAltWaxingCrescent1,
        ),
        (
            0.06,
            2,
            Icon::MoonWaxingCrescent2,
            Icon::MoonAltWaxingCrescent2,
        ),
        (
            0.07,
            2,
            Icon::MoonWaxingCrescent2,
            Icon::MoonAltWaxingCrescent2,
        ),
        (
            0.08,
            2,
            Icon::MoonWaxingCrescent2,
            Icon::MoonAltWaxingCrescent2,
        ),
        (
            0.09,
            3,
            Icon::MoonWaxingCrescent3,
            Icon::MoonAltWaxingCrescent3,
        ),
        (
            0.10,
            3,
            Icon::MoonWaxingCrescent3,
            Icon::MoonAltWaxingCrescent3,
        ),
        (
            0.11,
            3,
            Icon::MoonWaxingCrescent3,
            Icon::MoonAltWaxingCrescent3,
        ),
        (
            0.12,
            3,
            Icon::MoonWaxingCrescent3,
            Icon::MoonAltWaxingCrescent3,
        ),
        (
            0.13,
            4,
            Icon::MoonWaxingCrescent4,
            Icon::MoonAltWaxingCrescent4,
        ),
        (
            0.14,
            4,
            Icon::MoonWaxingCrescent4,
            Icon::MoonAltWaxingCrescent4,
        ),
        (
            0.15,
            4,
            Icon::MoonWaxingCrescent4,
            Icon::MoonAltWaxingCrescent4,
        ),
        (
            0.16,
            4,
            Icon::MoonWaxingCrescent4,
            Icon::MoonAltWaxingCrescent4,
        ),
        (
            0.17,
            5,
            Icon::MoonWaxingCrescent5,
            Icon::MoonAltWaxingCrescent5,
        ),
        (
            0.18,
            5,
            Icon::MoonWaxingCrescent5,
            Icon::MoonAltWaxingCrescent5,
        ),
        (
            0.19,
            5,
            Icon::MoonWaxingCrescent5,
            Icon::MoonAltWaxingCrescent5,
        ),
        (
            0.20,
            6,
            Icon::MoonWaxingCrescent6,
            Icon::MoonAltWaxingCrescent6,
        ),
        (
            0.21,
            6,
            Icon::MoonWaxingCrescent6,
            Icon::MoonAltWaxingCrescent6,
        ),
        (
            0.22,
            6,
            Icon::MoonWaxingCrescent6,
            Icon::MoonAltWaxingCrescent6,
        ),
        (
            0.23,
            6,
            Icon::MoonWaxingCrescent6,
            Icon::MoonAltWaxingCrescent6,
        ),
        (0.24, 7, Icon::MoonFirstQuarter, Icon::MoonAltFirstQuarter),
        (0.25, 7, Icon::MoonFirstQuarter, Icon::MoonAltFirstQuarter),
        (0.26, 7, Icon::MoonFirstQuarter, Icon::MoonAltFirstQuarter),
        (
            0.27,
            8,
            Icon::MoonWaxingGibbous1,
            Icon::MoonAltWaxingGibbous1,
        ),
        (
            0.28,
            8,
            Icon::MoonWaxingGibbous1,
            Icon::MoonAltWaxingGibbous1,
        ),
        (
            0.29,
            8,
            Icon::MoonWaxingGibbous1,
            Icon::MoonAltWaxingGibbous1,
        ),
        (
            0.30,
            8,
            Icon::MoonWaxingGibbous1,
            Icon::MoonAltWaxingGibbous1,
        ),
        (
            0.31,
            9,
            Icon::MoonWaxingGibbous2,
            Icon::MoonAltWaxingGibbous2,
        ),
        (
            0.32,
            9,
            Icon::MoonWaxingGibbous2,
            Icon::MoonAltWaxingGibbous2,
        ),
        (
            0.33,
            9,
            Icon::MoonWaxingGibbous2,
            Icon::MoonAltWaxingGibbous2,
        ),
        (
            0.34,
            10,
            Icon::MoonWaxingGibbous3,
            Icon::MoonAltWaxingGibbous3,
        ),
        (
            0.35,
            10,
            Icon::MoonWaxingGibbous3,
            Icon::MoonAltWaxingGibbous3,
        ),
        (
            0.36,
            10,
            Icon::MoonWaxingGibbous3,
            Icon::MoonAltWaxingGibbous3,
        ),
        (
            0.37,
            10,
            Icon::MoonWaxingGibbous3,
            Icon::MoonAltWaxingGibbous3,
        ),
        (
            0.38,
            11,
            Icon::MoonWaxingGibbous4,
            Icon::MoonAltWaxingGibbous4,
        ),
        (
            0.39,
            11,
            Icon::MoonWaxingGibbous4,
            Icon::MoonAltWaxingGibbous4,
        ),
        (
            0.40,
            11,
            Icon::MoonWaxingGibbous4,
            Icon::MoonAltWaxingGibbous4,
        ),
        (
            0.41,
            11,
            Icon::MoonWaxingGibbous4,
            Icon::MoonAltWaxingGibbous4,
        ),
        (
            0.42,
            12,
            Icon::MoonWaxingGibbous5,
            Icon::MoonAltWaxingGibbous5,
        ),
        (
            0.43,
            12,
            Icon::MoonWaxingGibbous5,
            Icon::MoonAltWaxingGibbous5,
        ),
        (
            0.44,
            12,
            Icon::MoonWaxingGibbous5,
            Icon::MoonAltWaxingGibbous5,
        ),
        (
            0.45,
            13,
            Icon::MoonWaxingGibbous6,
            Icon::MoonAltWaxingGibbous6,
        ),
        (
            0.46,
            13,
            Icon::MoonWaxingGibbous6,
            Icon::MoonAltWaxingGibbous6,
        ),
        (
            0.47,
            13,
            Icon::MoonWaxingGibbous6,
            Icon::MoonAltWaxingGibbous6,
        ),
        (
            0.48,
            13,
            Icon::MoonWaxingGibbous6,
            Icon::MoonAltWaxingGibbous6,
        ),
        (0.49, 14, Icon::MoonFull, Icon::MoonAltFull),
        (0.50, 14, Icon::MoonFull, Icon::MoonAltFull),
        (0.51, 14, Icon::MoonFull, Icon::MoonAltFull),
        (
            0.52,
            15,
            Icon::MoonWaningGibbous1,
            Icon::MoonAltWaningGibbous1,
        ),
        (
            0.53,
            15,
            Icon::MoonWaningGibbous1,
            Icon::MoonAltWaningGibbous1,
        ),
        (
            0.54,
            15,
            Icon::MoonWaningGibbous1,
            Icon::MoonAltWaningGibbous1,
        ),
        (
            0.55,
            15,
            Icon::MoonWaningGibbous1,
            Icon::MoonAltWaningGibbous1,
        ),
        (
            0.56,
            16,
            Icon::MoonWaningGibbous2,
            Icon::MoonAltWaningGibbous2,
        ),
        (
            0.57,
            16,
            Icon::MoonWaningGibbous2,
            Icon::MoonAltWaningGibbous2,
        ),
        (
            0.58,
            16,
            Icon::MoonWaningGibbous2,
            Icon::MoonAltWaningGibbous2,
        ),
        (
            0.59,
            17,
            Icon::MoonWaningGibbous3,
            Icon::MoonAltWaningGibbous3,
        ),
        (
            0.60,
            17,
            Icon::MoonWaningGibbous3,
            Icon::MoonAltWaningGibbous3,
        ),
        (
            0.61,
            17,
            Icon::MoonWaningGibbous3,
            Icon::MoonAltWaningGibbous3,
        ),
        (
            0.62,
            17,
            Icon::MoonWaningGibbous3,
            Icon::MoonAltWaningGibbous3,
        ),
        (
            0.63,
            18,
            Icon::MoonWaningGibbous4,
            Icon::MoonAltWaningGibbous4,
        ),
        (
            0.64,
            18,
            Icon::MoonWaningGibbous4,
            Icon::MoonAltWaningGibbous4,
        ),
        (
            0.65,
            18,
            Icon::MoonWaningGibbous4,
            Icon::MoonAltWaningGibbous4,
        ),
        (
            0.66,
            18,
            Icon::MoonWaningGibbous4,
            Icon::MoonAltWaningGibbous4,
        ),
        (
            0.67,
            19,
            Icon::MoonWaningGibbous5,
            Icon::MoonAltWaningGibbous5,
        ),
        (
            0.68,
            19,
            Icon::MoonWaningGibbous5,
            Icon::MoonAltWaningGibbous5,
        ),
        (
            0.69,
            19,
            Icon::MoonWaningGibbous5,
            Icon::MoonAltWaningGibbous5,
        ),
        (
            0.70,
            20,
            Icon::MoonWaningGibbous6,
            Icon::MoonAltWaningGibbous6,
        ),
        (
            0.71,
            20,
            Icon::MoonWaningGibbous6,
            Icon::MoonAltWaningGibbous6,
        ),
        (
            0.72,
            20,
            Icon::MoonWaningGibbous6,
            Icon::MoonAltWaningGibbous6,
        ),
        (
            0.73,
            20,
            Icon::MoonWaningGibbous6,
            Icon::MoonAltWaningGibbous6,
        ),
        (0.74, 21, Icon::MoonThirdQuarter, Icon::MoonAltThirdQuarter),
        (0.75, 21, Icon::MoonThirdQuarter, Icon::MoonAltThirdQuarter),
        (0.76, 21, Icon::MoonThirdQuarter, Icon::MoonAltThirdQuarter),
        (
            0.77,
            22,
            Icon::MoonWaningCrescent1,
            Icon::MoonAltWaningCrescent1,
        ),
        (
            0.78,
            22,
            Icon::MoonWaningCrescent1,
            Icon::MoonAltWaningCrescent1,
        ),
        (
            0.79,
            22,
            Icon::MoonWaningCrescent1,
            Icon::MoonAltWaningCrescent1,
        ),
        (
            0.80,
            22,
            Icon::MoonWaningCrescent1,
            Icon::MoonAltWaningCrescent1,
        ),
        (
            0.81,
            23,
            Icon::MoonWaningCrescent2,
            Icon::MoonAltWaningCrescent2,
        ),
        (
            0.82,
            23,
            Icon::MoonWaningCrescent2,
            Icon::MoonAltWaningCrescent2,
        ),
        (
            0.83,
            23,
            Icon::MoonWaningCrescent2,
            Icon::MoonAltWaningCrescent2,
        ),
        (
            0.84,
            24,
            Icon::MoonWaningCrescent3,
            Icon::MoonAltWaningCrescent3,
        ),
        (
            0.85,
            24,
            Icon::MoonWaningCrescent3,
            Icon::MoonAltWaningCrescent3,
        ),
        (
            0.86,
            24,
            Icon::MoonWaningCrescent3,
            Icon::MoonAltWaningCrescent3,
        ),
        (
            0.87,
            24,
            Icon::MoonWaningCrescent3,
            Icon::MoonAltWaningCrescent3,
        ),
        (
            0.88,
            25,
            Icon::MoonWaningCrescent4,
            Icon::MoonAltWaningCrescent4,
        ),
        (
            0.89,
            25,
            Icon::MoonWaningCrescent4,
            Icon::MoonAltWaningCrescent4,
        ),
        (
            0.90,
            25,
            Icon::MoonWaningCrescent4,
            Icon::MoonAltWaningCrescent4,
        ),
        (
            0.91,
            25,
            Icon::MoonWaningCrescent4,
            Icon::MoonAltWaningCrescent4,
        ),
        (
            0.92,
            26,
            Icon::MoonWaningCrescent5,
            Icon::MoonAltWaningCrescent5,
        ),
        (
            0.93,
            26,
            Icon::MoonWaningCrescent5,
            Icon::MoonAltWaningCrescent5,
        ),
        (
            0.94,
            26,
            Icon::MoonWaningCrescent5,
            Icon::MoonAltWaningCrescent5,
        ),
        (
            0.95,
            27,
            Icon::MoonWaningCrescent6,
            Icon::MoonAltWaningCrescent6,
        ),
        (
            0.96,
            27,
            Icon::MoonWaningCrescent6,
            Icon::MoonAltWaningCrescent6,
        ),
        (
            0.97,
            27,
            Icon::MoonWaningCrescent6,
            Icon::MoonAltWaningCrescent6,
        ),
        (
            0.98,
            27,
            Icon::MoonWaningCrescent6,
            Icon::MoonAltWaningCrescent6,
        ),
        (0.99, 0, Icon::MoonNew, Icon::MoonAltNew),
        (1.00, 0, Icon::MoonNew, Icon::MoonAltNew),
    ];

    #[test]
    fn test_moon_phase() {
        for i in TEST_MOON_PHASES.iter() {
            let expected_primary = i.2;
            let expected_alt = i.3;

            let actual = Moon::new().phase(i.0).unwrap().build();

            assert_eq!(
                expected_primary as u32, actual.0,
                "Expected primary: \\u{{{:x}}} {:?}, actual: \\u{{{:x}}} {:?}",
                expected_primary as u32, expected_primary, actual.0, actual
            );
            assert_eq!(
                expected_alt as u32, actual.0,
                "Expected alt: \\u{{{:x}}} {:?}, actual: \\u{{{:x}}} {:?}",
                expected_alt as u32, expected_alt, actual.0, actual
            );
        }
    }

    #[test]
    #[should_panic]
    fn lunar_number_less_than_0_primary() {
        Moon::new().style(Style::Primary).phase(-1f64).unwrap();
    }

    #[test]
    #[should_panic]
    fn lunar_number_less_than_0_alt() {
        Moon::new().style(Style::Alt).phase(-1f64).unwrap();
    }

    #[test]
    #[should_panic]
    fn lunar_number_greater_than_1_primary() {
        Moon::new().style(Style::Primary).phase(2f64).unwrap();
    }

    #[test]
    #[should_panic]
    fn lunar_number_greater_than_1_alt() {
        Moon::new().style(Style::Alt).phase(2f64).unwrap();
    }
}
