//! # Round numbers and durations to a given factor
//!
//! This provides an implementation of rounding for various values, including
//! the the native number types and [`core::time::Duration`] (also known as
//! `std::time::Duration`).
//!
//! The [`Roundable`] trait adds the following functions to roundable values:
//!
//!  * [`Roundable::try_round_to(factor, tie_strategy)`](Roundable::try_round_to())
//!    (returns `None` on overflow)
//!  * [`Roundable::round_to(factor, tie_strategy)`](Roundable::round_to())
//!    (panics on overflow)
//!
//! ### Example
//!
//! ```rust
//! use roundable::{Roundable, Tie};
//!
//! assert!(310 == 314.round_to(10, Tie::Up));
//! assert!(300.0 == 314.1.round_to(100.0, Tie::Up));
//!
//! // To avoid panicking on overflow:
//! assert!(Some(260) == 255.try_round_to(10, Tie::Up));
//! assert!(None == 255u8.try_round_to(10, Tie::Up));
//! ```
//!
//! ## Tie strategies
//!
//! “Ties” are numbers exactly halfway between two round numbers, e.g. 0.5 when
//! rounding to the nearest whole number. Traditionally, ties are resolved by
//! picking the higher number, but there are other strategies. `Roundable` supports
//! the following rules:
//!
//!   * [`Tie::Up`]: Round ties up (what most people consider correct).
//!   * [`Tie::Down`]: Round ties down.
//!   * [`Tie::TowardZero`]: Round ties toward zero.
//!   * [`Tie::AwayFromZero`]: Round ties away from zero.
//!   * [`Tie::TowardEven`]: Round ties toward the “even” number (see docs).
//!   * [`Tie::TowardOdd`]: Round ties toward the “odd” number (see docs).
//!
//! ## Rounding `Duration`
//!
//! See [the list of constants](#constants) for a list of time units that make
//! rounding [`Duration`](core::time::Duration) easier.
//!
//! ```rust
//! use roundable::{SECOND, MINUTE, Roundable, Tie};
//! use std::time::Duration;
//!
//! assert!(Duration::ZERO == Duration::from_millis(314).round_to(SECOND, Tie::Up));
//! assert!(MINUTE == Duration::from_millis(59_500).round_to(SECOND, Tie::Up));
//! ```
//!
//! ## `#![no_std]` by default
//!
//! You can use this crate with or without `std` and `alloc`. You do not need to
//! enable or disable features either way.
//!
//! ## Minimum supported Rust version
//!
//! Currently the minimum supported Rust version (MSRV) is **1.56.1**. Future
//! increases in the MSRV will require a major version bump.

// Lint configuration in Cargo.toml isn’t supported by cargo-geiger.
#![forbid(unsafe_code)]
#![no_std]

mod duration;
pub use duration::*;

