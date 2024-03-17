//! Implement `Roundable` for floats.

use crate::{Roundable, Tie};

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
#[allow(clippy::cognitive_complexity, clippy::float_cmp)]
mod tests {
    use super::*;
    use assert2::check;

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
