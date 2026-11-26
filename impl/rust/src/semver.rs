//! SemVer 2.0.0 parsing, hand-written from the grammar at
//! <https://semver.org/spec/v2.0.0.html> (ADP-1 §9).
//!
//! Numeric identifiers are kept as digit strings, never converted to a
//! fixed-width integer: SemVer sets no upper bound, so
//! `1.18446744073709551616.0` is a valid version.

/// The parts of a valid version that ADP checks.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Version<'a> {
    pub(crate) major: &'a str,
    pub(crate) minor: &'a str,
}

/// Parses `s` as a complete SemVer 2.0.0 version string.
///
/// Returns `None` when `s` is not one, including when it has leading or
/// trailing whitespace.
pub(crate) fn parse(s: &str) -> Option<Version<'_>> {
    let (rest, build) = match s.split_once('+') {
        Some((rest, build)) => (rest, Some(build)),
        None => (s, None),
    };
    let (core, pre) = match rest.split_once('-') {
        Some((core, pre)) => (core, Some(pre)),
        None => (rest, None),
    };

    let mut parts = core.split('.');
    let major = parts.next()?;
    let minor = parts.next()?;
    let patch = parts.next()?;
    if parts.next().is_some() {
        return None;
    }
    if ![major, minor, patch].into_iter().all(is_numeric_identifier) {
        return None;
    }
    if let Some(pre) = pre {
        if !pre.split('.').all(is_pre_release_identifier) {
            return None;
        }
    }
    if let Some(build) = build {
        if !build.split('.').all(is_build_identifier) {
            return None;
        }
    }
    Some(Version { major, minor })
}

/// `<numeric identifier>`: `0`, or a positive digit followed by digits.
fn is_numeric_identifier(s: &str) -> bool {
    match s.as_bytes() {
        [] => false,
        [b'0'] => true,
        [b'0', ..] => false,
        bytes => bytes.iter().all(u8::is_ascii_digit),
    }
}

/// `<pre-release identifier>`: an alphanumeric identifier, or a numeric
/// identifier without leading zeros.
fn is_pre_release_identifier(s: &str) -> bool {
    if !s.is_empty() && s.bytes().all(|b| b.is_ascii_digit()) {
        is_numeric_identifier(s)
    } else {
        is_build_identifier(s)
    }
}

/// `<build identifier>`: one or more ASCII letters, digits or hyphens.
fn is_build_identifier(s: &str) -> bool {
    !s.is_empty() && s.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'-')
}

/// Compares two numeric identifiers by value.
///
/// Both must be valid numeric identifiers, so neither has a leading zero
/// and the longer one is the larger.
pub(crate) fn numeric_gt(a: &str, b: &str) -> bool {
    a.len() > b.len() || (a.len() == b.len() && a > b)
}

#[cfg(test)]
mod tests {
    use super::{Version, numeric_gt, parse};

    #[test]
    fn accepts_valid_versions() {
        for s in [
            "1.0.0",
            "0.0.0",
            "1.2.3",
            "10.20.30",
            "1.0.0-rc.1",
            "1.0.0-0",
            "1.0.0-alpha-1",
            "1.0.0-0a",
            "1.0.0-x.7.z.92",
            "1.0.0+build.5",
            "1.0.0+001",
            "1.0.0-rc.1+build.5",
            "1.0.0--",
            "1.0.0-a+b-c",
        ] {
            assert!(parse(s).is_some(), "{s:?} should be valid");
        }
    }

    #[test]
    fn rejects_invalid_versions() {
        for s in [
            "",
            "1",
            "1.0",
            "1.0.0.0",
            "01.0.0",
            "1.00.0",
            "1.0.00",
            "1.0.0-01",
            "1.0.0-",
            "1.0.0+",
            "1.0.0-rc..1",
            "1.0.0+build..5",
            "1.0.0-rc_1",
            "1.0.0\n",
            " 1.0.0",
            "v1.0.0",
            "1.0.0+a+b",
            "1.a.0",
            "1.-1.0",
            "١.0.0",
        ] {
            assert!(parse(s).is_none(), "{s:?} should be invalid");
        }
    }

    #[test]
    fn numbers_have_no_upper_bound() {
        assert_eq!(
            parse("1.18446744073709551616.0"),
            Some(Version {
                major: "1",
                minor: "18446744073709551616"
            })
        );
        assert!(parse("1.0.0-18446744073709551616").is_some());
    }

    #[test]
    fn compares_numeric_identifiers_by_value() {
        assert!(numeric_gt("2", "1"));
        assert!(numeric_gt("10", "9"));
        assert!(numeric_gt("18446744073709551616", "1"));
        assert!(!numeric_gt("1", "1"));
        assert!(!numeric_gt("0", "1"));
        assert!(!numeric_gt("9", "10"));
    }
}
