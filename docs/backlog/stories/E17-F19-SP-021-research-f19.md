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
- [ ] T3 — Measure `check:all` lane by lane and test oxlint and oxfmt on this repository (4 h)
- [ ] T4 — Write the research note, propose the `check:all` target, draft F19's criteria and stories (3 h)
