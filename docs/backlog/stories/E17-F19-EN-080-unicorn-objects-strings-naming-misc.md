---
id: EN-080
epic: E17
feature: F19
title: Unicorn's objects, strings, naming and miscellaneous rules in Rust
status: done
points: 8
---

# E17 · F19 · EN-080 — Unicorn's objects, strings, naming and miscellaneous rules in Rust

Rewrite in the project's Rust linter the 37 kept rules of `eslint-plugin-unicorn` on objects, strings, naming and the miscellaneous group, which oxlint runs only as JS plugins (Product Owner, 2026-10-17, fourth round). Sources: [research note 0023](../../research/notes/0023-f19-oxc-and-rust-tools.md), section "Triage of the rules oxlint lacks"; `REF-OXLINT-JS-PLUGINS`; each rule's own documentation in the installed plugin, read before it is rewritten.

## Acceptance criteria

- Given each of the 37 rules listed below, when the project's Rust linter runs on the cases its plugin's documentation gives, then it reports the same cases as invalid and lets the same cases through, each case a test of its crate (F19.AC11).
- Given the tree, when `npm run lint` runs, then the rewritten rules report nothing, and the findings they give on a copy with one planted case each are those ESLint gives (F19.AC11).
- Given ESLint's configuration, when the story ends, then these rules are off in it, so that each rule runs in one linter only, and the triage of note 0023 records each one as rewritten here (F19.AC11).

## Tasks

One task = one commit, referenced as `EN-080.Tn`.

- [ ] T1 — Objects (12): `consistent-destructuring`, `no-computed-property-existence-check`, `no-invalid-property-descriptor`, `no-invalid-well-known-symbol-methods`, `no-unreadable-new-expression`, `no-unreadable-object-destructuring`, `no-unsafe-property-key`, `no-unused-builtin-method-return`, `no-unused-properties`, `prefer-object-define-properties`, `prefer-object-destructuring-defaults`, `prefer-single-object-destructuring`, each with its documented cases as tests (3 h)
- [ ] T2 — Strings (14): `no-incorrect-template-string-interpolation`, `no-unnecessary-string-trim`, `no-unsafe-string-replacement`, `prefer-escaped-irregular-whitespace`, `prefer-literal-ascii`, `prefer-regexp-escape`, `prefer-short-escape-sequences`, `prefer-single-replace`, `prefer-split-limit`, `prefer-string-match-all`, `prefer-string-pad-start-end`, `prefer-string-repeat`, `prefer-unicode-code-point-escapes`, `string-content`, each with its documented cases as tests (3 h)
- [ ] T3 — Naming and miscellaneous (11): `comment-content`, `consistent-compound-words`, `consistent-tuple-labels`, `name-replacements`, `no-keyword-prefix`, `prefer-type-literal-last`, `single-line-block-comment-style`, `consistent-optional-chaining`, `no-useless-compound-assignment`, `no-useless-delete-check`, `prefer-https`, each with its documented cases as tests (3 h)
- [ ] T4 — The options of `string-content`, `name-replacements` and `comment-content` carried from `eslint/rules/unicorn/`, each with a test (2 h)
- [ ] T5 — The rules turned off in ESLint, the findings compared on the tree and on planted cases, the triage updated (1 h)

## For the main thread

- Rules: 33 of the 37 run in the project's linter (`tools/lint/src/rules/unicorn/objects/`, `strings/`, `naming/`, `misc/`, as EN-079 and EN-081 group theirs; one module each with the cases of its documentation in `eslint-plugin-unicorn` 78.0.0); dropped with their reasons in note 0023 and `DROPPED_RULES`: `prefer-escaped-irregular-whitespace` (`no-irregular-whitespace`, native, already refuses such characters in strings, templates and regular expressions), `prefer-string-match-all` (an `exec()` loop needs a `let` assigned in a `while` condition: `no-cond-assign` and `functional/no-let` refuse both), `single-line-block-comment-style` (all it reports, `@stylistic/multiline-comment-style` already refuses), `string-content` (no pattern configured: it reports nothing). Each drop was tried on a probe file. None runs natively in oxlint. The 33 are `off` in ESLint.
- Compared: ESLint and the Rust linter give the same 610 findings and the same fixed text on the documented cases and on 900 copies of the tree's files with their terms lower-cased, names abbreviated and links turned to `http:`; none on the tree. Autofixed as in ESLint: 17 rules; reported only where ESLint only suggests.
- Decided by the main thread on 2026-10-17: the option `comment-content: {replacements: {application: false}}` never worked — the plugin's key is the pattern `\bapplication\b(?!/)`, so ESLint kept turning `application` into `app` in comments. Its intent, keeping `application`, is carried: the Rust linter's option names the plugin's pattern, which ESLint, given the same key, also honours (checked on a probe: `application` no longer reported, `applications` still becomes `apps`, `application/json` untouched). No comment of the tree uses the word today.
- Shared code: `finding::apply` now applies fixes in ESLint's order (from the first range, a touching or overlapping fix waiting for the next pass), so three adjacent destructurings merge as ESLint merges them; the indentation helpers of `iteration-fallback-style` moved to `unicorn/layout.rs`; new shared modules `known_values`, `escapes`, `identifiers` and `pattern` (a backtracking matcher for the plugin's regular expressions, used by `comment-content` and `prefer-https`). Rebased on EN-079 and EN-081: `identifiers` reuses `modules::globals` (EN-081) for `ES_GLOBALS` and `is_identifier_name`. `crate::expressions` (EN-079) and `unicorn::syntax` (EN-078) overlap (text, comments, parentheses, booleans): one for the naming pass.
- Timings: the Rust linter's run on `npm run lint`'s 2,053 targets, with every Unicorn rule so far, takes 0.22 s (1.8 s of processor time, 0.75 s of it `comment-content`); ESLint spent 1,071 ms on the 37 rules (`TIMING=all`, `comment-content` 567 ms), its run from 14.2 s to 13.4 s.
- Rule-docs: none affected, the story changes tooling only, no rule of `docs/domain/`.

### Journal

- `#run` EN-080: Unicorn's objects, strings, naming and miscellaneous rules in the project's linter, 33 rewritten in Rust from their documented cases, four dropped as guarding nothing here; the same 610 findings and fixes as ESLint on the cases and on 900 altered copies of the tree, none on the tree; fixes now land in ESLint's order; the `comment-content` option of the project found to drop nothing, then keyed by the plugin's pattern so that it keeps `application`.
