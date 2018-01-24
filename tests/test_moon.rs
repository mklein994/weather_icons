extern crate weather_icons;

use weather_icons::{Moon, Theme};

#[test]
fn check_phase_icons() {
    use Theme::*;
    assert_eq!(Moon::phase(Primary, 0f64 / 28f64), '\u{f095}'); // new moon
    assert_eq!(Moon::phase(Primary, 1f64 / 28f64), '\u{f096}');
    assert_eq!(Moon::phase(Primary, 2f64 / 28f64), '\u{f097}');
    assert_eq!(Moon::phase(Primary, 3f64 / 28f64), '\u{f098}');
    assert_eq!(Moon::phase(Primary, 4f64 / 28f64), '\u{f099}');
    assert_eq!(Moon::phase(Primary, 5f64 / 28f64), '\u{f09a}');
    assert_eq!(Moon::phase(Primary, 6f64 / 28f64), '\u{f09b}');
    assert_eq!(Moon::phase(Primary, 7f64 / 28f64), '\u{f09c}'); // first quarter moon
    assert_eq!(Moon::phase(Primary, 8f64 / 28f64), '\u{f09d}');
    assert_eq!(Moon::phase(Primary, 9f64 / 28f64), '\u{f09e}');
    assert_eq!(Moon::phase(Primary, 10f64 / 28f64), '\u{f09f}');
    assert_eq!(Moon::phase(Primary, 11f64 / 28f64), '\u{f0a0}');
    assert_eq!(Moon::phase(Primary, 12f64 / 28f64), '\u{f0a1}');
    assert_eq!(Moon::phase(Primary, 13f64 / 28f64), '\u{f0a2}');
    assert_eq!(Moon::phase(Primary, 14f64 / 28f64), '\u{f0a3}'); // full moon
    assert_eq!(Moon::phase(Primary, 15f64 / 28f64), '\u{f0a4}');
    assert_eq!(Moon::phase(Primary, 16f64 / 28f64), '\u{f0a5}');
    assert_eq!(Moon::phase(Primary, 17f64 / 28f64), '\u{f0a6}');
    assert_eq!(Moon::phase(Primary, 18f64 / 28f64), '\u{f0a7}');
    assert_eq!(Moon::phase(Primary, 19f64 / 28f64), '\u{f0a8}');
    assert_eq!(Moon::phase(Primary, 20f64 / 28f64), '\u{f0a9}');
    assert_eq!(Moon::phase(Primary, 21f64 / 28f64), '\u{f0aa}'); // last quarter moon
    assert_eq!(Moon::phase(Primary, 22f64 / 28f64), '\u{f0ab}');
    assert_eq!(Moon::phase(Primary, 23f64 / 28f64), '\u{f0ac}');
    assert_eq!(Moon::phase(Primary, 24f64 / 28f64), '\u{f0ad}');
    assert_eq!(Moon::phase(Primary, 25f64 / 28f64), '\u{f0ae}');
    assert_eq!(Moon::phase(Primary, 26f64 / 28f64), '\u{f0af}');
    assert_eq!(Moon::phase(Primary, 27f64 / 28f64), '\u{f0b0}');
    assert_eq!(Moon::phase(Primary, 28f64 / 28f64), '\u{f095}'); // Back to new moon
}

#[test]
#[should_panic]
fn lunar_number_less_than_0() {
    Moon::phase(Theme::Primary, -1f64);
}

#[test]
#[should_panic]
fn lunar_number_greater_than_1() {
    Moon::phase(Theme::Primary, 2f64);
}
