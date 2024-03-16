//! # Round numbers and durations to a given factor
//!
//! This provides an implementation of rounding for various values, including
//! the the native number types and [`core::time::Duration`] (or
//! `std::time::Duration`).
//!
//! This crate is does not need `std` or `alloc` (it’s always in `no_std` mode).
//! No features need to be enabled or disabled.
//!
//! ```rust
//! use roundable::Roundable;
//!
//! assert!(310 == 314.round_to(10));
//! assert!(300.0 == 314.1.round_to(100.0));
//!
//! // To avoid panicking on overflow:
//! assert!(Some(260) == 255.try_round_to(10));
//! assert!(None == 255u8.try_round_to(10));
//! ```
//!
//! See [the list of constants][#constants] for a list of time units that make
//! rounding [`Duration`](core::time::Duration) easier.
//!
//! ```rust
//! use roundable::{SECOND, MINUTE, Roundable};
//! use std::time::Duration;
//!
//! assert!(Duration::ZERO == Duration::from_millis(314).round_to(SECOND));
//! assert!(MINUTE == Duration::from_millis(59_500).round_to(SECOND));
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

/// Methods to round the value to an arbitrary factor.
///
/// For example, you might wish to round an integer to the nearest 10s:
///
/// ```rust
/// use roundable::Roundable;
///
/// assert!(310 == 314.round_to(10));
/// assert!(Some(300) == 314.try_round_to(100));
/// ```
pub trait Roundable: Sized {
    /// Round to the nearest `factor`. Panics if there is an overflow.
    ///
    /// ```rust
    /// use roundable::Roundable;
    ///
    /// assert!(315 == 314.round_to(5));
    /// assert!(-10 == (-15).round_to(10));
    /// ```
    ///
    /// `255u8` can’t be rounded to the nearest 10 (which would be 260) because
    /// 260 won’t fit in a `u8`:
    ///
    /// ```rust,should_panic
    /// # use roundable::Roundable;
    /// let _ = 255u8.round_to(10u8);
    /// ```
    #[must_use]
    fn round_to(self, factor: Self) -> Self {
        self.try_round_to(factor).expect("overflow while rounding")
    }

    /// Round to the nearest `factor`. Returns `None` if there is an overflow.
    ///
    /// ```rust
    /// use roundable::Roundable;
    ///
    /// assert!(Some(315) == 314.try_round_to(5));
    /// assert!(Some(-10) == (-15).try_round_to(10));
    /// ```
    ///
    /// `255u8` can’t be rounded to the nearest 10 (which would be 260) because
    /// 260 won’t fit in a `u8`:
    ///
    /// ```rust
    /// # use roundable::Roundable;
    /// assert!(None == 255u8.try_round_to(10));
    /// ```
    #[must_use]
    fn try_round_to(self, factor: Self) -> Option<Self>;
}

