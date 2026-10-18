---
id: EN-110
epic: E17
feature: F21
title: The secret scan in Rust, without secretlint
status: done
points: 3
---

# E17 · F21 · EN-110 — The secret scan in Rust, without secretlint

`secretlint` and `@secretlint/secretlint-rule-preset-recommend` scan every staged file at each commit and every tracked file in `check:all` (`lint:secrets`). Rewritten in Rust (F21.AC4, the feature's table of dev packages): each pattern of the preset carried with its source and a test; the preset's patterns read first, at 13.0.7, so that none is lost. The preset is MIT-licensed, Copyright (c) 2020 azu, as SP-023 read in its installed package: a pattern ported names its package and version, and `NOTICE` and `THIRD-PARTY-LICENSES.md` receive the licence (ADR-0076).

What the preset holds at 13.0.7, read by SP-023 ([research note 0025](../../research/notes/0025-f21-agent-tools-in-rust.md), The rewritten packages; `REF-SECRETLINT`, `REF-SECRETLINT-PRESET`): 27 detecting rules and the comment filter, 41 regular expressions. Rust's `regex` 1.13.1 runs them once these are handled in code: 25 look-behinds `(?<!\p{L})` and their look-aheads, each one character before or after the match; 2 backreferences (a password of one repeated character, a dummy); JavaScript's ASCII `\b`, written `(?-u:\b)`; each rule's options and dummy values with their defaults. The GCP rule also checks a binary `.p12` file's PKCS #12 MAC against the password `notasecret`, with HMAC over SHA-1 or SHA-256: RustCrypto's crates, each checked as every crate is, or a port of the two algorithms tested against the test vectors of their standards, read in that task; decided in T1. secretlint's own behaviour to keep: exit 0, 1 or 2, secrets masked, the built-in ignores, `.secretlintignore`, each folder's `.gitignore`, symlinks not followed, the `secretlint-disable` comments.

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

Before, measured by SP-023 (0025, Each tool's time before its port), medians: `lint:secrets` on every tracked file 911 ms (5 runs); `secretlint` on one file 123 ms (20 calls).

Before and after, measured by EN-110 on 2026-10-18 back to back in this worktree, load 3.6 to 4.5, medians (a timer around each process, `perf_counter`): every tracked file, `git ls-files -z | xargs -0 secretlint --no-glob` **970 ms** (5 runs), `target/release/secret-scan` **106 ms** (5 runs), `npm run -s lint:secrets` through `cargo run` 271 ms (5 runs); one file, `secretlint --no-glob README.md` **127 ms** (10 calls), `target/release/secret-scan README.md`, as the pre-commit hook calls it, **34 ms** (10 calls). A pattern is compiled only when the text holds one of its literals (`pattern.rs`): before that, every pattern compiled at each call and one file took 70 ms. An earlier series under load 20 to 30 gave 998 and 135 ms against 135 and 34 ms.

The findings are secretlint's: on a clone of the tree with the preset's 108 snapshot inputs added and tracked, secretlint and the scan report the same 112 findings, file, line, column, message id, rule and masked message alike; on the tree itself, none.

## Tasks

One task = one commit, referenced as `EN-110.Tn`.

- [ ] T1 — The preset's rules read at 13.0.7; each pattern carried with its cases and its source; the licence recorded (5 h)
- [ ] T2 — The scan over tracked and staged files, the ignore file; the packages removed; docs; times measured (3 h)

## For the main thread

- Measures and the comparison with secretlint: above (Measures).
- `docs/tooling/versions-and-security.md`, named in Files, is not changed: it says nothing of the secret scan; `docs/tooling/ci.md`, `git-workflow.md`, `commands.md`, `agent-tools.md`, `dependencies.md` (its Security section, now empty, removed), `conventions/formatting.md` and `conventions/rules.md` are.
- ADR-0041 and ADR-0044, accepted, still name secretlint and `.secretlintignore` as they were at the time: not rewritten.
- `.secretlintignore` is renamed `.secret-scan-ignore` (same 12 paths); `.secretlintrc.json` is removed.