/// How to handle a value that is exactly half, e.g. `5.round_to(10, ...)`.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Tie {
    /// Round half up (what most people consider correct).
    ///
    /// ```rust
    /// use roundable::{Roundable, Tie};
    ///
    /// assert!(10 == 5.round_to(10, Tie::Up));
    /// assert!(0 == (-5).round_to(10, Tie::Up));
    ///
    /// // Other values are unaffected:
    /// assert!(0 == 4.round_to(10, Tie::Up));
    /// assert!(10 == 6.round_to(10, Tie::Up));
    /// assert!(0 == (-4).round_to(10, Tie::Up));
    /// assert!(-10 == (-6).round_to(10, Tie::Up));
    /// ```
    Up,

    /// Round half down.
    ///
    /// ```rust
    /// use roundable::{Roundable, Tie};
    ///
    /// assert!(0 == 5.round_to(10, Tie::Down));
    /// assert!(-10 == (-5).round_to(10, Tie::Down));
    ///
    /// // Other values are unaffected:
    /// assert!(0 == 4.round_to(10, Tie::Down));
    /// assert!(10 == 6.round_to(10, Tie::Down));
    /// assert!(0 == (-4).round_to(10, Tie::Down));
    /// assert!(-10 == (-6).round_to(10, Tie::Down));
    /// ```
    Down,

    /// Round half toward zero.
    ///
    /// ```rust
    /// use roundable::{Roundable, Tie};
    ///
    /// assert!(0 == 5.round_to(10, Tie::TowardZero));
    /// assert!(0 == (-5).round_to(10, Tie::TowardZero));
    ///
    /// // Other values are unaffected:
    /// assert!(0 == 4.round_to(10, Tie::TowardZero));
    /// assert!(10 == 6.round_to(10, Tie::TowardZero));
    /// assert!(0 == (-4).round_to(10, Tie::TowardZero));
    /// assert!(-10 == (-6).round_to(10, Tie::TowardZero));
    /// ```
    TowardZero,

    /// Round half away from zero.
    ///
    /// ```rust
    /// use roundable::{Roundable, Tie};
    ///
    /// assert!(10 == 5.round_to(10, Tie::AwayFromZero));
    /// assert!(-10 == (-5).round_to(10, Tie::AwayFromZero));
    ///
    /// // Other values are unaffected:
    /// assert!(0 == 4.round_to(10, Tie::AwayFromZero));
    /// assert!(10 == 6.round_to(10, Tie::AwayFromZero));
    /// assert!(0 == (-4).round_to(10, Tie::AwayFromZero));
    /// assert!(-10 == (-6).round_to(10, Tie::AwayFromZero));
    /// ```
    AwayFromZero,

    /// Round half toward even.
    ///
    /// “Even” has a special meaning here since we only care about round values.
    /// If we are rounding to the nearest 10, then 0 is even, 10 is odd, 20 is
    /// even, and so on.
    ///
    /// If we are rounding to whole numbers, then even and odd have the
    /// conventional meaning.
    ///
    /// In general, a multiple of factor _n_ is even if `(n / factor) % 2 == 0`.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use roundable::{Roundable, Tie};
    ///
    /// assert!(20 == 15.round_to(10, Tie::TowardEven));
    /// assert!(0 == 5.round_to(10, Tie::TowardEven));
    /// assert!(0 == (-5).round_to(10, Tie::TowardEven));
    /// assert!(-20 == (-15).round_to(10, Tie::TowardEven));
    ///
    /// // Other values are unaffected:
    /// assert!(0 == 4.round_to(10, Tie::TowardEven));
    /// assert!(10 == 6.round_to(10, Tie::TowardEven));
    /// assert!(0 == (-4).round_to(10, Tie::TowardEven));
    /// assert!(-10 == (-6).round_to(10, Tie::TowardEven));
    /// ```
    TowardEven,

    /// Round half toward odd.
    ///
    /// “Odd” has a special meaning here since we only care about round values.
    /// If we are rounding to the nearest 10, then 0 is even, 10 is odd, 20 is
    /// even, and so on.
    ///
    /// If we are rounding to whole numbers, then even and odd have the
    /// conventional meaning.
    ///
    /// In general, a multiple of factor _n_ is odd if `(n / factor) % 2 != 0`.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use roundable::{Roundable, Tie};
    ///
    /// assert!(10 == 15.round_to(10, Tie::TowardOdd));
    /// assert!(10 == 5.round_to(10, Tie::TowardOdd));
    /// assert!(-10 == (-5).round_to(10, Tie::TowardOdd));
    /// assert!(-10 == (-15).round_to(10, Tie::TowardOdd));
    ///
    /// // Other values are unaffected:
    /// assert!(0 == 4.round_to(10, Tie::TowardOdd));
    /// assert!(10 == 6.round_to(10, Tie::TowardOdd));
    /// assert!(0 == (-4).round_to(10, Tie::TowardOdd));
    /// assert!(-10 == (-6).round_to(10, Tie::TowardOdd));
    /// ```
    TowardOdd,
}

/// Methods to round to an arbitrary factor.
///
/// For example, you might wish to round an integer to the nearest ten or
/// nearest hundred:
///
/// ```rust
/// use roundable::{Roundable, Tie};
///
/// assert!(310 == 314.round_to(10, Tie::Up));
/// assert!(Some(300) == 314.try_round_to(100, Tie::Up));
/// ```
pub trait Roundable: Sized {
    /// Round to the nearest `factor`. Panics if there is an overflow.
    ///
    /// Ties (values exactly halfway between to round numbers) are handled
    /// according to the second parameter. For traditional rounding use
    /// [`Tie::Up`], which will cause ties to be resolved by choosing the higher
    /// round number.
    ///
    /// See [`Tie`] for other tie strategies.
    ///
    /// ```rust
    /// use roundable::{Roundable, Tie};
    ///
    /// assert!(315 == 314.round_to(5, Tie::Up));
    /// assert!(-10 == (-15).round_to(10, Tie::Up));
    /// ```
    ///
    /// `255u8` can’t be rounded to the nearest 10 (which would be 260) because
    /// 260 won’t fit in a `u8`:
    ///
    /// ```rust,should_panic
    /// # use roundable::{Roundable, Tie};
    /// let _ = 255u8.round_to(10u8, Tie::Up);
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if `factor` is not positive, e.g. if it’s 0, or if rounding would
    /// return a value that does not fit in the return type.
    #[must_use]
    fn round_to(self, factor: Self, tie: Tie) -> Self {
        self.try_round_to(factor, tie)
            .expect("overflow while rounding")
    }

    /// Round to the nearest `factor`. Returns `None` if there is an overflow.
    ///
    /// Ties (values exactly halfway between to round numbers) are handled
    /// according to the second parameter. For traditional rounding use
    /// [`Tie::Up`], which will cause ties to be resolved by choosing the higher
    /// round number.
    ///
    /// See [`Tie`] for other tie strategies.
    ///
    /// ```rust
    /// use roundable::{Roundable, Tie};
    ///
    /// assert!(Some(315) == 314.try_round_to(5, Tie::Up));
    /// assert!(Some(-10) == (-15).try_round_to(10, Tie::Up));
    /// ```
    ///
    /// `255u8` can’t be rounded to the nearest 10 (which would be 260) because
    /// 260 won’t fit in a `u8`:
    ///
    /// ```rust
    /// # use roundable::{Roundable, Tie};
    /// assert!(None == 255u8.try_round_to(10, Tie::Up));
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if `factor` is not positive, e.g. if it’s 0.
    #[must_use]
    fn try_round_to(self, factor: Self, tie: Tie) -> Option<Self>;
}

