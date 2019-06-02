//! # [Heron's method](https://en.wikipedia.org/wiki/Methods_of_computing_square_roots#Babylonian_method) implmentation
//!
//! Heron's method is a way of approximating the square root of a number.
//! ## Example
//!
//! ```
//! use heron::heron;
//! let square = 25.0;
//! let precision = 0.00001;
//!
//! assert_eq!(heron(square, precision), 5.0);
//! ```
//!

/// Approximates the square root of the given number until it reaches the given precision.
pub fn heron(square: f32, precision: f32) -> f32 {
    let mut current = (square - 1.0) / 2.0;
    while (current * current - square).abs() > precision {
        current = (current + square / current) / 2.0;
    }

    current
}

#[cfg(test)]
mod test {
    use super::heron;

    #[test]
    fn four() {
        assert_eq!(heron(4.0, 0.000001), 2.0)
    }

    #[test]
    fn twenty_four() {
        assert_eq!(heron(25.0, 0.000001), 5.0)
    }
}
