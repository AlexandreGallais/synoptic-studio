---
id: EN-072
epic: E17
feature: F19
title: The API pages from a Rust generator on Oxc
status: ready
points: 5
---

# E17 · F19 · EN-072 — The API pages from a Rust generator on Oxc

Write the API pages of the documentation site with a Rust generator of the workspace on Oxc's crates, instead of TypeDoc, which needs TypeScript 6's API (Product Owner, 2026-10-17, second round). Sources: [research note 0023](../../research/notes/0023-f19-oxc-and-rust-tools.md), section "TypeDoc without TypeScript 6" (the generator of our own recommended over `deno_doc` and TypeDoc on TypeScript 7); `REF-OXC-CRATE`, `REF-DENO-DOC`, `REF-TYPEDOC-TS7`, `REF-TS71-PLAN`, `REF-TYPEDOC-MARKDOWN` (the pages written today).

## Acceptance criteria

- Given the library's exports, when the generator runs, then it writes one Markdown page per exported function, type and constant (536 on 2026-10-17), their index and the sidebar file that `docs/.vitepress/config.ts` reads, each page with the signature read from the written annotations, the TSDoc description and its tags (F19.AC7).
- Given an export without its TSDoc block, a `{@link}` or `@see` that names nothing exported, or an internal module reachable from the API, when the generator runs, then it fails naming each, as TypeDoc's configuration did (warnings as errors, every export documented, every link resolved, nothing internal published) (F19.AC7).
- Given `npm run docs:build`, when it runs, then the site builds with the generated pages, its API sidebar the same as TypeDoc's, and every link resolves (F19.AC7).
- Given the generator's crate, when the story ends, then `typedoc`, `typedoc-plugin-markdown`, `typedoc-vitepress-theme` and `typedoc.json` are out, Oxc's crates have their rows in `docs/tooling/dependencies.md` (0.153.0 or later, at least 7 days old), and its tests cover each of the three checks (F19.AC6, F19.AC7).

## Tasks

One task = one commit, referenced as `EN-072.Tn`.

- [ ] T1 — The generator's crate: the exports read with `oxc_parser` and `oxc_semantic`, one page per function (3 h)
- [ ] T2 — The pages of types and constants, the index and the sidebar file, compared with TypeDoc's output (3 h)
- [ ] T3 — The three checks, each with a test that fails on a planted case (2 h)
- [ ] T4 — `docs:api` calling the generator, TypeDoc and its plugins removed, the site built (2 h)
