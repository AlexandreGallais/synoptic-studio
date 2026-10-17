---
id: EN-083
epic: E17
feature: F19
title: JSDoc's rules in Rust
status: ready
points: 5
---

# E17 · F19 · EN-083 — JSDoc's rules in Rust

Rewrite in the project's Rust linter the 31 kept rules of `eslint-plugin-jsdoc`, which oxlint runs only as JS plugins; the 12 rules on JSDoc types are dropped since `jsdoc/no-types` forbids them (Product Owner, 2026-10-17, fourth round). Sources: [research note 0023](../../research/notes/0023-f19-oxc-and-rust-tools.md), section "Triage of the rules oxlint lacks"; `REF-OXLINT-JS-PLUGINS`; each rule's own documentation in the installed plugin, read before it is rewritten.

## Acceptance criteria

- Given each of the 31 rules listed below, when the project's Rust linter runs on the cases its plugin's documentation gives, then it reports the same cases as invalid and lets the same cases through, each case a test of its crate (F19.AC11).
- Given the tree, when `npm run lint` runs, then the rewritten rules report nothing, and the findings they give on a copy with one planted case each are those ESLint gives (F19.AC11).
- Given ESLint's configuration, when the story ends, then these rules are off in it, so that each rule runs in one linter only, and the triage of note 0023 records each one as rewritten here (F19.AC11).

## Tasks

One task = one commit, referenced as `EN-083.Tn`.

- [ ] T1 — Content (11): `check-param-names`, `check-syntax`, `check-template-names`, `escape-inline-tags`, `informative-docs`, `match-description`, `no-restricted-syntax`, `normalize-see-links`, `no-types`, `ts-ban-ts-comment`, `require-description`, each with its documented cases as tests (3 h)
- [ ] T2 — Layout (10): `check-alignment`, `check-indentation`, `check-line-alignment`, `lines-before-block`, `multiline-blocks`, `no-bad-blocks`, `no-blank-block-descriptions`, `no-multi-asterisks`, `require-asterisk-prefix`, `require-hyphen-before-param-description`, each with its documented cases as tests (3 h)
- [ ] T3 — Tags and requirements (10): `sort-tags`, `tag-lines`, `require-jsdoc`, `require-next-description`, `require-rejects`, `require-returns-check`, `require-template`, `require-template-description`, `require-throws`, `require-yields-check`, each with its documented cases as tests (3 h)
- [ ] T4 — A TSDoc block parser shared by the rules, from the comments `oxc_ast` keeps (2 h)
- [ ] T5 — The rules turned off in ESLint, the findings compared on the tree and on planted cases, the triage updated (1 h)

## Timings (ADR-0069)

Measured on this Mac, 2026-10-17, on `npm run lint`'s targets (2,052 files), other worktrees running beside:

| Tool | Before | After |
| --- | --- | --- |
| Rust linter (`tools/lint`) | 0.13–0.14 s | 0.20–0.21 s, 31 rules more |
| ESLint | 13.0 s, the 33 JSDoc rules taking 3.69 s (`TIMING=all`) | 8.6 s, none of them on |

## For the main thread

