//! Floating-point comparison policy primitives.

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Tolerance {
    pub absolute: f64,
    pub relative: f64,
}
