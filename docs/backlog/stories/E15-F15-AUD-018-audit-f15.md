---
id: AUD-018
epic: E15
feature: F15
title: Audit F15
status: ready
points: 2
---

# E15 · F15 · AUD-018 — Audit F15

The feature in depth, the project in breadth, following [audit.md](../../conventions/audit.md) (ADR-0020), with the `/audit` skill.

## Acceptance criteria

- Given the functions of F15, when audited, then every step of `DERIV-font-embedding` and `DERIV-html-layer` is re-derived, every expected value recomputed by hand, the base 64 encoder checked against RFC 4648's vectors and a decoder, and each function survives a mutation spot-check.
- Given the sources, when audited, then every `@see` of F15 is re-verified and dated, and the readers' behaviors read in code (Blink, Gecko, WebKit, resvg, Batik, Inkscape) are read again at their current version.
- Given the project, when audited, then no duplicate, no copied code, no unnamed calculation (ADR-0049), no SVG element outside the places ADR-0053 gives, and no inconsistency between code, domain, ADRs and docs remains unrecorded.

## Tasks

One task = one commit, referenced as `AUD-018.Tn`.

- [ ] T1 — The audit by independent reviewers and its findings (2 h)
