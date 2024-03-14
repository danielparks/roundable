//! # Minimum supported Rust version
//!
//! Currently the minimum supported Rust version (MSRV) is **1.60**. Future
//! increases in the MSRV will require a major version bump.

// Most lint configuration is in lints.toml, but that isn’t supported by
// cargo-geiger, and it only supports deny, not forbid.
#![forbid(unsafe_code)]

/// Encrypt single byte with secure ROT13 function
///
/// ~~~
/// use roundable::rot13_u8;
/// assert_eq!(rot13_u8(b'a'), b'n')
/// ~~~
#[must_use]
pub const fn rot13_u8(c: u8) -> u8 {
    // The formulas below will never over or underflow.
    #![allow(clippy::arithmetic_side_effects)]
    if c.is_ascii_lowercase() {
        ((c - b'a') + 13) % 26 + b'a'
    } else if c.is_ascii_uppercase() {
        ((c - b'A') + 13) % 26 + b'A'
    } else {
        c
    }
}

/// Encrypt string with secure ROT13 function
///
/// ~~~
/// use roundable::rot13;
/// assert_eq!(rot13("super secure"), "fhcre frpher")
/// ~~~
///
/// # Panics
///
/// This could panic if it can’t allocate memory.
///
/// Strictly speaking, it could panic when converting back to UTF-8, but that
/// won’t happen because ROT13 only operates on ASCII bytes.
#[must_use]
pub fn rot13(source: &str) -> String {
    let mut buffer: Vec<u8> = Vec::with_capacity(source.len());
    for c in source.bytes() {
        buffer.push(rot13_u8(c));
    }
    String::from_utf8(buffer).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test {
        ($name:ident, $($test:tt)+) => {
            #[test]
            fn $name() {
                ::assert2::assert!($($test)+);
            }
        };
    }

    test!(byte_tilde, rot13_u8(b'~') == b'~');
    test!(byte_lower_a, rot13_u8(b'a') == b'a' + 13);
    test!(byte_upper_a, rot13_u8(b'A') == b'A' + 13);
    test!(byte_lower_z, rot13_u8(b'z') == b'a' + 12);
    test!(byte_upper_z, rot13_u8(b'Z') == b'A' + 12);
    test!(str_abc, rot13(".abc NOP") == ".nop ABC");
}