- Rules: 31 run in the project's linter, in `tools/lint/src/rules/jsdoc/`: 30 of the 31 kept by note 0023 and `convert-to-jsdoc-comments`, on in ESLint but missing from the triage's count of 43 (the plugin has 45 rules oxlint lacks). Dropped with their reasons in note 0023 and `DROPPED_RULES`: `ts-ban-ts-comment` (oxlint's `typescript/ban-ts-comment`, stricter, reports every comment it reports, checked on its 17 documented cases) and `type-formatting` (it formats JSDoc types, refused by `no-types` but on the error classes of `@throws`, single names with nothing to format; also missing from the triage). None goes to oxlint: version 1.87.0's JSDoc plugin holds only the 12 rules of the native triage.
- Shared reading (`tools/lint/src/jsdoc/`): a port of the parser the plugin reads (`comment-parser` 1.4.9 with `@es-joy/jsdoccomment`'s tokenizers), the same tokens, tags and problems as the JavaScript one on the 3,676 blocks of the tree and the 1,812 of the documented cases; the block a declaration carries found as `getJSDocComment` finds it; the plugin's block edits (`removeTag`, `addTag`, `addLine`), its flow walks (`hasThrowValue`, `hasReturnValue`, `hasRejectValue`, `hasNonFunctionYield`) and V8's sort order of a short array (`sort-tags` reports where V8's comparisons meet the first misplaced tag).
- Compared: every test runs the plugin's own cases (`test/rules/assertions/` at v65.3.1, the source of its documentation's examples) and a fault planted in a copy of a file of the tree, against ESLint 10.12.0's findings and fixed text under the project's options, recorded in `tools/lint/tests/fixtures/jsdoc/`: 1,942 cases, all equal, none Oxc refuses — 1,894 runs of the plugin's cases (7 that ESLint's TypeScript parser refuses left out at recording), 35 planted faults, and 13 cases written for the branches of the flow walks the plugin's cases leave, which brought `jsdoc/flow.rs` from 53 % to 98 % of its lines. With every JSDoc rule on over the 32 planted files, both linters give the same 39 findings; none on the tree. The recording ran with ESLint's report of unused directives off, which otherwise strips `eslint-disable` comments from the fixed text.
- Autofixed as in ESLint: `check-alignment`, `check-line-alignment`, `check-param-names` (order and duplicates), `convert-to-jsdoc-comments`, `lines-before-block`, `multiline-blocks`, `no-bad-blocks`, `no-blank-block-descriptions`, `no-multi-asterisks`, `no-types`, `normalize-see-links`, `require-asterisk-prefix`, `require-hyphen-before-param-description`, `sort-tags`, `tag-lines`.
- ESLint: the 31 rules are `off` in `eslint/rules/jsdoc/`; the library's scope that turned `require-jsdoc` on again for type members (`eslint/scopes/modules.ts`) and the settings only those rules read (`JSDOC_TAG_SEQUENCE`, `DOCUMENTED_CONTEXTS`, `LIBRARY_DOCUMENTED_CONTEXTS`, `JSDOC_DESCRIPTION_PATTERNS`, `LIBRARY_FILES`) are removed. The plugin stays loaded for the dropped and oxlint-owned rules until EN-085.
- Shared lists edited beside EN-080 and EN-082: `rules::named` (one `.chain(jsdoc::RULES)` line), `scopes::linted_by` and its test (the JSDoc names after Unicorn's), `DROPPED_RULES` (two lines), and note 0023's counts (JSDoc 45, 31 and 14; all 267, 199 and 68, EN-080's and EN-082's included after a rebase on the feature branch).
- Two tests of the crate wrote undocumented constants (`lib.rs`, `tests/cli.rs`): their code now carries blocks, since `require-jsdoc` runs on every file. The T3 commit left them failing; T5 fixes them.
- Rule-docs: none affected, the story moves lint rules between linters and changes no rule of `docs/domain/`.

### Journal

- `#run` EN-083: JSDoc's rules carried to the Rust linter on a port of the plugin's block parser, 31 rules equal to ESLint on 1,942 recorded cases and on faults planted in copies of the tree's files; `ts-ban-ts-comment` and `type-formatting` dropped; ESLint 13.0 s to 8.6 s, the Rust linter 0.14 s to 0.21 s.

### Improvements

| 2026-10-17 | EN-083 | A worker that runs a crate's tests by a name filter while it adds a rule misses the older tests the rule breaks: the T3 commit of EN-083 left two tests of `tools/lint` failing, found at T5; run the crate's whole suite before each commit, it takes under a second | process |
| 2026-10-17 | EN-083 | A rule rewritten from a plugin is tested best against the plugin's own test cases with the old linter's findings recorded beside them: the record survives the old linter, and differences show at once; the plugin's assertion files hold structured cases, its Markdown documentation only examples joined by blank lines | tooling |