/// Implement rounding for integer types.
macro_rules! roundable_integer {
    ($($ty:ident)+) => {$(
        impl Roundable for $ty {
            #[allow(
                clippy::integer_division,
                clippy::arithmetic_side_effects,
                unused_comparisons,
            )]
            fn try_round_to(self, factor: Self, tie: Tie) -> Option<Self> {
                assert!(factor > 0, "try_round_to() requires positive factor");

                let remainder = self % factor;

                // Safe: remainder has the same sign as self, so subtracting
                // remainder will always be closer to 0. Also, remainder is
                // always between 0 and self, so it base can never switch signs.
                let base = self - remainder;

                let use_smaller = || {
                    match tie {
                        Tie::Up => false,
                        Tie::Down => true,
                        Tie::TowardZero => self > 0,
                        Tie::AwayFromZero => self < 0,
                        Tie::TowardEven =>
                            ((base / factor) % 2 == 0) ^ (self < 0),
                        Tie::TowardOdd =>
                            ((base / factor) % 2 != 0) ^ (self < 0),
                    }
                };

                if self > 0 {
                    // Add factor % 2 to make things work when factor is odd.
                    if remainder < factor / 2 + factor % 2
                        || ( remainder == factor / 2 && use_smaller() )
                    {
                        Some(base)
                    } else {
                        base.checked_add(factor)
                    }
                } else { // self <= 0
                    // Add factor % 2 to make things work when factor is odd.
                    // Safe: 0 ≤ -remainder ≤ factor
                    if remainder + factor < factor / 2 + factor % 2
                        || ( remainder + factor / 2 + factor % 2 == 0
                            && use_smaller() )
                    {
                        base.checked_sub(factor)
                    } else {
                        Some(base)
                    }
                }
            }
        }
    )+}
}

roundable_integer!(u8 u16 u32 u64 u128 usize);
roundable_integer!(i8 i16 i32 i64 i128 isize);

/// Implement rounding for floating point types.
macro_rules! roundable_float {
    ($($ty:ident)+) => {$(
        impl Roundable for $ty {
            #[allow(clippy::arithmetic_side_effects, unused_comparisons)]
            fn try_round_to(self, factor: Self, tie: Tie) -> Option<Self> {
                /// `$ty::abs(self)` is not in std.
                fn abs ( v: $ty ) -> $ty {
                    if v < 0.0 {
                        -v
                    } else {
                        v
                    }
                }

                assert!(
                    factor > 0.0,
                    "try_round_to() requires positive factor",
                );

                let remainder = self % factor;
                let base = self - remainder;

                let use_smaller = || {
                    match tie {
                        Tie::Up => false,
                        Tie::Down => true,
                        Tie::TowardZero => self > 0.0,
                        Tie::AwayFromZero => self < 0.0,
                        Tie::TowardEven =>
                            (abs((base / factor) % 2.0) < Self::EPSILON)
                            ^ (self < 0.0),
                        Tie::TowardOdd =>
                            (abs((base / factor) % 2.0) >= Self::EPSILON)
                            ^ (self < 0.0),
                    }
                };

                if self > 0.0 {
                    if remainder - factor / 2.0 < -Self::EPSILON
                        || ( abs(remainder - factor / 2.0) < Self::EPSILON
                            && use_smaller() )
                    {
                        Some(base)
                    } else {
                        Some(base + factor)
                    }
                } else { // self <= 0.0
                    if remainder - factor / 2.0 + factor < -Self::EPSILON
                        || ( abs(remainder + factor / 2.0) < Self::EPSILON
                            && use_smaller() )
                    {
                        Some(base - factor)
                    } else {
                        Some(base)
                    }
                }
            }
        }
    )+}
}

roundable_float!(f32 f64);

#[cfg(test)]
#[allow(
    clippy::cognitive_complexity,
    clippy::integer_division,
    clippy::float_cmp
)]
mod tests {
    use super::*;
    use assert2::check;

    #[test]
    fn round_small_unsigned_integer() {
        check!(10 == 10.round_to(1, Tie::Up));

        check!(0 == 0.round_to(2, Tie::Up));
        check!(2 == 1.round_to(2, Tie::Up));
        check!(2 == 2.round_to(2, Tie::Up));
        check!(4 == 3.round_to(2, Tie::Up));
        check!(4 == 4.round_to(2, Tie::Up));

        check!(0 == 0.round_to(3, Tie::Up));
        check!(0 == 1.round_to(3, Tie::Up));
        check!(3 == 2.round_to(3, Tie::Up));
        check!(3 == 3.round_to(3, Tie::Up));
    }

