# Round numbers and durations to a given factor

[![docs.rs](https://img.shields.io/docsrs/roundable)][docs.rs]
[![Crates.io](https://img.shields.io/crates/v/roundable)][crates.io]
![Rust version 1.56.1+](https://img.shields.io/badge/Rust%20version-1.56.1%2B-success)

This provides an implementation of rounding for various values, including the
native number types and [`core::time::Duration`][`Duration`] (also known as
`std::time::Duration`).

The [`Roundable`] trait adds the following functions to roundable values:

 * [`Roundable::try_round_to(factor, tie_strategy)`][`try_round_to()`] (returns
   `None` on overflow)
 * [`Roundable::round_to(factor, tie_strategy)`][`round_to()`] (panics on
   overflow)
 * [`Roundable::round_up_to(factor)`](`round_up_to()`)
   (panics on overflow; rounds ties to the larger round number)

### Example

```rust
use roundable::{Roundable, Tie};

assert!(310 == 314.round_to(10, Tie::Up));
assert!(300.0 == 314.1.round_to(100.0, Tie::Up));

// To avoid panicking on overflow:
assert!(Some(260) == 255.try_round_to(10, Tie::Up));
assert!(None == 255u8.try_round_to(10, Tie::Up));
```

### Tie strategies

“Ties” are numbers exactly halfway between two round numbers, e.g. 0.5 when
rounding to the nearest whole number. Traditionally, ties are resolved by
picking the higher number, but there are other strategies. `Roundable` supports
the following rules:

  * [`Tie::Up`]: Round ties up (what most people consider correct).
  * [`Tie::Down`]: Round ties down.
  * [`Tie::TowardZero`]: Round ties toward zero.
  * [`Tie::AwayFromZero`]: Round ties away from zero.
  * [`Tie::TowardEven`]: Round ties toward the “even” number (see docs).
  * [`Tie::TowardOdd`]: Round ties toward the “odd” number (see docs).

### Rounding `Duration`

[`Duration`] can be rounded to a `Duration` factor, just like a number type. For
convenience, there are a number of [constants] that can be used to make rounding
`Duration` easier.

```rust
use roundable::{SECOND, MINUTE, Roundable, Tie};
use std::time::Duration;

assert!(Duration::ZERO == Duration::from_millis(314).round_to(SECOND, Tie::Up));
assert!(MINUTE == Duration::from_millis(59_500).round_to(SECOND, Tie::Up));
```

## `#![no_std]` by default

You can use this crate with or without `std` and `alloc`. You do not need to
enable or disable features either way.

## ⚠️ Development status

This is in active development. The API may be entirely rewritten. I am open to
[suggestions][issues].

## Minimum supported Rust version

Currently the minimum supported Rust version (MSRV) is **1.56.1**. Future
increases in the MSRV will require a major version bump.

## License

This project dual-licensed under the Apache 2 and MIT licenses. You may choose
to use either.

  * [Apache License, Version 2.0](LICENSE-APACHE)
  * [MIT license](LICENSE-MIT)

### Contributions

Unless you explicitly state otherwise, any contribution you submit as defined
in the Apache 2.0 license shall be dual licensed as above, without any
additional terms or conditions.

[docs.rs]: https://docs.rs/roundable/0.1.1/roundable/
[crates.io]: https://crates.io/crates/roundable
[issues]: https://github.com/danielparks/roundable/issues
[`Duration`]: https://doc.rust-lang.org/core/time/struct.Duration.html
[`Roundable`]: https://docs.rs/roundable/0.1.1/roundable/trait.Roundable.html
[`try_round_to()`]: https://docs.rs/roundable/0.1.1/roundable/trait.Roundable.html#tymethod.try_round_to
[`round_to()`]: https://docs.rs/roundable/0.1.1/roundable/trait.Roundable.html#method.round_to
[`round_up_to()`]: https://docs.rs/roundable/0.1.1/roundable/trait.Roundable.html#method.round_up_to
[`Tie::Up`]: https://docs.rs/roundable/0.1.1/roundable/enum.Tie.html#variant.Up
[`Tie::Down`]: https://docs.rs/roundable/0.1.1/roundable/enum.Tie.html#variant.Down
[`Tie::TowardZero`]: https://docs.rs/roundable/0.1.1/roundable/enum.Tie.html#variant.TowardZero
[`Tie::AwayFromZero`]: https://docs.rs/roundable/0.1.1/roundable/enum.Tie.html#variant.AwayFromZero
[`Tie::TowardEven`]: https://docs.rs/roundable/0.1.1/roundable/enum.Tie.html#variant.TowardEven
[`Tie::TowardOdd`]: https://docs.rs/roundable/0.1.1/roundable/enum.Tie.html#variant.TowardOdd
[constants]: https://docs.rs/roundable/0.1.1/roundable/#constants
