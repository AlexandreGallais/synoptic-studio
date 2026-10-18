# ADR-0076 — Code ported into the tools under its licence, never into the library

**Status**: Accepted (Product Owner, 2026-10-17, at AUD-021). Amends the research protocol's rule on reference implementations (`docs/research/README.md`, section 4) for the tools only.

## Context

- The research protocol forbids a copy or a line-by-line translation of a reference implementation, and asks for a reimplementation from the paper or the specification, whatever the licence (`docs/research/README.md`, section 4).
- F19 rewrote in Rust the ESLint rules oxlint lacks (EN-075 to EN-084, EN-095), so that each finding reads as it did in ESLint: the rules, the JSDoc parser and the Markdown rules follow their packages' source, and the JSDoc rules are tested on `eslint-plugin-jsdoc`'s own test cases, copied into `tools/lint/tests/fixtures/jsdoc/`.
- The audit of F19 (AUD-021) found this code at odds with section 4, and `NOTICE` naming none of these packages, though each licence (MIT, BSD-3-Clause) asks its copyright notice to be kept with a copy or a substantial portion.
- A rule's behaviour is its specification: a linter that reports and fixes as the original does cannot be written from a paper, and its documented cases are the reference its tests check.

## Decision

1. **The tools** (`tools/`, the repository's scripts and hooks) may port code from, and copy test cases of, a package under a permissive licence (MIT, BSD, Apache-2.0, ISC), naming the package and its version in the module that ports it.
2. **Each such package** is listed in `NOTICE` with its version, its licence and its copyright lines, and its licence text, as published in its npm tarball at that version, is in `THIRD-PARTY-LICENSES.md`, in the change that ports it.
3. **The library** (`src/`) keeps section 4: no copy, no line-by-line translation; it is reimplemented from the paper or the specification, every function backed by its source or a derivation.
4. **A copyleft licence** (GPL, LGPL, AGPL) is never ported, into the tools or the library: read to understand only.
5. **A test** checks that each package a module of `tools/` names as the one it ports or follows is listed in `NOTICE` (`tests/third-party-licences.test.ts`).

## Reasons

| Option | For | Against |
| --- | --- | --- |
| Port into the tools with the notice, the library untouched (chosen) | the rules report and fix as the original; the licences' conditions met; the library's provenance unchanged | `NOTICE` and the licence texts to keep up to date |
| Reimplement the tools from the documentation only | one rule for the whole repository | the findings and fixes drift from the original; F19's rules rewritten again |
| Remove the ported rules and copied cases | no licence question | the rules the Product Owner kept are lost (F19.AC11) |

## Consequences

- `NOTICE` lists 14 packages and `THIRD-PARTY-LICENSES.md` holds their texts (AUD-021).
- `docs/research/README.md`, section 4, states the rule for the library and points here for the tools.
- A tool story that ports a new package adds its row to `NOTICE` and its text to `THIRD-PARTY-LICENSES.md`; the test fails otherwise.

## References

`docs/research/README.md` (section 4), ADR-0025, ADR-0073, ADR-0075, [AUD-021](../backlog/stories/E17-F19-AUD-021-audit-f19.md), `NOTICE`, `THIRD-PARTY-LICENSES.md`
