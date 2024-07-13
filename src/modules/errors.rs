#![allow(dead_code)]

use defmt::Format;

pub enum PeripheralsError {
    PwmInitError,
    LedInitError,
    ImuInitError,
}

impl Format for PeripheralsError {
    fn format(&self, f: defmt::Formatter) {
        match self {
            PeripheralsError::PwmInitError => defmt::write!(f, "PwmInitError"),
            PeripheralsError::LedInitError => defmt::write!(f, "LedInitError"),
            PeripheralsError::ImuInitError => defmt::write!(f, "ImuInitError"),
        }
    }
}