    #[test]
    fn round_small_signed_integer() {
        check!(10 == 10i8.round_to(1, Tie::Up));

        check!(0 == 0i8.round_to(2, Tie::Up));
        check!(2 == 1i8.round_to(2, Tie::Up));
        check!(2 == 2i8.round_to(2, Tie::Up));
        check!(4 == 3i8.round_to(2, Tie::Up));
        check!(4 == 4i8.round_to(2, Tie::Up));

        check!(0 == 0i8.round_to(3, Tie::Up));
        check!(0 == 1i8.round_to(3, Tie::Up));
        check!(3 == 2i8.round_to(3, Tie::Up));
        check!(3 == 3i8.round_to(3, Tie::Up));

        check!(-10 == (-10i8).round_to(1, Tie::Up));

        check!(0 == (-1i8).round_to(2, Tie::Up));
        check!(-2 == (-2i8).round_to(2, Tie::Up));
        check!(-2 == (-3i8).round_to(2, Tie::Up));
        check!(-4 == (-4i8).round_to(2, Tie::Up));

        check!(0 == (-1i8).round_to(3, Tie::Up));
        check!(-3 == (-2i8).round_to(3, Tie::Up));
        check!(-3 == (-3i8).round_to(3, Tie::Up));
    }

    #[test]
    fn round_integer_tie_up() {
        check!(10 == 10.round_to(1, Tie::Up));

        check!(0 == 0.round_to(2, Tie::Up));
        check!(2 == 1.round_to(2, Tie::Up));
        check!(2 == 2.round_to(2, Tie::Up));
        check!(4 == 3.round_to(2, Tie::Up));
        check!(4 == 4.round_to(2, Tie::Up));

        check!(0 == 0.round_to(3, Tie::Up));
        check!(0 == 1.round_to(3, Tie::Up));
        check!(3 == 2.round_to(3, Tie::Up));
        check!(3 == 3.round_to(3, Tie::Up));

        check!(-10 == (-10).round_to(1, Tie::Up));

        check!(0 == (-1).round_to(2, Tie::Up));
        check!(-2 == (-2).round_to(2, Tie::Up));
        check!(-2 == (-3).round_to(2, Tie::Up));
        check!(-4 == (-4).round_to(2, Tie::Up));

        check!(0 == (-1).round_to(3, Tie::Up));
        check!(-3 == (-2).round_to(3, Tie::Up));
        check!(-3 == (-3).round_to(3, Tie::Up));
    }

    #[test]
    fn round_integer_tie_down() {
        check!(10 == 10.round_to(1, Tie::Down));

        check!(0 == 0.round_to(2, Tie::Down));
        check!(0 == 1.round_to(2, Tie::Down));
        check!(2 == 2.round_to(2, Tie::Down));
        check!(2 == 3.round_to(2, Tie::Down));
        check!(4 == 4.round_to(2, Tie::Down));

        check!(0 == 0.round_to(3, Tie::Down));
        check!(0 == 1.round_to(3, Tie::Down));
        check!(3 == 2.round_to(3, Tie::Down));
        check!(3 == 3.round_to(3, Tie::Down));

        check!(-10 == (-10).round_to(1, Tie::Down));

        check!(-2 == (-1).round_to(2, Tie::Down));
        check!(-2 == (-2).round_to(2, Tie::Down));
        check!(-4 == (-3).round_to(2, Tie::Down));
        check!(-4 == (-4).round_to(2, Tie::Down));

        check!(0 == (-1).round_to(3, Tie::Down));
        check!(-3 == (-2).round_to(3, Tie::Down));
        check!(-3 == (-3).round_to(3, Tie::Down));
    }

    #[test]
    fn round_integer_tie_toward_zero() {
        check!(10 == 10.round_to(1, Tie::TowardZero));

        check!(0 == 0.round_to(2, Tie::TowardZero));
        check!(0 == 1.round_to(2, Tie::TowardZero));
        check!(2 == 2.round_to(2, Tie::TowardZero));
        check!(2 == 3.round_to(2, Tie::TowardZero));
        check!(4 == 4.round_to(2, Tie::TowardZero));

        check!(0 == 0.round_to(3, Tie::TowardZero));
        check!(0 == 1.round_to(3, Tie::TowardZero));
        check!(3 == 2.round_to(3, Tie::TowardZero));
        check!(3 == 3.round_to(3, Tie::TowardZero));

        check!(-10 == (-10).round_to(1, Tie::TowardZero));

        check!(0 == (-1).round_to(2, Tie::TowardZero));
        check!(-2 == (-2).round_to(2, Tie::TowardZero));
        check!(-2 == (-3).round_to(2, Tie::TowardZero));
        check!(-4 == (-4).round_to(2, Tie::TowardZero));

        check!(0 == (-1).round_to(3, Tie::TowardZero));
        check!(-3 == (-2).round_to(3, Tie::TowardZero));
        check!(-3 == (-3).round_to(3, Tie::TowardZero));
    }