/// Implement rounding for integer types.
macro_rules! roundable_integer {
    ($($ty:ident)+) => {$(
        impl Roundable for $ty {
            fn try_round_to(self, factor: Self) -> Option<Self> {
                // FIXME: make into error
                assert!(factor > 0, "try_round_to() requires positive factor");

                #[allow(clippy::arithmetic_side_effects)]
                let remainder = self % factor;

                // Safe: remainder has the same sign as self, so subtracting
                // remainder will always be closer to 0. Also, remainder is
                // always between 0 and self, so it base can never switch signs.
                #[allow(clippy::arithmetic_side_effects)]
                let base = self - remainder;

                #[allow(unused_comparisons)]
                if self < 0 {
                    #[allow(clippy::integer_division)]
                    #[allow(clippy::arithmetic_side_effects)]
                    if remainder < factor / 2 + factor % 2 - factor {
                        // FIXME: document how this can fail and test it
                        base.checked_sub(factor)
                    } else {
                        Some(base)
                    }
                } else {
                    #[allow(clippy::integer_division)]
                    #[allow(clippy::arithmetic_side_effects)]
                    if remainder < factor / 2 + factor % 2 {
                        Some(base)
                    } else {
                        // FIXME: document how this can fail and test it
                        base.checked_add(factor)
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
            fn try_round_to(self, factor: Self) -> Option<Self> {
                // FIXME: make into error
                assert!(factor > 0.0, "try_round_to() requires positive factor");

                #[allow(clippy::arithmetic_side_effects)]
                let remainder = self % factor;
                let remainder = if remainder < 0.0 {
                    factor + remainder
                } else {
                    remainder
                };

                // remainder <= self
                #[allow(clippy::arithmetic_side_effects)]
                let base = self - remainder;

                if remainder < factor / 2.0 {
                    Some(base)
                } else {
                    Some(base + factor)
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
        check!(10 == 10.round_to(1));

        check!(0 == 0.round_to(2));
        check!(2 == 1.round_to(2));
        check!(2 == 2.round_to(2));
        check!(4 == 3.round_to(2));
        check!(4 == 4.round_to(2));

        check!(0 == 0.round_to(3));
        check!(0 == 1.round_to(3));
        check!(3 == 2.round_to(3));
        check!(3 == 3.round_to(3));
    }

    #[test]
    fn round_small_signed_integer() {
        // FIXME: what if factor is negative?
        check!(10 == 10i8.round_to(1));

        check!(0 == 0i8.round_to(2));
        check!(2 == 1i8.round_to(2));
        check!(2 == 2i8.round_to(2));
        check!(4 == 3i8.round_to(2));
        check!(4 == 4i8.round_to(2));

        check!(0 == 0i8.round_to(3));
        check!(0 == 1i8.round_to(3));
        check!(3 == 2i8.round_to(3));
        check!(3 == 3i8.round_to(3));

        check!(-10 == (-10i8).round_to(1));

        check!(0 == (-1i8).round_to(2));
        check!(-2 == (-2i8).round_to(2));
        check!(-2 == (-3i8).round_to(2));
        check!(-4 == (-4i8).round_to(2));

        check!(0 == (-1i8).round_to(3));
        check!(-3 == (-2i8).round_to(3));
        check!(-3 == (-3i8).round_to(3));
    }

    #[test]
    fn round_integer_to_ten() {
        check!(10 == 14.round_to(10));
        check!(20 == 15.round_to(10));
        check!(20 == 16.round_to(10));

        check!(-10 == (-14).round_to(10));
        check!(-10 == (-15).round_to(10));
        check!(-20 == (-16).round_to(10));
    }

    #[test]
    fn round_max_integer() {
        check!(0 == 10.round_to(u32::MAX));
        check!(0 == (u32::MAX / 2).round_to(u32::MAX));
        check!(u32::MAX == (u32::MAX / 2 + 1).round_to(u32::MAX));
        check!(u32::MAX == u32::MAX.round_to(u32::MAX));

        check!(0 == 10.round_to(i32::MAX));
        check!(0 == (i32::MAX / 2).round_to(i32::MAX));
        check!(i32::MAX == (i32::MAX / 2 + 1).round_to(i32::MAX));
        check!(i32::MAX == i32::MAX.round_to(i32::MAX));
    }

    #[test]
    fn round_min_integer() {
        check!(-i32::MAX == i32::MIN.round_to(i32::MAX));
        check!(-i32::MAX == (i32::MIN / 2).round_to(i32::MAX));
        check!(0 == (i32::MIN / 2 + 1).round_to(i32::MAX));
    }

    #[test]
    fn round_small_float() {
        // FIXME: what if factor is negative?
        check!(10.0 == 10.0.round_to(1.0));

        check!(0.0 == 0.0.round_to(2.0));
        check!(2.0 == 1.0.round_to(2.0));
        check!(2.0 == 2.0.round_to(2.0));
        check!(4.0 == 3.0.round_to(2.0));
        check!(4.0 == 4.0.round_to(2.0));

        check!(0.0 == 0.0.round_to(3.0));
        check!(0.0 == 1.0.round_to(3.0));
        check!(3.0 == 2.0.round_to(3.0));
        check!(3.0 == 3.0.round_to(3.0));

        check!(-10.0 == (-10.0).round_to(1.0));

        check!(0.0 == (-1.0).round_to(2.0));
        check!(-2.0 == (-2.0).round_to(2.0));
        check!(-2.0 == (-3.0).round_to(2.0));
        check!(-4.0 == (-4.0).round_to(2.0));

        check!(0.0 == (-1.0).round_to(3.0));
        check!(-3.0 == (-2.0).round_to(3.0));
        check!(-3.0 == (-3.0).round_to(3.0));
    }

    #[test]
    fn round_float_to_ten() {
        check!(10.0 == 14.9.round_to(10.0));
        check!(20.0 == 15.0.round_to(10.0));
        check!(20.0 == 15.1.round_to(10.0));

        check!(-10.0 == (-14.9).round_to(10.0));
        check!(-10.0 == (-15.0).round_to(10.0));
        check!(-20.0 == (-15.1).round_to(10.0));
    }

    #[test]
    fn round_max_float() {
        check!(0.0 == 10.0.round_to(f32::MAX));
        check!(0.0 == (f32::MAX * 0.4).round_to(f32::MAX));
        check!(f32::MAX == (f32::MAX * 0.5).round_to(f32::MAX));
        check!(f32::MAX == (f32::MAX * 0.6).round_to(f32::MAX));
    }

    #[test]
    fn round_min_float() {
        check!(-f32::MAX == f32::MIN.round_to(f32::MAX));
        check!(0.0 == (f32::MIN * 0.4).round_to(f32::MAX));
        check!(0.0 == (f32::MIN * 0.5).round_to(f32::MAX));
        check!(-f32::MAX == (f32::MIN * 0.6).round_to(f32::MAX));
    }
}
