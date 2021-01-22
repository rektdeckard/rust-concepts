use std::vec::Vec;

const CELCIUS_KELVIN_OFFSET: f64 = 273.15;
const CELCIUS_FAHRENHEIT_RATIO: f64 = 9.0 / 5.0;
const CELCIUS_FAHRENHEIT_OFFSET: f64 = 32.0;

pub struct Temperature {
    celcius: f64,
}

impl Temperature {
    pub fn new(celcius: f64) -> Temperature {
        Temperature { celcius }
    }

    pub fn in_fahrenheit(&self) -> f64 {
        (self.celcius * CELCIUS_FAHRENHEIT_RATIO) + CELCIUS_FAHRENHEIT_OFFSET
    }

    pub fn in_celcius(&self) -> f64 {
        self.celcius
    }

    pub fn in_kelvin(&self) -> f64 {
        self.celcius + CELCIUS_KELVIN_OFFSET
    }
}

impl Default for Temperature {
    fn default() -> Self {
        Temperature { celcius: 0.0 }
    }
}

impl From<&str> for Temperature {
    fn from(s: &str) -> Self {
        let parts = s.splitn(2, '/').collect::<Vec<&str>>();
        match &parts[..] {
            [temp, scale] if !temp.is_empty() => temp
                .parse::<f64>()
                .map(|temp| Temperature {
                    celcius: if *scale == "f" || *scale == "F" {
                        (temp - CELCIUS_FAHRENHEIT_OFFSET) * (1.0 / CELCIUS_FAHRENHEIT_RATIO)
                    } else if *scale == "k" || *scale == "K" {
                        temp - CELCIUS_KELVIN_OFFSET
                    } else {
                        temp
                    },
                })
                .unwrap_or_default(),
            _ => Temperature::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn temp_from_constructor() {
        let ten_c = Temperature::new(10.0);
        assert_eq!(10.0, ten_c.celcius);
    }

    #[test]
    fn temp_from_string() {
        let fifty_c = Temperature::from("50/C");
        assert_eq!(50.0, fifty_c.celcius);
    }

    #[test]
    fn temp_from_fahreneheit_string() {
        let freezing = Temperature::from("32/f");
        assert_eq!(0.0, freezing.celcius);
    }

    #[test]
    fn default_temp_from_bad_string() {
        let default_on_bad = Temperature::from("F90");
        assert_eq!(0.0, default_on_bad.celcius);
    }

    #[test]
    fn in_fahrenheit() {
        let temp = Temperature::new(35.0);
        assert_eq!(95.0, temp.in_fahrenheit());
    }

    #[test]
    fn in_kelvin() {
        let temp = Temperature::new(50.0);
        assert_eq!(323.15, temp.in_kelvin())
    }

    #[test]
    fn in_celcius() {
        let temp = Temperature::new(42.0);
        assert_eq!(42.0, temp.in_celcius());
    }
}