    #[test]
    fn round_integer_tie_away_from_zero() {
        check!(10 == 10.round_to(1, Tie::AwayFromZero));

        check!(0 == 0.round_to(2, Tie::AwayFromZero));
        check!(2 == 1.round_to(2, Tie::AwayFromZero));
        check!(2 == 2.round_to(2, Tie::AwayFromZero));
        check!(4 == 3.round_to(2, Tie::AwayFromZero));
        check!(4 == 4.round_to(2, Tie::AwayFromZero));

        check!(0 == 0.round_to(3, Tie::AwayFromZero));
        check!(0 == 1.round_to(3, Tie::AwayFromZero));
        check!(3 == 2.round_to(3, Tie::AwayFromZero));
        check!(3 == 3.round_to(3, Tie::AwayFromZero));

        check!(-10 == (-10).round_to(1, Tie::AwayFromZero));

        check!(-2 == (-1).round_to(2, Tie::AwayFromZero));
        check!(-2 == (-2).round_to(2, Tie::AwayFromZero));
        check!(-4 == (-3).round_to(2, Tie::AwayFromZero));
        check!(-4 == (-4).round_to(2, Tie::AwayFromZero));

        check!(0 == (-1).round_to(3, Tie::AwayFromZero));
        check!(-3 == (-2).round_to(3, Tie::AwayFromZero));
        check!(-3 == (-3).round_to(3, Tie::AwayFromZero));
    }

    #[test]
    fn round_integer_tie_toward_even() {
        check!(10 == 10.round_to(1, Tie::TowardEven));

        check!(0 == 0.round_to(2, Tie::TowardEven));
        check!(0 == 1.round_to(2, Tie::TowardEven));
        check!(2 == 2.round_to(2, Tie::TowardEven));
        check!(4 == 3.round_to(2, Tie::TowardEven));
        check!(4 == 4.round_to(2, Tie::TowardEven));

        check!(0 == 0.round_to(3, Tie::TowardEven));
        check!(0 == 1.round_to(3, Tie::TowardEven));
        check!(3 == 2.round_to(3, Tie::TowardEven));
        check!(3 == 3.round_to(3, Tie::TowardEven));

        check!(-10 == (-10).round_to(1, Tie::TowardEven));

        check!(0 == (-1).round_to(2, Tie::TowardEven));
        check!(-2 == (-2).round_to(2, Tie::TowardEven));
        check!(-4 == (-3).round_to(2, Tie::TowardEven));
        check!(-4 == (-4).round_to(2, Tie::TowardEven));

        check!(0 == (-1).round_to(3, Tie::TowardEven));
        check!(-3 == (-2).round_to(3, Tie::TowardEven));
        check!(-3 == (-3).round_to(3, Tie::TowardEven));
    }

    #[test]
    fn round_integer_tie_toward_odd() {
        check!(10 == 10.round_to(1, Tie::TowardOdd));

        check!(0 == 0.round_to(2, Tie::TowardOdd));
        check!(2 == 1.round_to(2, Tie::TowardOdd));
        check!(2 == 2.round_to(2, Tie::TowardOdd));
        check!(2 == 3.round_to(2, Tie::TowardOdd));
        check!(4 == 4.round_to(2, Tie::TowardOdd));

        check!(0 == 0.round_to(3, Tie::TowardOdd));
        check!(0 == 1.round_to(3, Tie::TowardOdd));
        check!(3 == 2.round_to(3, Tie::TowardOdd));
        check!(3 == 3.round_to(3, Tie::TowardOdd));

        check!(-10 == (-10).round_to(1, Tie::TowardOdd));

        check!(-2 == (-1).round_to(2, Tie::TowardOdd));
        check!(-2 == (-2).round_to(2, Tie::TowardOdd));
        check!(-2 == (-3).round_to(2, Tie::TowardOdd));
        check!(-4 == (-4).round_to(2, Tie::TowardOdd));

        check!(0 == (-1).round_to(3, Tie::TowardOdd));
        check!(-3 == (-2).round_to(3, Tie::TowardOdd));
        check!(-3 == (-3).round_to(3, Tie::TowardOdd));
    }

    /// All tie behaviors.
    const TIE_BEHAVIORS: [Tie; 6] = [
        Tie::Up,
        Tie::Down,
        Tie::TowardZero,
        Tie::AwayFromZero,
        Tie::TowardEven,
        Tie::TowardOdd,
    ];

    #[test]
    fn round_max_integer() {
        // Tie behaviors should be irrelevant in all these cases.
        for behavior in TIE_BEHAVIORS {
            check!(0 == 10.round_to(u32::MAX, behavior));
            check!(0 == (u32::MAX / 2).round_to(u32::MAX, behavior));
            check!(u32::MAX == (u32::MAX / 2 + 1).round_to(u32::MAX, behavior));
            check!(u32::MAX == u32::MAX.round_to(u32::MAX, behavior));

            check!(0 == 10.round_to(i32::MAX, behavior));
            check!(0 == (i32::MAX / 2).round_to(i32::MAX, behavior));
            check!(i32::MAX == (i32::MAX / 2 + 1).round_to(i32::MAX, behavior));
            check!(i32::MAX == i32::MAX.round_to(i32::MAX, behavior));
        }
    }

