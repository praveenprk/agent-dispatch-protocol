//! Rule identifiers from the validation tables of the ADP documents.

use core::fmt;

/// A validation rule, named by its id in the spec.
///
/// `Display` gives the spec id, e.g. `V-13` or `R-01`.
///
/// New rules are added as new variants, so the enum is
/// `#[non_exhaustive]`: a `match` on it needs a wildcard arm.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Rule {
    /// ADP-1 §7.1 V-01: `adp_version` present and valid SemVer 2.0.0.
    V01,
    /// ADP-1 §7.1 V-02: `dispatch.agent` non-empty string.
    V02,
    /// ADP-1 §7.1 V-03: `dispatch.anchor` non-empty array.
    V03,
    /// ADP-1 §7.1 V-04: `context.read` non-empty array.
    V04,
    /// ADP-1 §7.1 V-05: `exit` non-empty array.
    V05,
    /// ADP-1 §7.1 V-06: `report` non-empty array.
    V06,
    /// ADP-1 §7.1 V-07: at least one of `produce` or `work_items` present.
    V07,
    /// ADP-1 §7.1 V-08: every `work_items[].keyword` is one of the 10 keywords.
    V08,
    /// ADP-1 §7.1 V-09: no unknown field in any §4 object (§7.3).
    V09,
    /// ADP-1 §7.1 V-10: `dispatch.tier`, if present, is 1, 2 or 3.
    V10,
    /// ADP-1 §7.1 V-11: `adp_version` major version is 1 (§7.3).
    V11,
    /// ADP-1 §7.1 V-12: every `metadata` key has the §3.5 form.
    V12,
    /// ADP-1 §7.1 V-13: field types and entry fields per §4.5.
    V13,
    /// ADP-1 §7.2 R-01: `adp_version` present and valid SemVer 2.0.0.
    R01,
    /// ADP-1 §7.2 R-02: `report.agent` non-empty string.
    R02,
    /// ADP-1 §7.2 R-03: `report.anchor` non-empty array.
    R03,
    /// ADP-1 §7.2 R-04: `report.status` is `COMPLETE`, `BLOCKED` or `SURFACE`.
    R04,
    /// ADP-1 §7.2 R-05: `exit_results` non-empty array.
    R05,
    /// ADP-1 §7.2 R-06: `items` non-empty array.
    R06,
    /// ADP-1 §7.2 R-07: if status is `BLOCKED`, `items` contains key `blocker`.
    R07,
    /// ADP-1 §7.2 R-08: if status is `SURFACE`, `items` contains key `finding`.
    R08,
    /// ADP-1 §7.2 R-09: `adp_version` major version is 1 (§7.3).
    R09,
    /// ADP-1 §7.2 R-10: no unknown field in any §5 object (§7.3).
    R10,
    /// ADP-1 §7.2 R-11: every `metadata` key has the §3.5 form.
    R11,
    /// ADP-1 §7.2 R-12: field types and entry fields per §5.5.
    R12,
    /// ADP-1 §7.2 R-13: if status is `COMPLETE`, every `exit_results[].result` is true.
    R13,
}

impl Rule {
    /// The rule's id as the spec writes it, e.g. `"V-13"`.
    #[must_use]
    pub const fn id(self) -> &'static str {
        match self {
            Self::V01 => "V-01",
            Self::V02 => "V-02",
            Self::V03 => "V-03",
            Self::V04 => "V-04",
            Self::V05 => "V-05",
            Self::V06 => "V-06",
            Self::V07 => "V-07",
            Self::V08 => "V-08",
            Self::V09 => "V-09",
            Self::V10 => "V-10",
            Self::V11 => "V-11",
            Self::V12 => "V-12",
            Self::V13 => "V-13",
            Self::R01 => "R-01",
            Self::R02 => "R-02",
            Self::R03 => "R-03",
            Self::R04 => "R-04",
            Self::R05 => "R-05",
            Self::R06 => "R-06",
            Self::R07 => "R-07",
            Self::R08 => "R-08",
            Self::R09 => "R-09",
            Self::R10 => "R-10",
            Self::R11 => "R-11",
            Self::R12 => "R-12",
            Self::R13 => "R-13",
        }
    }
}

impl fmt::Display for Rule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.id())
    }
}

#[cfg(test)]
mod tests {
    use super::Rule;

    #[test]
    fn display_is_the_spec_id() {
        assert_eq!(Rule::V01.to_string(), "V-01");
        assert_eq!(Rule::V13.to_string(), "V-13");
        assert_eq!(Rule::R13.to_string(), "R-13");
    }
}
