---
id: EN-110
epic: E17
feature: F21
title: The secret scan in Rust, without secretlint
status: draft
points: 3
---

# E17 · F21 · EN-110 — The secret scan in Rust, without secretlint

`secretlint` and `@secretlint/secretlint-rule-preset-recommend` scan every staged file at each commit and every tracked file in `check:all` (`lint:secrets`). Rewritten in Rust (F21.AC4, the feature's table of dev packages): each pattern of the preset carried with its source and a test; the preset's patterns read first, at 13.0.7, so that none is lost. The preset is MIT-licensed if SP-023 confirms it: a pattern ported names its package and version, and `NOTICE` and `THIRD-PARTY-LICENSES.md` receive the licence (ADR-0076).

## Acceptance criteria

- Given each rule of the recommended preset at 13.0.7, when the story ends, then the Rust scan carries each of its patterns, naming the rule and its source file, and a test per rule proves it finds the preset's own positive cases and leaves its negative ones.
- Given `.secretlintignore`'s paths, when the scan runs, then it skips the same paths, read from one ignore file that the docs name.
- Given the tracked files, when `npm run lint:secrets` runs, then it reports the same findings as `secretlint` (none on the tree; the same on a copy with one planted secret per rule), and exits 1 on a finding, naming the file, the line and the rule, never the secret.
- Given staged files, when the pre-commit hook of EN-108 runs, then it calls the Rust scan in place of `secretlint`; `secretlint`, its preset and `.secretlintrc.json` are gone.
- Given the scan, when it is timed, then its time on every tracked file and on one staged file is stated before and after.

## Files

`tools/secrets/` (new crate, or a binary of `tools/git-hooks/`), `tools/git-hooks/` (the staged-file runner), `package.json`, `package-lock.json`, `.secretlintrc.json` (removed), `.secretlintignore` (renamed or kept), `docs/tooling/versions-and-security.md`, `docs/tooling/dependencies.md`, `docs/tooling/commands.md`, `NOTICE`, `THIRD-PARTY-LICENSES.md`.

## Measures (ADR-0069)

Before and after on this Mac, 10 runs each: `lint:secrets` on every tracked file; the scan of one staged Markdown file inside a commit.

## Tasks

One task = one commit, referenced as `EN-110.Tn`.

- [ ] T1 — The preset's rules read at 13.0.7; each pattern carried with its cases and its source; the licence recorded (5 h)
- [ ] T2 — The scan over tracked and staged files, the ignore file; the packages removed; docs; times measured (3 h)
