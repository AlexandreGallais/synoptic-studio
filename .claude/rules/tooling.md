---
paths:
  - "*.config.ts"
  - "*.test.ts"
  - "package.json"
  - ".github/**"
  - ".claude/**"
---

# Tooling pitfalls

- **TypeScript 7 is the only TypeScript** (`typescript` 7.0.2, ADR-0075): TypeScript 6 and TypeDoc are removed, the API pages come from the Rust generator `tools/api-pages` (EN-072) and the declarations from Oxc (ADR-0071); TypeScript 7 has no API until 7.1, so no tool imports `typescript`. **VitePress is on 2.0 alpha** (1.6 bundles a vulnerable Vite 5). Change either only with an ADR.
- **The linters are oxlint, then the project's Rust linter `tools/lint`; the formatter is oxfmt through `tools/format`** (ADR-0075): ESLint and Prettier are removed; a rule oxlint lacks is written in `tools/lint`, never as a JavaScript plugin.
- Any new dev package: a row in `docs/tooling/dependencies.md` (tested), its release notes read, the latest version at least 7 days old, `npm audit` clean. A new GitHub Action or Node.js bump: `npm run deps:tools` checks them too.
- An oxlint upgrade may rename or remove rules: `tests/oxlint-config.test.ts` fails when `.oxlintrc.json` names a rule oxlint no longer has; read its release notes for new rules and decide them in `.oxlintrc.json`.
- `unicorn/prefer-import-meta-properties` turns `new URL(".", import.meta.url)` into `import.meta.dirname` (no trailing slash): build paths with `join()`.
- A test that reads files is mutation-checked once: break its input on purpose and see it fail.
- A check tool is always given explicit targets (folders and files by name, never `.` nor `**/*`); a new tracked folder joins the targets in `package.json`, or `tests/check-targets.test.ts` fails (EN-045).
- A tooling change that makes existing files invalid is committed together with their fix.
- **Every new script, hook or agent tool is written in Rust** (ADR-0073, E17, 2026-10-16): a crate of `tools/` without a crates.io dependency other than the reviewed ones of `[workspace.dependencies]` (`docs/tooling/dependencies.md`, Crates: Oxc's, EN-072), its tests beside it, a member of the root Cargo workspace by its folder alone, with `[lints] workspace = true`, checked by `check:rust` (`cargo fmt`, Clippy with warnings denied, `cargo deny --offline check advisories`, the tests under `cargo llvm-cov`'s floor of 93 % of lines and functions), its release build run by `check:all` before its lanes; scripts and Git hooks run it with `cargo run --release -q -p <crate> [--bin <binary>] --`, never `--manifest-path` (EN-068). The TypeScript ones below may still be edited until F21 rewrites them (split from F19, 2026-10-17).
- Hooks in `.claude/hooks/` and `scripts/` run with `node` (type stripping): relative imports name the `.ts` file (lint scope `node-run`), exported pure functions tested, side effects under `if (import.meta.main)`. Temporal needs `node --harmony-temporal` on Node.js 24.
- Stylelint is planned for SCSS but not installed (`braces` advisory GHSA-vfj7-8cjw-p6xm): see `docs/tooling/versions-and-security.md`.
- The `guard-bash` hook skips the bodies of here-documents (`<<'EOF' … EOF`), which are data; a guarded command written elsewhere on the line (inside `-m "…"`) is still read: write such text in a here-document.
- Delete a remote branch with `gh api -X DELETE repos/<owner>/<repo>/git/refs/heads/<branch>`: `git push --delete` runs the pre-push hook (`check:all`, about 40 s, 60 s when the docs build runs) each time.
- Vite's `?raw` imports are typed by `vite/client` (tsconfig `types`); never add a `.d.ts` file for it.
- Commit scopes are a closed list (`commitlint.config.ts`): `docs/process/` changes use `docs(docs)`, not `docs(process)` — a refused commit leaves its files staged for the next one (F07).
- A commit-body line that starts with a word and a colon (`word: …`) is read by commitlint as a footer, the lines after it as its value: reword it, or put the colon later in the line (F14).
- Timing tests: the lint forbids `performance.now()` and `Date.now()` (non-deterministic); bound the time with Vitest's per-test `timeout` (`it("…", { timeout: 1000 }, …)`), with a limit at least four times the test's duration alone: a loaded `check:all` ran tests up to 2.5 times slower (EN-036) and failed two timing tests in E15 (EN-054).
- The pre-push hook refuses a branch named outside `<type>/<id>-<topic>`, `feature/f<nn>-<topic>`, `main` and `develop` (`.validate-branch-namerc.json`): a worktree's own branch cannot be pushed; work on a story branch (F14). A side branch carries its story's id (`docs/val-018-remarks`): the story auto-merge refuses any other name (E15).
- No force push on a feature branch, even with lease: rebase the story branch, or merge `develop` into the feature branch (E15, a ruleset bypass).
- The shell is zsh: an unquoted `$ids` holding several lines is **one** word, so `for id in $ids` runs once with all of them (2026-10-13: a cleanup loop sent malformed requests for 7 minutes). Read lines with `while read -r id; do …; done <<< "$ids"` in a `bash` script, never hide the errors of a loop (`2>/dev/null`), and make every loop progress or stop: a round that changes nothing ends it and prints the error.
- Never poll a subagent with a background loop (`until grep … sleep`): its end arrives as a notification on its own, and its transcript may never hold the string looked for (2026-10-13: four loops left running after the stop). Before a stop message, stop every background task still running (TaskStop).
- A test running `git` in a temporary repository removes every `GIT_*` variable from its commands: the `pre-push` hook sets `GIT_DIR`, and `git init --bare` run under it turned the real repository bare (EN-050, `tools/shared-files/tests/shared.rs`).
- Text put into a file by `String.prototype.replace` or `replaceAll` is always returned by a function (`() => text`), never passed as the replacement string: `$'`, `$&` and `$1` in it are read as patterns (AUD-021: a `$'` in a sentence copied the rest of `docs/conventions/lint.md` into it). `npm run replace` and the agent tools already do so; an edit by hand goes through them or the Edit tool.
- A tool that ports a package's code or copies its test cases names the package and its version in its module, adds the package to `NOTICE` with its copyright lines and its licence text, as published at that version, to `THIRD-PARTY-LICENSES.md` (ADR-0076; `tests/third-party-licences.test.ts`); only a permissive licence, and never into `src/`.
