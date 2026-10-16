---
id: SP-021
epic: E17
feature: F19
title: Research for F19, Oxc and Rust tools
status: ready
points: 5
---

# E17 · F19 · SP-021 — Research for F19, Oxc and Rust tools

Find, read and record the sources of F19 before any of its stories is written (ADR-0020): VoidZero's toolchain, Oxc, and the Rust tooling. The site voidzero.dev and Oxc's documentation are read first. Research notes gathered on 2026-10-16 (a study of how to make AI agents cheaper, the facts and research of RET-003, the Product Owner's input for this epic) are registered here as sources, each one read and checked before it is cited.

## Acceptance criteria

- Given voidzero.dev and Oxc's documentation, when the spike ends, then each tool used (oxlint, oxfmt, the transformer, the resolver, the minifier) is a `REF-*` row with `[verified]` and its date.
- Given the gathered research notes, when the spike ends, then each claim kept is tied to a `REF-*` source read, or dropped; the notes are summarized in a research note of `docs/research/notes/`.
- Given the open questions below, when the spike ends, then each is answered with a source and a measure, or turned into a research request.
- Given the answers, when the spike ends, then the target of `check:all` is proposed to the Product Owner and F19's acceptance criteria and stories are drafted.

## Questions

- Which of the project's ESLint rules does oxlint carry as they are — the custom rules of `eslint/`, the functional rules, `jsdoc`, `unicorn`, `import-x`, Markdown linting — and which must be rewritten in Rust on Oxc? How much work is each?
- Can the current configurations be kept while the tools are swapped, or must they be translated?
- Does oxfmt format TypeScript and Markdown as Prettier does here (tables, front matter, `proseWrap`, ADR-0065)?
- Which agent tools to rewrite in Rust first (hooks, status line, session report, `activity`, a context tool), and what each saves in time and tokens, measured?
- How fast can `check:all` get, lane by lane, and which target to set?
- How is the Rust code built, tested and checked in this repository (rustfmt, clippy; Rust stable 1.99.1 and the `wasm32` target are installed on the Product Owner's machine) without a runtime dependency of the library?
- What do the gathered notes (`ai-optimization-research.md`, `ret-003-research.md`, `ret-003-facts.md`, `rust-oxc-epic-input.md`) claim that is verified, and what is not?

## Tasks

One task = one commit, referenced as `SP-021.Tn`.

- [x] T1 — Read voidzero.dev and Oxc's documentation; record the sources (3 h)
- [x] T2 — Register the gathered research notes as sources, each claim checked (3 h)
- [x] T3 — Measure `check:all` lane by lane and test oxlint and oxfmt on this repository (4 h)
- [x] T4 — Write the research note, propose the `check:all` target, draft F19's criteria and stories (3 h)

## Result

Answered in [research note 0023](../../research/notes/0023-f19-oxc-and-rust-tools.md); the sources are the rows `REF-VOIDZERO` to `REF-RUSTUP-TOOLCHAIN` of the [references](../../references.md), section "Oxc and Rust tooling".

- oxlint carries 432 of the 781 rules natively and runs 275 more as JS plugins of the current packages, the 10 project rules among them; 4 core rules, 21 Markdown rules and `functional/no-return-void` are to rewrite; 8 rules need types that Oxc has not.
- oxfmt formats the 2,039 files as Prettier does but for 3 union types, Markdown included, with `proseWrap` `preserve` and `never`.
- `check:all` took 123.4 s on 2026-10-16, bound by the test lane (113 s alone, 64 % setup and import); lint 66.6 s, format 18.7 s and type check 7.4 s go to about 3 s with Oxc.
- A hook in Rust starts in 3 ms against 60 to 136 ms in Node.js; no token is saved by the language alone.
- The Rust layout proposed: one virtual Cargo workspace at the root, crates in `tools/`, `rust-toolchain.toml` at 1.99.1, warnings denied in `.cargo/config.toml`, a `rust` lane in `check:all`.

## Questions for the Product Owner

- The target of `check:all`: option A (about 120 s, F19 only), B (A and the docs build only when the docs change) or C (B and the test lane's setup cut, estimated 50 to 70 s, one more story).
- The 8 rules that need types (`functional/immutable-data`, `prefer-immutable-types`, `type-declaration-immutability`, `no-mixed-types`, `no-conditional-statements`; `@typescript-eslint/naming-convention`, `prefer-destructuring`, `no-unsafe-enum-assignment`): rewritten without types, which checks less (a `readonly` written, not a type proven immutable), or dropped.
- The rules to rewrite (`no-restricted-syntax` and the others): as JavaScript rules run by oxlint's plugin API, in the same pass, or in Rust as a second linter on the `oxc` crate, as the decision of 2026-10-16 on new tools in Rust would read.
- A coverage floor for the Rust tools (no coverage tool installed), and an advisory check of crates (`cargo audit` or `cargo deny`, not installed).

## For the main thread

- Notes index (`docs/research/notes/README.md`), a row after 0022: `[0023](./0023-f19-oxc-and-rust-tools.md)` — "Sources of F19: oxlint and oxfmt tried on the repository, the rules left to rewrite, `check:all` by lane, hooks timed, the Rust workspace".
- Feature F19: its acceptance criteria and its stories, drafted in note 0023, sections "F19's acceptance criteria and stories (draft)" and "The Rust layout", for the refinement.
- `.claude/rules/tooling.md`, at F19's ADR: TypeScript stays at 6.0 because TypeDoc 0.28.20 asks for it, not only typescript-eslint.
