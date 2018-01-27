extern crate weather_icons;

use weather_icons::{Moon, Style};

#[test]
fn check_phase_icons() {
    let mut moon = Moon::new();

    for (p, m) in (0..29).zip(0xf095..0xf0b2) {
        assert_eq!(
            format!("{}", moon.phase(f64::from(p) / 28f64).unwrap().build()),
            ::std::char::from_u32(m).unwrap().to_string()
        );
    }
}

#[test]
fn check_phase_icons_alt() {
    let mut moon = Moon::new().style(Style::Alt).to_owned();

    for (p, m) in (0..29).zip(0xf0eb..0xf108) {
        eprintln!("{}, {:4x}", p, m);
        assert_eq!(
            format!("{}", moon.phase(f64::from(p) / 28f64).unwrap().build()),
            ::std::char::from_u32(m).unwrap().to_string()
        );
    }
}

#[test]
#[should_panic]
fn lunar_number_less_than_0() {
    Moon::new().style(Style::Primary).phase(-1f64).unwrap();
}

#[test]
#[should_panic]
fn lunar_number_greater_than_1() {
    Moon::new().style(Style::Primary).phase(2f64).unwrap();
}
