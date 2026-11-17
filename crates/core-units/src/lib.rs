//! Unit types and conversion interfaces; domain conversions are added with sourced algorithms.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TemperatureUnit {
    Celsius,
    Kelvin,
    Fahrenheit,
}
