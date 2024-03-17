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
//!  * [`Roundable::round_up_to(factor)`](Roundable::round_up_to())
//!    (panics on overflow; rounds ties to the larger round number)
//!
//! ### Example
//!
//! ```rust
//! use roundable::{Roundable, Tie};
//!
//! assert!(310 == 314.round_up_to(10));
//! assert!(300.0 == 314.1.round_up_to(100.0));
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
//! [`Duration`](core::time::Duration) can be rounded to a `Duration` factor,
//! just like a number type. For convenience, there are a number of
//! [constants](#constants) that can be used to make rounding `Duration` easier.
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
mod float;
mod int;

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
/// assert!(320 == 315.round_up_to(10));
/// assert!(310 == 315.round_to(10, Tie::Down));
/// assert!(Some(300) == 314.try_round_to(100, Tie::Up));
/// ```
pub trait Roundable: Sized {
    /// Traditional round to the nearest `factor`. Panics on overflow.
    ///
    /// Ties (values exactly halfway between to round numbers) are handled by
    /// choosing the larger round number (see [`Tie::Up`]).
    ///
    /// See [`Self::round_to()`] to use other tie strategies. This is exactly
    /// equivalent to `value.round_to(factor, Tie::Up)`.
    ///
    /// ```rust
    /// use roundable::Roundable;
    ///
    /// assert!(10 == 14.round_up_to(10));
    /// assert!(20 == 15.round_up_to(10));
    /// assert!(-10 == (-15).round_up_to(10));
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
    fn round_up_to(self, factor: Self) -> Self {
        self.round_to(factor, Tie::Up)
    }

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
