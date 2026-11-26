//! Runs every conformance vector of every document in `adp::CLAIMS`
//! (ADP-0 §3.4, §5.6).
//!
//! Per vector: the verdict equals `expect`, and on a `fail` vector the set
//! of reported rules equals the vector's `rules`. A `known_defect` marker
//! is handled as ADP-0 §5.6.4 says.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

use serde_json::Value;

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

/// Every `conformance/<doc>/<rule>/<file>.json`, sorted.
fn vector_files(root: &Path) -> Vec<PathBuf> {
    let mut files = Vec::new();
    let conformance = root.join("conformance");
    for doc in read_dir(&conformance) {
        if !doc.is_dir() {
            continue;
        }
        for rule in read_dir(&doc) {
            if !rule.is_dir() {
                continue;
            }
            for file in read_dir(&rule) {
                if file.extension().is_some_and(|e| e == "json") {
                    files.push(file);
                }
            }
        }
    }
    files.sort();
    files
}

fn read_dir(dir: &Path) -> Vec<PathBuf> {
    fs::read_dir(dir)
        .unwrap_or_else(|e| panic!("cannot read {}: {e}", dir.display()))
        .map(|entry| entry.expect("directory entry").path())
        .collect()
}

/// `ADP-D-NNN` → status, from `DEFECTS.md` (ADP-0 §4.2).
fn defect_status(root: &Path) -> BTreeMap<String, String> {
    let text = fs::read_to_string(root.join("DEFECTS.md")).expect("read DEFECTS.md");
    let mut status = BTreeMap::new();
    let mut current: Option<String> = None;
    for line in text.lines() {
        if let Some(id) = line.strip_prefix("### ") {
            let id = id.trim();
            current = (id.starts_with("ADP-D-")).then(|| id.to_owned());
        } else if line.starts_with("Status") {
            if let (Some(id), Some((_, value))) = (current.take(), line.split_once(':')) {
                status.insert(id, value.trim().to_owned());
            }
        }
    }
    status
}

/// What the crate says about one input: `"pass"` or `"fail"`, and the
/// set of rules it reports.
fn run(kind: &str, input: &Value) -> Option<(&'static str, BTreeSet<String>)> {
    let verdict = match kind {
        "dispatch" => adp::v1::validate_dispatch(input),
        "report" => adp::v1::validate_report(input),
        _ => return None,
    };
    let rules = verdict
        .violations()
        .iter()
        .map(|v| v.rule().to_string())
        .collect();
    Some((if verdict.is_valid() { "pass" } else { "fail" }, rules))
}

#[test]
fn conformance_vectors() {
    let root = repo_root();
    let defects = defect_status(&root);
    let mut failures = Vec::new();
    let mut ran: BTreeMap<&str, usize> = adp::CLAIMS.iter().map(|d| (*d, 0)).collect();

    for path in vector_files(&root) {
        let name = path
            .strip_prefix(&root)
            .unwrap_or(&path)
            .display()
            .to_string();
        let text = fs::read_to_string(&path).unwrap_or_else(|e| panic!("{name}: {e}"));
        let vector: Value = serde_json::from_str(&text).unwrap_or_else(|e| panic!("{name}: {e}"));

        let doc = vector["target"]["doc"].as_str().unwrap_or_default();
        let Some(count) = ran.get_mut(doc) else {
            continue; // not claimed
        };
        *count += 1;

        let kind = vector["target"]["kind"].as_str().unwrap_or_default();
        let expect = vector["expect"].as_str().unwrap_or_default();
        let rules: BTreeSet<String> = vector["rules"]
            .as_array()
            .map(|a| {
                a.iter()
                    .filter_map(Value::as_str)
                    .map(str::to_owned)
                    .collect()
            })
            .unwrap_or_default();

        let Some((got, reported)) = run(kind, &vector["input"]) else {
            failures.push(format!(
                "{name}: {doc} is claimed, but the crate does not handle kind {kind:?}"
            ));
            continue;
        };
        let agrees = got == expect && (expect == "pass" || reported == rules);

        match vector.get("known_defect").and_then(Value::as_str) {
            None if agrees => {}
            None => failures.push(format!(
                "{name}: expected {expect} {rules:?}, crate says {got} {reported:?}"
            )),
            Some(id) => match defects.get(id).map(String::as_str) {
                None => failures.push(format!("{name}: known_defect {id} is not in DEFECTS.md")),
                Some("OPEN") if agrees => failures.push(format!(
                    "{name}: known_defect {id} is stale; the crate now agrees (ADP-0 §5.6.4)"
                )),
                Some("OPEN") => {}
                Some(status) => failures.push(format!(
                    "{name}: known_defect {id} is {status}; remove the marker (ADP-0 §5.6.4)"
                )),
            },
        }
    }

    for (doc, count) in &ran {
        if *count == 0 {
            failures.push(format!("{doc} is claimed, but no vector targets it"));
        }
        println!("{doc}: {count} vectors");
    }
    assert!(
        failures.is_empty(),
        "{} failure(s):\n{}",
        failures.len(),
        failures.join("\n")
    );
}
