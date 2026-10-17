---
id: EN-081
epic: E17
feature: F19
title: Unicorn's modules, platform and promises rules in Rust
status: done
points: 5
---

# E17 · F19 · EN-081 — Unicorn's modules, platform and promises rules in Rust

Rewrite in the project's Rust linter the 29 kept rules of `eslint-plugin-unicorn` on modules, the platform and promises, which oxlint runs only as JS plugins (Product Owner, 2026-10-17, fourth round). Sources: [research note 0023](../../research/notes/0023-f19-oxc-and-rust-tools.md), section "Triage of the rules oxlint lacks"; `REF-OXLINT-JS-PLUGINS`; each rule's own documentation in the installed plugin, read before it is rewritten.

## Acceptance criteria

- Given each of the 29 rules listed below, when the project's Rust linter runs on the cases its plugin's documentation gives, then it reports the same cases as invalid and lets the same cases through, each case a test of its crate (F19.AC11).
- Given the tree, when `npm run lint` runs, then the rewritten rules report nothing, and the findings they give on a copy with one planted case each are those ESLint gives (F19.AC11).
- Given ESLint's configuration, when the story ends, then these rules are off in it, so that each rule runs in one linter only, and the triage of note 0023 records each one as rewritten here (F19.AC11).

## Tasks

One task = one commit, referenced as `EN-081.Tn`.

- [ ] T1 — Modules (8): `default-export-style`, `no-global-object-property-assignment`, `no-top-level-side-effects`, `no-unnecessary-global-this`, `no-useless-re-export`, `prefer-global-number-constants`, `prefer-identifier-import-export-specifiers`, `prefer-json-import`, each with its documented cases as tests (3 h)
- [ ] T2 — Platform (11): `consistent-json-file-read`, `no-invalid-intl-options`, `no-invalid-temporal-arithmetic`, `no-unsafe-buffer-conversion`, `no-unsafe-json-serialization`, `no-using-resource-escape`, `prefer-dispose`, `prefer-temporal`, `prefer-temporal-conversion`, `prefer-uint8array-base64`, `prefer-uint8array-hex`, each with its documented cases as tests (3 h)
- [ ] T3 — Promises (10): `no-async-iterator-callback`, `no-async-promise-finally`, `no-multiple-promise-resolver-calls`, `prefer-array-from-async`, `prefer-await`, `prefer-promise-static-methods`, `prefer-promise-try`, `prefer-promise-with-resolvers`, `prefer-queue-microtask`, `prefer-then-catch`, each with its documented cases as tests (3 h)
- [ ] T4 — The rules turned off in ESLint, the findings compared on the tree and on planted cases, the triage updated (1 h)

## Timings (ADR-0069)

Measured on this Mac, 2026-10-17, on `npm run lint`'s targets (2,048 files), another worktree running beside:

| Tool | Before | After |
| --- | --- | --- |
| Rust linter (`tools/lint`) | 0.12–0.13 s (EN-079) | 0.14–0.19 s, 16 rules more |
| ESLint | 12.9–13.2 s, the 29 rules taking 321 ms (`TIMING=all`) | 12.5–15.3 s, 29 rules fewer |

## For the main thread

- Rules: 16 of the 29 run in the project's linter, in `tools/lint/src/rules/unicorn/modules/` (7), `platform/` (2) and `promises/` (7), each on the cases of its documentation in `eslint-plugin-unicorn` 78.0.0; 13 are dropped with their reasons in note 0023 and `DROPPED_RULES`: `default-export-style`, `consistent-json-file-read`, `no-invalid-intl-options`, `no-invalid-temporal-arithmetic`, `prefer-temporal`, `prefer-temporal-conversion`, `no-using-resource-escape`, `prefer-dispose`, `prefer-array-from-async`, `prefer-uint8array-base64`, `prefer-uint8array-hex`, `prefer-promise-with-resolvers`, `prefer-queue-microtask`. None goes to oxlint: version 1.87.0 runs none of the 29 natively.
- Compared: on the tree, no finding from either linter; on the documented and planted cases, the same findings and the same fixed text, but for a string specifier ESLint reports twice at one place, a destructured `Number.NaN` whose fix by the plugin drops the local name (reported here without a fix), and a documented snippet Oxc refuses to parse.
- `no-top-level-side-effects` leaves by itself the files of the `tooling` and `tests` scopes and `playground/index.ts`, where ESLint turned it off, since Unicorn's rules run on every file (`scopes::linted_by`).
- Open point: `expressions::has_side_effect` (EN-079) walks into the functions an expression holds, which `hasSideEffect` of `@eslint-community/eslint-utils` does not; `no-top-level-side-effects` uses its own search for that reason, and the rules of EN-079 that call it were left unchanged.
- Shared lists edited beside EN-080: `rules::unicorn::RULES` and `NAMES`, the `pub mod` lines of `rules/unicorn/mod.rs`, `DROPPED_RULES`, and note 0023's counts (Unicorn 157 kept and 40 dropped, all 208 and 57, before EN-080's own).
- Rule-docs: none affected, the story moves lint rules between linters and changes no rule of `docs/domain/`.

### Journal

- `#run` EN-081: 16 Unicorn rules on modules, the platform and promises carried to the Rust linter, 13 dropped (Node.js, `Temporal`, `using` and APIs `tsc` refuses under ES2025); the same findings and fixes as ESLint on the documented and planted cases, none on the tree; the resolver rule follows each function's paths where ESLint asks its code paths.
