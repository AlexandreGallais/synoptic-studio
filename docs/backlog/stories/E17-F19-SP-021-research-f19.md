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
- [x] T5 — Research the Product Owner's answers: the rules that need types, JS plugins, TypeScript 7, Rust checks, the tools around the library, platforms (4 h)
- [x] T6 — Draft F19 again from the answers: criteria, stories, the clean-up story (2 h)
- [x] T7 — Record the second round: the split into F21, TypeScript 7 alone and TypeDoc's replacement, the Rust linter, F20's stories, the product ideas (3 h)

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

## Answers of the Product Owner (2026-10-17)

Recorded as understood; the research each asked for is in [research note 0023](../../research/notes/0023-f19-oxc-and-rust-tools.md), section "The Product Owner's answers".

- **`check:all`**: option B, about 70 to 90 s most of the time. Every tool moves to Oxc and Rust: no ESLint nor Prettier use left, every TypeScript script and hook rewritten in Rust; the library itself stays TypeScript, run as JavaScript.
- **The 8 rules that need types**: none dropped without a stated reason; types from an existing tool where one gives them. Found: `tsgolint` carries none of the 8; `prefer-destructuring` runs natively with no loss for the project's options; `no-unsafe-enum-assignment` is dropped because `enum` is forbidden; `naming-convention` and the 5 functional rules are rewritten without types (or, as a question for the refinement, kept in a 9 s ESLint pass with types).
- **The project's rules**: JS plugins if usual and not noticeably slower. Found: JS plugins are oxlint's documented way; the 10 rules add about 1 s on the whole repository, beside the test lane; so they stay JS plugins.
- **Rust checks**: a coverage floor (`cargo-llvm-cov`) and an advisory check (`cargo-deny`, 0.5 s offline).
- **TypeScript 7**: yes, now — 6 times faster here, the same errors and the same declarations; TypeScript 6 kept under an alias for TypeDoc.
- **Scope widened**: Rust for the project's scripts, hooks, agent tools and the `library` MCP server (`rmcp`); Vite 8 already bundles with Rolldown; Vitest kept.
- **Clean-up**: one story at the end of E17 removes every script, hook, configuration and dev package left unused.
- **Platforms**: the library's tests never depend on macOS; the tools use Rust's portable standard calls where one exists (`File::lock`, Rust 1.89.0), with one cheap check.

## Second round of the Product Owner (2026-10-17)

Recorded as understood; details in note 0023, section "The Product Owner's second round".

- **Split** accepted: F19 holds the checks (11 stories, the API generator added); F21, "Agent tools and the library server in Rust", holds the hooks, scripts, the `library` MCP server and the clean-up of E17, last before NAM-002.
- **The 6 rules without types**: strict form, `readonly` or `Readonly<…>` written in place; nothing kept in ESLint.
- **TypeScript 7 only**: TypeScript 6 removed; TypeDoc replaced by a Rust generator of the API pages on Oxc (recommended over `deno_doc`); Vite, Vitest and VitePress run with TypeScript 7 alone (tried).
- **Every tool in Rust or Go**: the project's rules move from JS plugins to a Rust linter on Oxc's crates (0.153.0); third-party JS plugin rules left as a question for the refinement.
- **F20**: three stories drafted — push notification, progress page as an Artifact, `SubagentStop` hook in Rust.
- **Agent setup**: no cloud agents; scheduled tasks only when asked (`docs/tooling/agent.md`).
- **Product**: Q40 (GitHub only, no npm), Q41 (no accessibility layer nor other language in V1, pro idea), Q42 (font subsetting at export, open).

## For the main thread

- Notes index (`docs/research/notes/README.md`), a row after 0022: `[0023](./0023-f19-oxc-and-rust-tools.md)` — "Sources of F19: oxlint and oxfmt tried on the repository, the rules left to rewrite, `check:all` by lane, hooks timed, the Rust workspace".
- Features F19, F20, F21 and the features index, epic E17's table: edited in this branch on 2026-10-17 (F19 rewritten, F21 created, three stories drafted in F20); to merge with any change made to them on `develop` meanwhile.
- `.claude/rules/tooling.md`, at F19's ADR: TypeScript 7 is the only TypeScript; TypeScript 6 and TypeDoc are removed, the API pages come from a Rust generator (note 0023, "The Product Owner's second round").
- Epic E17, closing table: F21's clean-up story comes before NAM-002.
- Stories index and F20: number the three stories drafted in F20's "Stories to write" (push notification, progress page, `SubagentStop` hook) and create their files.
- F21: create its checkpoint, audit and validation stories, and number its five drafted stories, at its refinement.
- Improvements log (`docs/process/improvements.md`), a row of 2026-10-17: a speed budget test — for example 1,000 symbols drawn under a set time — wanted later by the Product Owner.
- Journal: the Product Owner's two rounds of decisions on SP-021 of 2026-10-17 (note 0023).
