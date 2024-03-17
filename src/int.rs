//! Implement `Roundable` for integers.

use crate::{Roundable, Tie};

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

#[cfg(test)]
#[allow(clippy::cognitive_complexity, clippy::integer_division)]
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
}