    #[test]
    fn round_min_integer() {
        // Tie behaviors should be irrelevant in all these cases.
        for behavior in TIE_BEHAVIORS {
            check!(-i32::MAX == i32::MIN.round_to(i32::MAX, behavior));
            check!(-i32::MAX == (i32::MIN / 2).round_to(i32::MAX, behavior));
            check!(0 == (i32::MIN / 2 + 1).round_to(i32::MAX, behavior));
        }
    }

    #[test]
    fn round_largest_integer_tie() {
        check!(254 == 127u8.round_to(254, Tie::Up));
        check!(0 == 127u8.round_to(254, Tie::Down));
        check!(0 == 127u8.round_to(254, Tie::TowardZero));
        check!(254 == 127u8.round_to(254, Tie::AwayFromZero));
        check!(0 == 127u8.round_to(254, Tie::TowardEven));
        check!(254 == 127u8.round_to(254, Tie::TowardOdd));
    }

    #[test]
    fn round_all_u8s() {
        // Just make sure they don’t panic.
        for behavior in TIE_BEHAVIORS {
            for value in u8::MIN..=u8::MAX {
                for factor in 1..=u8::MAX {
                    let _ = value.try_round_to(factor, behavior);
                }
            }
        }
    }

    #[test]
    fn round_all_i8s() {
        // Just make sure they don’t panic.
        for behavior in TIE_BEHAVIORS {
            for value in i8::MIN..=i8::MAX {
                for factor in 1..=i8::MAX {
                    let _ = value.try_round_to(factor, behavior);
                }
            }
        }
    }

    #[test]
    #[should_panic(expected = "try_round_to() requires positive factor")]
    fn round_integer_zero_factor() {
        let _ = 0.round_to(0, Tie::Up);
    }

    #[test]
    #[should_panic(expected = "try_round_to() requires positive factor")]
    fn round_integer_negative_factor() {
        let _ = 0.round_to(-1, Tie::Up);
    }

    #[test]
    fn round_float_tie_up() {
        check!(10.0 == 10.0.round_to(1.0, Tie::Up));

        check!(0.0 == 0.0.round_to(2.0, Tie::Up));
        check!(2.0 == 1.0.round_to(2.0, Tie::Up));
        check!(2.0 == 2.0.round_to(2.0, Tie::Up));
        check!(4.0 == 3.0.round_to(2.0, Tie::Up));
        check!(4.0 == 4.0.round_to(2.0, Tie::Up));

        check!(0.0 == 0.0.round_to(3.0, Tie::Up));
        check!(0.0 == 1.0.round_to(3.0, Tie::Up));
        check!(3.0 == 1.5.round_to(3.0, Tie::Up));
        check!(3.0 == 2.0.round_to(3.0, Tie::Up));
        check!(3.0 == 3.0.round_to(3.0, Tie::Up));

        check!(-10.0 == (-10.0).round_to(1.0, Tie::Up));

        check!(0.0 == (-1.0).round_to(2.0, Tie::Up));
        check!(-2.0 == (-2.0).round_to(2.0, Tie::Up));
        check!(-2.0 == (-3.0).round_to(2.0, Tie::Up));
        check!(-4.0 == (-4.0).round_to(2.0, Tie::Up));

        check!(0.0 == (-1.0).round_to(3.0, Tie::Up));
        check!(0.0 == (-1.5).round_to(3.0, Tie::Up));
        check!(-3.0 == (-2.0).round_to(3.0, Tie::Up));
        check!(-3.0 == (-3.0).round_to(3.0, Tie::Up));
    }

    #[test]
    fn round_float_tie_down() {
        check!(10.0 == 10.0.round_to(1.0, Tie::Down));

        check!(0.0 == 0.0.round_to(2.0, Tie::Down));
        check!(0.0 == 1.0.round_to(2.0, Tie::Down));
        check!(2.0 == 2.0.round_to(2.0, Tie::Down));
        check!(2.0 == 3.0.round_to(2.0, Tie::Down));
        check!(4.0 == 4.0.round_to(2.0, Tie::Down));

        check!(0.0 == 0.0.round_to(3.0, Tie::Down));
        check!(0.0 == 1.0.round_to(3.0, Tie::Down));
        check!(0.0 == 1.5.round_to(3.0, Tie::Down));
        check!(3.0 == 2.0.round_to(3.0, Tie::Down));
        check!(3.0 == 3.0.round_to(3.0, Tie::Down));

        check!(-10.0 == (-10.0).round_to(1.0, Tie::Down));

        check!(-2.0 == (-1.0).round_to(2.0, Tie::Down));
        check!(-2.0 == (-2.0).round_to(2.0, Tie::Down));
        check!(-4.0 == (-3.0).round_to(2.0, Tie::Down));
        check!(-4.0 == (-4.0).round_to(2.0, Tie::Down));

        check!(0.0 == (-1.0).round_to(3.0, Tie::Down));
        check!(-3.0 == (-1.5).round_to(3.0, Tie::Down));
        check!(-3.0 == (-2.0).round_to(3.0, Tie::Down));
        check!(-3.0 == (-3.0).round_to(3.0, Tie::Down));
    }

