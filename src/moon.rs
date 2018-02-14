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
