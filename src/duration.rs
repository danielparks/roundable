//! Functions, constants, etc. related to Duration.

use crate::Roundable;
use core::time::Duration;

/// A microsecond. Useful for rounding [`Duration`].
///
/// ```rust
/// use roundable::{MICROSECOND, Roundable};
/// use std::time::Duration;
///
/// assert!(MICROSECOND == Duration::from_nanos(500).round_to(MICROSECOND));
/// ```
pub const MICROSECOND: Duration = Duration::from_micros(1);

/// A millisecond. Useful for rounding [`Duration`].
///
/// ```rust
/// use roundable::{MILLISECOND, Roundable};
/// use std::time::Duration;
///
/// assert!(MILLISECOND == Duration::from_micros(500).round_to(MILLISECOND));
/// ```
pub const MILLISECOND: Duration = Duration::from_millis(1);

/// A second. Useful for rounding [`Duration`].
///
/// ```rust
/// use roundable::{SECOND, Roundable};
/// use std::time::Duration;
///
/// assert!(SECOND == Duration::from_millis(500).round_to(SECOND));
/// ```
pub const SECOND: Duration = Duration::from_secs(1);

/// A minute. Useful for rounding [`Duration`].
///
/// ```rust
/// use roundable::{MINUTE, Roundable};
/// use std::time::Duration;
///
/// assert!(MINUTE == Duration::from_secs(30).round_to(MINUTE));
/// ```
pub const MINUTE: Duration = Duration::from_secs(60);

/// An hour. Useful for rounding [`Duration`].
///
/// ```rust
/// use roundable::{HOUR, Roundable};
/// use std::time::Duration;
///
/// assert!(HOUR == Duration::from_secs(30*60).round_to(HOUR));
/// ```
pub const HOUR: Duration = Duration::from_secs(60 * 60);

impl Roundable for Duration {
    fn try_round_to(self, factor: Self) -> Option<Self> {
        // Duration will always fit into u128 as nanoseconds.
        self.as_nanos()
            .try_round_to(factor.as_nanos())
            .map(nanos_to_duration)
    }
}

/// Create a new [`Duration`] from a `u128` of nanoseconds.
///
/// This is essentially just [`Duration::from_nanos()`] but it works on a
/// `u128`, which can represent any valid `Duration`.
///
/// # Panics
///
/// `Duration` can only represent 64 bits worth of seconds and less than 32 bits
/// (1e9) worth of nanoseconds, which works out to being roughly 94 bits. A
/// `u128` can therefore represent values that are invalid `Duration`s. This
/// will panic in those cases.
///
/// `nanos_to_duration(duration.as_nanos())` should never panic.
///
/// ```rust
/// use roundable::nanos_to_duration;
/// use std::time::Duration;
///
/// assert!(Duration::MAX == nanos_to_duration(Duration::MAX.as_nanos()));
/// assert!(Duration::ZERO == nanos_to_duration(Duration::ZERO.as_nanos()));
/// ```
#[must_use]
pub fn nanos_to_duration(total: u128) -> Duration {
    /// Just to make things clear.
    const NANOS_PER_SECOND: u128 = 1_000_000_000;
    #[allow(clippy::integer_division)]
    Duration::new(
        (total / NANOS_PER_SECOND).try_into().expect(
            "nanos_to_duration() overflowed seconds value for Duration",
        ),
        (total % NANOS_PER_SECOND).try_into().unwrap(),
    )
}

#[cfg(test)]
#[allow(clippy::cognitive_complexity)]
mod tests {
    use super::*;
    use assert2::check;

    /// Convenient alias for [`Duration::from_millis()`].
    const fn ms(ms: u64) -> Duration {
        Duration::from_millis(ms)
    }

    #[test]
    fn round_millisecond_to_nearest_millisecond() {
        check!(ms(10) == ms(10).round_to(MILLISECOND));

        check!(ms(10) == ms(10).round_to(ms(2)));
        check!(ms(10) == ms(9).round_to(ms(2)));

        check!(ms(9) == ms(9).round_to(ms(3)));
        check!(ms(9) == ms(10).round_to(ms(3)));
        check!(ms(12) == ms(11).round_to(ms(3)));
        check!(ms(12) == ms(12).round_to(ms(3)));
    }

    #[test]
    fn round_second_to_nearest_millisecond() {
        check!(ms(1_010) == ms(1_010).round_to(MILLISECOND));

        check!(ms(1_010) == ms(1_010).round_to(ms(2)));
        check!(ms(1_010) == ms(1_009).round_to(ms(2)));

        check!(ms(1_008) == ms(1_008).round_to(ms(3)));
        check!(ms(1_008) == ms(1_009).round_to(ms(3)));
        check!(ms(1_011) == ms(1_010).round_to(ms(3)));
        check!(ms(1_011) == ms(1_011).round_to(ms(3)));
    }

    #[test]
    fn round_second_to_nearest_second() {
        check!(ms(0) == ms(499).round_to(SECOND));
        check!(SECOND == ms(500).round_to(SECOND));
        check!(SECOND == ms(1_010).round_to(SECOND));
        check!(SECOND == ms(1_499).round_to(SECOND));
        check!(ms(2_000) == ms(1_500).round_to(SECOND));

        check!(ms(1_001) == ms(1_000).round_to(ms(1_001)));
        check!(ms(1_001) == ms(1_001).round_to(ms(1_001)));
        check!(ms(1_001) == ms(1_002).round_to(ms(1_001)));
    }

    #[test]
    fn round_to_giant_factor() {
        check!(ms(0) == ms(1_000_000).round_to(Duration::MAX));
        check!(Duration::MAX == Duration::MAX.round_to(Duration::MAX));
    }

    #[test]
    #[should_panic(expected = "try_round_to() requires positive factor")]
    fn round_to_zero_factor() {
        let _ = ms(10).round_to(ms(0));
    }

    /// Theoretical maximum Duration as nanoseconds (based on u64 for seconds).
    const NANOS_MAX: u128 = u64::MAX as u128 * 1_000_000_000 + 999_999_999;

    #[test]
    #[allow(clippy::arithmetic_side_effects)]
    fn nanos_to_duration_ok() {
        check!(Duration::ZERO == nanos_to_duration(0));
        check!(Duration::new(1, 1) == nanos_to_duration(1_000_000_001));

        // Check Duration::MAX two ways, since according it its docs it can vary
        // based on platform.
        check!(Duration::MAX == nanos_to_duration(Duration::MAX.as_nanos()));
        check!(
            Duration::new(u64::MAX, 999_999_999)
                == nanos_to_duration(NANOS_MAX)
        );
    }

    #[test]
    #[should_panic(
        expected = "nanos_to_duration() overflowed seconds value for Duration: TryFromIntError(())"
    )]
    fn nanos_to_duration_overflow() {
        let _ = nanos_to_duration(Duration::MAX.as_nanos() + 1);
    }

    #[test]
    #[should_panic(
        expected = "nanos_to_duration() overflowed seconds value for Duration: TryFromIntError(())"
    )]
    #[allow(clippy::arithmetic_side_effects)]
    fn nanos_to_duration_overflow_manual() {
        // One over the maximum duration. Just in case `Duration::MAX` is some
        // other value, since the docs say it can vary by platform even if it
        // currently is always `u64::MAX * 1_000_000_000 + 999_999_999`.
        let _ = nanos_to_duration(NANOS_MAX + 1);
    }
}