    #[test]
    fn round_float_tie_toward_zero() {
        check!(10.0 == 10.0.round_to(1.0, Tie::TowardZero));

        check!(0.0 == 0.0.round_to(2.0, Tie::TowardZero));
        check!(0.0 == 1.0.round_to(2.0, Tie::TowardZero));
        check!(2.0 == 2.0.round_to(2.0, Tie::TowardZero));
        check!(2.0 == 3.0.round_to(2.0, Tie::TowardZero));
        check!(4.0 == 4.0.round_to(2.0, Tie::TowardZero));

        check!(0.0 == 0.0.round_to(3.0, Tie::TowardZero));
        check!(0.0 == 1.0.round_to(3.0, Tie::TowardZero));
        check!(0.0 == 1.5.round_to(3.0, Tie::TowardZero));
        check!(3.0 == 2.0.round_to(3.0, Tie::TowardZero));
        check!(3.0 == 3.0.round_to(3.0, Tie::TowardZero));

        check!(-10.0 == (-10.0).round_to(1.0, Tie::TowardZero));

        check!(0.0 == (-1.0).round_to(2.0, Tie::TowardZero));
        check!(-2.0 == (-2.0).round_to(2.0, Tie::TowardZero));
        check!(-2.0 == (-3.0).round_to(2.0, Tie::TowardZero));
        check!(-4.0 == (-4.0).round_to(2.0, Tie::TowardZero));

        check!(0.0 == (-1.0).round_to(3.0, Tie::TowardZero));
        check!(0.0 == (-1.5).round_to(3.0, Tie::TowardZero));
        check!(-3.0 == (-2.0).round_to(3.0, Tie::TowardZero));
        check!(-3.0 == (-3.0).round_to(3.0, Tie::TowardZero));
    }

    #[test]
    fn round_float_tie_away_from_zero() {
        check!(10.0 == 10.0.round_to(1.0, Tie::AwayFromZero));

        check!(0.0 == 0.0.round_to(2.0, Tie::AwayFromZero));
        check!(2.0 == 1.0.round_to(2.0, Tie::AwayFromZero));
        check!(2.0 == 2.0.round_to(2.0, Tie::AwayFromZero));
        check!(4.0 == 3.0.round_to(2.0, Tie::AwayFromZero));
        check!(4.0 == 4.0.round_to(2.0, Tie::AwayFromZero));

        check!(0.0 == 0.0.round_to(3.0, Tie::AwayFromZero));
        check!(0.0 == 1.0.round_to(3.0, Tie::AwayFromZero));
        check!(3.0 == 1.5.round_to(3.0, Tie::AwayFromZero));
        check!(3.0 == 2.0.round_to(3.0, Tie::AwayFromZero));
        check!(3.0 == 3.0.round_to(3.0, Tie::AwayFromZero));

        check!(-10.0 == (-10.0).round_to(1.0, Tie::AwayFromZero));

        check!(-2.0 == (-1.0).round_to(2.0, Tie::AwayFromZero));
        check!(-2.0 == (-2.0).round_to(2.0, Tie::AwayFromZero));
        check!(-4.0 == (-3.0).round_to(2.0, Tie::AwayFromZero));
        check!(-4.0 == (-4.0).round_to(2.0, Tie::AwayFromZero));

        check!(0.0 == (-1.0).round_to(3.0, Tie::AwayFromZero));
        check!(-3.0 == (-1.5).round_to(3.0, Tie::AwayFromZero));
        check!(-3.0 == (-2.0).round_to(3.0, Tie::AwayFromZero));
        check!(-3.0 == (-3.0).round_to(3.0, Tie::AwayFromZero));
    }

    #[test]
    fn round_float_tie_toward_even() {
        check!(10.0 == 10.0.round_to(1.0, Tie::TowardEven));

        check!(0.0 == 0.0.round_to(2.0, Tie::TowardEven));
        check!(0.0 == 1.0.round_to(2.0, Tie::TowardEven));
        check!(2.0 == 2.0.round_to(2.0, Tie::TowardEven));
        check!(4.0 == 3.0.round_to(2.0, Tie::TowardEven));
        check!(4.0 == 4.0.round_to(2.0, Tie::TowardEven));

        check!(0.0 == 0.0.round_to(3.0, Tie::TowardEven));
        check!(0.0 == 1.0.round_to(3.0, Tie::TowardEven));
        check!(0.0 == 1.5.round_to(3.0, Tie::TowardEven));
        check!(3.0 == 2.0.round_to(3.0, Tie::TowardEven));
        check!(3.0 == 3.0.round_to(3.0, Tie::TowardEven));

        check!(-10.0 == (-10.0).round_to(1.0, Tie::TowardEven));

        check!(0.0 == (-1.0).round_to(2.0, Tie::TowardEven));
        check!(-2.0 == (-2.0).round_to(2.0, Tie::TowardEven));
        check!(-4.0 == (-3.0).round_to(2.0, Tie::TowardEven));
        check!(-4.0 == (-4.0).round_to(2.0, Tie::TowardEven));

        check!(0.0 == (-1.0).round_to(3.0, Tie::TowardEven));
        check!(0.0 == (-1.5).round_to(3.0, Tie::TowardEven));
        check!(-3.0 == (-2.0).round_to(3.0, Tie::TowardEven));
        check!(-3.0 == (-3.0).round_to(3.0, Tie::TowardEven));
    }

