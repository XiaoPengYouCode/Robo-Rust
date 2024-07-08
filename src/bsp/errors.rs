use defmt::Format;

pub enum PeripheralsError {
    PwmInitError,
    LedInitError,
}

impl Format for PeripheralsError {
    fn format(&self, f: defmt::Formatter) {
        match self {
            PeripheralsError::PwmInitError => defmt::write!(f, "PwmInitError"),
            PeripheralsError::LedInitError => defmt::write!(f, "LedInitError"),
        }
    }
}