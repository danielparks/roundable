# Round numbers and durations to a given factor

[![docs.rs](https://img.shields.io/docsrs/roundable)][docs.rs]
[![Crates.io](https://img.shields.io/crates/v/roundable)][crates.io]
![Rust version 1.56.1+](https://img.shields.io/badge/Rust%20version-1.56.1%2B-success)

This provides an implementation of rounding for various values, including
[`std::time::Duration`][`Duration`].

```rust
use roundable::Roundable;

assert!(310 == 314.round_to(10));
assert!(300.0 == 314.1.round_to(100.0));
```

See [Constants][] for a list of time units that make rounding [`Duration`][]
easier.

```rust
use roundable::{SECOND, MINUTE, Roundable};
use std::time::Duration;

assert!(Duration::ZERO == Duration::from_millis(314).round_to(SECOND));
assert!(MINUTE == Duration::from_millis(59_500).round_to(SECOND));
```

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

[docs.rs]: https://docs.rs/roundable/latest/roundable/
[crates.io]: https://crates.io/crates/roundable
[issues]: https://github.com/danielparks/roundable/issues
[`Duration`]: https://doc.rust-lang.org/std/time/struct.Duration.html
[Constants]: https://docs.rs/roundable/latest/roundable/#Constants
