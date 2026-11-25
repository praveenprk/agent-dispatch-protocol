"""check_defects — DEFECTS.md is append-only (ADP-0 §4.1, §4.2, §4.5).

Compares DEFECTS.md at BASE with the working tree:
- no entry removed, no ID duplicated, new IDs continue the sequence
- after filing, only `Status`, `Resolved by` and `Notes` may change
- every entry has the required fields, with a valid Severity and Status

Usage: python3 scripts/ci/check_defects.py BASE [FILE]   (from the repo root;
       FILE defaults to DEFECTS.md, the version compared against BASE)
Needs: git. Exit: 0 pass, 1 any violation, 2 usage. Quiet on success.
"""
import re
import subprocess
import sys
from pathlib import Path

REQUIRED = ("Artifact", "Says", "Does", "Evidence", "Severity", "Status", "Resolved by")
MUTABLE = {"Status", "Resolved by", "Notes"}
SEVERITY = {"CRITICAL", "HIGH", "MEDIUM", "LOW"}
STATUS = {"OPEN", "RESOLVED", "WONTFIX", "SUPERSEDED"}

HEADING = re.compile(r"^### (ADP-D-(\d{3}))\s*$")
FIELD = re.compile(r"^([A-Z][A-Za-z ]*?)\s*:\s?(.*)$")


def parse(text):
    """Return ([ids in order], {id: {field: normalised value}})."""
    order, entries = [], {}
    cur = field = None
    for line in text.splitlines():
        m = HEADING.match(line)
        if m:
            cur, field = m.group(1), None
            order.append(cur)
            entries.setdefault(cur, {})
            continue
        if cur is None:
            continue
        if line.startswith("#") or line.startswith("---"):
            cur = field = None
            continue
        if not line.strip():
            continue
        if line[0].isspace():
            if field:
                entries[cur][field] += " " + line.strip()
            continue
        f = FIELD.match(line)
        if f:
            field = f.group(1).strip()
            entries[cur][field] = f.group(2).strip()
    return order, entries


def main():
    if len(sys.argv) not in (2, 3):
        print("usage: check_defects.py BASE [FILE]", file=sys.stderr)
        return 2
    base = sys.argv[1]
    errors = []

    # Fail closed: a BASE git cannot resolve (including the all-zero SHA a
    # branch-creating push gives) would otherwise read as an empty register
    # and skip the append-only checks. A BASE that resolves but has no
    # DEFECTS.md is a real empty register.
    resolved = subprocess.run(["git", "rev-parse", "--verify", "--quiet", f"{base}^{{commit}}"],
                              capture_output=True, text=True)
    if resolved.returncode != 0 or set(base) == {"0"}:
        print(f"check_defects.py: cannot resolve BASE {base!r} to a commit", file=sys.stderr)
        return 2

    old = subprocess.run(["git", "show", f"{base}:DEFECTS.md"],
                         capture_output=True, text=True)
    old_order, old_entries = parse(old.stdout) if old.returncode == 0 else ([], {})
    new_order, new_entries = parse(Path(sys.argv[2] if len(sys.argv) == 3 else "DEFECTS.md").read_text(encoding="utf-8"))

    dupes = sorted({i for i in new_order if new_order.count(i) > 1})
    if dupes:
        errors.append(f"duplicate IDs: {', '.join(dupes)}")

    expected = [f"ADP-D-{n:03d}" for n in range(1, len(set(new_order)) + 1)]
    if sorted(set(new_order)) != expected:
        errors.append("IDs are not a gapless sequence from ADP-D-001 "
                      f"(found {', '.join(sorted(set(new_order)))})")

    for i in old_order:
        if i not in new_entries:
            errors.append(f"{i}: entry removed (ADP-0 §4.5: never deleted)")
            continue
        for k in sorted(set(old_entries[i]) | set(new_entries[i])):
            if k in MUTABLE:
                continue
            if old_entries[i].get(k) != new_entries[i].get(k):
                errors.append(f"{i}: field '{k}' changed after filing (ADP-0 §4.5)")

    for i in new_order:
        e = new_entries[i]
        for k in REQUIRED:
            if k not in e:
                errors.append(f"{i}: missing field '{k}' (ADP-0 §4.2)")
        if "Severity" in e and e["Severity"] not in SEVERITY:
            errors.append(f"{i}: Severity '{e['Severity']}' not in {sorted(SEVERITY)}")
        if "Status" in e and e["Status"] not in STATUS:
            errors.append(f"{i}: Status '{e['Status']}' not in {sorted(STATUS)}")

    for err in errors:
        print(f"DEFECTS.md: {err}", file=sys.stderr)
    return 1 if errors else 0


if __name__ == "__main__":
    sys.exit(main())
