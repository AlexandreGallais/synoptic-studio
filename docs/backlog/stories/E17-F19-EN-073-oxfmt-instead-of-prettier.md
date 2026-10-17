---
id: EN-073
epic: E17
feature: F19
title: oxfmt instead of Prettier
status: done
points: 2
---

# E17 · F19 · EN-073 — oxfmt instead of Prettier

Format the repository with oxfmt instead of Prettier, Markdown included, and print Markdown tables without padding while keeping paragraphs as written (ADR-0068, moved from F18). Sources: [research note 0023](../../research/notes/0023-f19-oxc-and-rust-tools.md), section "oxfmt against Prettier" (3 union types differ, no Markdown file differs, 0.24 s against 8.8 s); `REF-OXFMT`, `REF-OXFMT-MIGRATE`, `REF-OXFMT-LANGUAGES`, `REF-OXFMT-MD-DIVERGENCES`.

**Decisions (Product Owner, 2026-10-17).** One formatter for every file kind the repository formats (TypeScript, JavaScript, JSON, Markdown, YAML, CSS…), Prettier removed entirely. Markdown tables are written without padding (ADR-0068), so that parallel branches stop conflicting on table widths: through an oxfmt option or plugin if one exists, otherwise through a small Rust pass of `tools/`, without a crates.io dependency, run by the same `format` and `fix` commands and by `lint-staged`, so that it stays one formatter to use. Whether oxfmt pads tables is measured first; a file kind oxfmt cannot format yet is listed with what formats it instead, a Rust pass preferred to keeping Prettier.

## Measures (2026-10-17)

- **oxfmt pads tables.** oxfmt 0.72.0 on `| a | b |` / `| - | -: |` / `| a much wider cell | 22 |` prints every row padded to the widest cell, with `proseWrap` `"preserve"` and `"never"` alike. Its configuration (`configuration_schema.json`) has no table option, and oxfmt takes no plugin (`REF-OXFMT-MIGRATE`). Hence the Rust pass `tools/format`: oxfmt, then each table without padding.
- **Paragraphs kept**: with `proseWrap: "preserve"` (migrated from Prettier), a paragraph and a list item written on two lines stay on two lines; `"never"` joins them.
- **File kinds**: every kind Prettier formatted here is formatted by oxfmt — TypeScript, `.cjs`, JSON, YAML, CSS, Markdown natively, HTML (`playground/index.html`) through the Prettier oxfmt bundles inside its own package; oxfmt also formats TOML, so `tools/` joins the targets (its 8 `Cargo.toml` already formatted). None is left to another tool; SVG, XML, shell hooks and dotfiles are formatted by neither, as before.
- **Globs**: oxfmt reads `*` at any depth (`oxfmt --check '*'` reads the whole tree, 2 111 files), so `tools/format` replaces the target `*` by the root's files itself.
- **Differences on the tree** before the reformatting commit: the 3 union types of note 0023 (`TextCase`, `LeftOutReason`, `MessageIds`) and 318 of 507 Markdown files whose tables lose their padding; nothing else.
- **Times** on the same targets, Apple M1 Pro: Prettier 3.9.10 `--check` 12.4 s; oxfmt alone 0.41 s; `tools/format --check` (oxfmt, then the Markdown formatted in a temporary copy and compared) 0.50 s.

## Acceptance criteria

- Given `.oxfmtrc.json` written by `oxfmt --migrate=prettier` from `.prettierrc.json` and `.prettierignore`, `printWidth` set to 100 by hand, when `npm run format:check` runs, then oxfmt checks the same targets as Prettier, Markdown included, and the tree is formatted (F19.AC5).
- Given the 3 union types oxfmt lays out one member per line (`TextCase`, `LeftOutReason`, `MessageIds`), when the story ends, then they are formatted by oxfmt, and no other file changes (F19.AC5).
- Given a Markdown table whose widest cell grows, when oxfmt formats the page, then no other row of the table changes, and a paragraph or a list item stays on the lines it was written on (F19.AC12); if oxfmt pads tables, the story stops with the measure and a proposal.
- Given `npm run fix`, `lint-staged` and `check:all`, when they run, then they call oxfmt and not Prettier; oxfmt (0.72.0 or later, at least 7 days old) has its row in `docs/tooling/dependencies.md` (F19.AC5).

## Tasks

One task = one commit, referenced as `EN-073.Tn`.

- [x] T1 — `.oxfmtrc.json` from the migration, the Rust pass `tools/format` (tables without padding, a test on a table whose cell grows), the scripts and `lint-staged` on it, Prettier's package and configuration out (3 h)
- [x] T2 — The docs: formatting conventions, commands, the oxfmt row, ADR-0068's decision 2 marked as carried, the measures (1 h)
- [x] T3 — The tree formatted, in a commit of its own, last, after a rebase on the feature branch (1 h)

The tasks were regrouped on 2026-10-17: the scripts call `tools/format`, so the tool, the scripts and Prettier's removal share one commit (knip refuses an unused `prettier`), and the reformatted tree comes last to keep conflicts with the parallel stories small.

## For the main thread

- EN-085's ADR on the Oxc toolchain names `tools/format` as the formatter (oxfmt, then tables without padding) and `eslint-config-prettier`, still used by ESLint, as left to EN-085.
- Rebased after EN-068 (Cargo workspace): `tools/format` is a workspace member; the scripts `format`, `format:check`, `fix`, `backlog:index`, `.lintstagedrc.json` and `scripts/story-finish-rows.ts` call it by `cargo run --release -q -p format --`. `.cargo/` and `tools/` are formatter targets (their TOML files), `target` is in oxfmt's `ignorePatterns`.
- The reformatting commit (T3) unpads the tables of the shared files (logs, indexes, feature files): `refuse-shared-files` lets a padding-only change through since the fix merged before this story (`fix/en-073-shared-files-table-padding`).