    #[test]
    fn round_float_tie_toward_odd() {
        check!(10.0 == 10.0.round_to(1.0, Tie::TowardOdd));

        check!(0.0 == 0.0.round_to(2.0, Tie::TowardOdd));
        check!(2.0 == 1.0.round_to(2.0, Tie::TowardOdd));
        check!(2.0 == 2.0.round_to(2.0, Tie::TowardOdd));
        check!(2.0 == 3.0.round_to(2.0, Tie::TowardOdd));
        check!(4.0 == 4.0.round_to(2.0, Tie::TowardOdd));

        check!(0.0 == 0.0.round_to(3.0, Tie::TowardOdd));
        check!(0.0 == 1.0.round_to(3.0, Tie::TowardOdd));
        check!(3.0 == 1.5.round_to(3.0, Tie::TowardOdd));
        check!(3.0 == 2.0.round_to(3.0, Tie::TowardOdd));
        check!(3.0 == 3.0.round_to(3.0, Tie::TowardOdd));

        check!(-10.0 == (-10.0).round_to(1.0, Tie::TowardOdd));

        check!(-2.0 == (-1.0).round_to(2.0, Tie::TowardOdd));
        check!(-2.0 == (-2.0).round_to(2.0, Tie::TowardOdd));
        check!(-2.0 == (-3.0).round_to(2.0, Tie::TowardOdd));
        check!(-4.0 == (-4.0).round_to(2.0, Tie::TowardOdd));

        check!(0.0 == (-1.0).round_to(3.0, Tie::TowardOdd));
        check!(-3.0 == (-1.5).round_to(3.0, Tie::TowardOdd));
        check!(-3.0 == (-2.0).round_to(3.0, Tie::TowardOdd));
        check!(-3.0 == (-3.0).round_to(3.0, Tie::TowardOdd));
    }

    #[test]
    fn round_float_to_ten() {
        check!(10.0 == 14.9.round_to(10.0, Tie::Up));
        check!(20.0 == 15.0.round_to(10.0, Tie::Up));
        check!(20.0 == 15.1.round_to(10.0, Tie::Up));

        check!(-10.0 == (-14.9).round_to(10.0, Tie::Up));
        check!(-10.0 == (-15.0).round_to(10.0, Tie::Up));
        check!(-20.0 == (-15.1).round_to(10.0, Tie::Up));
    }

    #[test]
    fn round_awkward_float_tie() {
        check!(0.4 == 0.3.round_to(0.2, Tie::Up));
        check!(0.2 == 0.3.round_to(0.2, Tie::Down));
        check!(0.2 == 0.3.round_to(0.2, Tie::TowardZero));
        check!(0.4 == 0.3.round_to(0.2, Tie::AwayFromZero));
        check!(0.4 == 0.3.round_to(0.2, Tie::TowardEven));
        check!(0.2 == 0.3.round_to(0.2, Tie::TowardOdd));
    }

    #[test]
    fn round_max_float() {
        check!(0.0 == 10.0.round_to(f32::MAX, Tie::Up));
        check!(0.0 == (f32::MAX * 0.4).round_to(f32::MAX, Tie::Up));
        check!(f32::MAX == (f32::MAX * 0.5).round_to(f32::MAX, Tie::Up));
        check!(f32::MAX == (f32::MAX * 0.6).round_to(f32::MAX, Tie::Up));
    }

    #[test]
    fn round_min_float() {
        check!(-f32::MAX == f32::MIN.round_to(f32::MAX, Tie::Up));
        check!(0.0 == (f32::MIN * 0.4).round_to(f32::MAX, Tie::Up));
        check!(0.0 == (f32::MIN * 0.5).round_to(f32::MAX, Tie::Up));
        check!(-f32::MAX == (f32::MIN * 0.6).round_to(f32::MAX, Tie::Up));
    }

    #[test]
    #[should_panic(expected = "try_round_to() requires positive factor")]
    fn round_float_zero_factor() {
        let _ = 0.0.round_to(0.0, Tie::Up);
    }

    #[test]
    #[should_panic(expected = "try_round_to() requires positive factor")]
    fn round_float_negative_factor() {
        let _ = 0.0.round_to(-1.0, Tie::Up);
    }
}
