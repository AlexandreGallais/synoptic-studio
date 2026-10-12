# ADR-0020 — Every feature starts with a research spike and ends with an audit

**Status**: Accepted

## Context

The Product Owner wants assurance that, at the end of the project, everything makes sense: mathematics correct, every source real and still valid, no copied code, no duplicate, documentation consistent. Research done story by story discovers missing or unverifiable sources late (EN-004: `REF-GG-FILLET` could not be read and a derivation was written mid-story).

## Decision

Every feature follows the same frame, enforced by the backlog test:

1. **Research spike first** (`SP`): before any implementation story, documentary research for **all** the feature's stories — each formula, algorithm and behavior gets a verified source, a derivation, or a research request. Its output: rows in `docs/references.md`, derivations, a research note, and stories updated (criteria, `@see` targets, blocked points).
2. **Implementation stories** (`US`, `EN`), as before.
3. **Audit** (`AUD`): a full check of the feature **in depth** and of the whole project **in breadth**, following the checklist of [audit.md](../conventions/audit.md): mathematics recomputed, sources re-verified, provenance of the code, duplicates, consistency of code, domain, ADRs and backlog. Findings are fixed in the audit or become stories.
4. **Validation** (`VAL`): the Product Owner validates the feature's acceptance criteria.

Regression and end-to-end tests are **not** part of this frame yet: they come when the applications exist (Product Owner, 2026-10-08).

## Consequences

- Two more stories per feature; a feature cannot be closed without its audit.
- Sources are found before coding, so a story is rarely blocked mid-way.
- F01, refined before this decision, receives its spike (for its remaining stories) and its audit now.

## References

ADR-0017, [audit.md](../conventions/audit.md), [research protocol](../research/)
