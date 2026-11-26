//! Reference validator for the Agent Dispatch Protocol (ADP).
//!
//! ADP defines the envelope an orchestrator sends to an agent (a
//! dispatch) and the envelope the agent sends back (a report). This crate
//! checks an envelope against the validation rules of the spec and says
//! which rules it breaks.
//!
//! ```
//! let envelope = serde_json::json!({
//!     "adp_version": "1.1.1",
//!     "dispatch": { "agent": "fixer", "anchor": ["CHG-4821"] },
//!     "context": { "read": [{ "path": "src/fees/interest.rs" }] },
//!     "produce": [{ "action": "create" }],
//!     "exit": ["rounding tests pass"],
//!     "report": ["diff summary"]
//! });
//! let verdict = adp::v1::validate_dispatch(&envelope);
//! assert!(!verdict.is_valid());
//! assert_eq!(verdict.violations()[0].rule(), adp::Rule::V13);
//! assert_eq!(verdict.violations()[0].pointer(), "/produce/0/path");
//! ```
//!
//! # Scope
//!
//! - Pure functions: no I/O, no network, no clock, no global state.
//! - Input is a [`serde_json::Value`]. A caller holding YAML, TOML or
//!   another serialization parses it into the JSON data model first.
//! - The reference implementation is not binding (ADP-0 §3.4). Where it
//!   disagrees with the spec text, the schemas or the conformance
//!   vectors, the crate has a defect.
//!
//! # Spec versions
//!
//! The crate has its own version line. Each release implements these
//! spec versions (ADP-A-008 C2.1):
//!
//! | Crate | ADP-1 | ADP-2 |
//! |-------|-------|-------|
//! | 0.1.0 | 1.1.1 | —     |
//!
//! [`CLAIMS`] lists the documents whose conformance vectors the crate
//! passes; [`v1::SPEC_VERSION`] gives the version.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

mod rule;
mod semver;
pub mod v1;
mod verdict;

pub use rule::Rule;
pub use verdict::{Verdict, Violation};

/// The ADP documents this crate claims (ADP-0 §3.4). It gives every
/// conformance vector of each one its expected verdict and rule.
pub const CLAIMS: &[&str] = &["ADP-1"];
