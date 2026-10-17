---
id: EN-074
epic: E17
feature: F19
title: oxlint with its native and type-aware rules
status: ready
points: 5
---

# E17 · F19 · EN-074 — oxlint with its native and type-aware rules

Run oxlint as the one lint command, with its native rules and `tsgolint`'s 55 type-aware rules, and confirm against the code the triage of the 265 rules it runs only as JS plugins. Sources: [research note 0023](../../research/notes/0023-f19-oxc-and-rust-tools.md), sections "The ESLint rules on oxlint", "The 8 rules that need types", "The Product Owner's third round" and "Triage of the rules oxlint lacks"; `REF-OXLINT`, `REF-OXLINT-PLUGINS`, `REF-OXLINT-MIGRATE`, `REF-OXLINT-TYPE-AWARE`, `REF-TSGOLINT`, `REF-OXLINT-PREFER-DESTRUCTURING`, `REF-TSESLINT-PREFER-DESTRUCTURING`, `REF-TSESLINT-NO-UNSAFE-ENUM-ASSIGNMENT`, `REF-OXLINT-ACCUMULATING-SPREAD`.

## Acceptance criteria

- Given `.oxlintrc.json` written by `@oxlint/migrate --type-aware` from `eslint/`, with oxlint 1.87.0 or later and `oxlint-tsgolint` 7.0.2004 or later (each at least 7 days old), when `npm run lint` runs on the tree, then oxlint runs the 432 native rules with their options and the 55 type-aware rules, and reports nothing (F19.AC1).
- Given the scopes of `eslint/scopes/`, when they are carried, then each is an `overrides` entry of `.oxlintrc.json`, and a file of each scope gets the rules it got from ESLint (F19.AC1).
- Given the 110 findings oxlint gives that ESLint did not (8 rules, from other defaults), when the story ends, then each rule has an option or a decision written in the story, and the tree reports none (F19.AC1).
- Given `@typescript-eslint/prefer-destructuring`, when it is carried, then the native `eslint/prefer-destructuring` runs with the same options; `no-unsafe-enum-assignment` is dropped, its reason (`enum` is forbidden) written for the ADR (F19.AC2).
- Given the triage of note 0023 (232 kept, 33 dropped), when each rule is checked against `src/`, `playground/`, `tests/` and the root configuration files, then each verdict is confirmed or changed with its reason, and the rewrite stories EN-078 to EN-084 hold the kept rules (F19.AC11).
- Given ESLint, when the story ends, then every rule now run by oxlint is off in ESLint's configuration, so that each rule runs in one linter only, and the ESLint configuration's tests (`eslint/config.test.ts`) that check those rules move to `.oxlintrc.json` (F19.AC1).

## Tasks

One task = one commit, referenced as `EN-074.Tn`.

- [ ] T1 — oxlint and `tsgolint` installed, `.oxlintrc.json` from the migration, their rows in `docs/tooling/dependencies.md` (2 h)
- [ ] T2 — The scopes as `overrides`, each checked on a file of its scope (3 h)
- [ ] T3 — The 110 findings decided rule by rule; `prefer-destructuring` native; `no-unsafe-enum-assignment` dropped (3 h)
- [ ] T4 — The triage of the 265 rules confirmed against the code, note 0023 updated (3 h)
- [ ] T5 — The carried rules off in ESLint, the configuration's tests moved, `npm run lint` running oxlint first (2 h)
