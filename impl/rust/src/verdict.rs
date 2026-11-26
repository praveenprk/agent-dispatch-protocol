//! The result of validating a document.

use crate::Rule;

/// One rule that a document breaks, and where.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Violation {
    rule: Rule,
    pointer: String,
    message: String,
}

impl Violation {
    pub(crate) fn new(rule: Rule, pointer: String, message: impl Into<String>) -> Self {
        Self {
            rule,
            pointer,
            message: message.into(),
        }
    }

    /// The rule that is broken.
    #[must_use]
    pub fn rule(&self) -> Rule {
        self.rule
    }

    /// Where in the input, as a JSON Pointer (RFC 6901).
    ///
    /// For a missing field, the pointer names the place where the field
    /// should be. The empty string is the whole document.
    #[must_use]
    pub fn pointer(&self) -> &str {
        &self.pointer
    }

    /// A short English description of what is wrong.
    ///
    /// The wording is not part of the API and may change in any release.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }
}

/// The verdict on one document: valid, or the rules it breaks.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Verdict {
    violations: Vec<Violation>,
}

impl Verdict {
    pub(crate) fn from_violations(violations: Vec<Violation>) -> Self {
        Self { violations }
    }

    /// `true` when the document breaks no rule.
    #[must_use]
    pub fn is_valid(&self) -> bool {
        self.violations.is_empty()
    }

    /// Every rule the document breaks, in the order the checks found them.
    #[must_use]
    pub fn violations(&self) -> &[Violation] {
        &self.violations
    }
}
