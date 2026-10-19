---
paths:
  - "*.config.ts"
  - "*.test.ts"
  - "package.json"
  - ".github/**"
  - ".claude/**"
---

# Tooling pitfalls

Each is a trap met in this repository's tooling, with the story that met it. The tools themselves: `docs/tooling/`.

## The toolchain

- **TypeScript 7 is the only TypeScript, VitePress is on 2.0 alpha** (ADR-0075): TypeScript 7 has no API until 7.1, so no tool imports `typescript`; the API pages come from `tools/api-pages`, the declarations from Oxc (ADR-0071); VitePress 1.6 bundles a vulnerable Vite 5. Either changes only with an ADR.
- **oxlint, then the project's Rust linter `tools/lint`; oxfmt through `tools/format`** (ADR-0075): a rule oxlint lacks is written in `tools/lint`, never as a JavaScript plugin. An oxlint upgrade may rename or remove rules: `tests/oxlint-config.test.ts` fails on a rule oxlint no longer has; read its release notes and decide its new rules in `.oxlintrc.json`.
- A new dev package needs its row in `docs/tooling/dependencies.md`, its release notes read, a version at least 7 days old and `npm audit` clean; a GitHub Action, a Node.js bump and a crate, `Cargo.lock`'s included, are checked by `npm run deps:outdated` too.
- A check tool is given explicit targets, never `.` nor `**/*`: a new tracked folder joins the targets in `package.json`, or `tests/check-targets.test.ts` fails (EN-045). A tooling change that makes existing files invalid is committed with their fix.
- Vite's `?raw` imports are typed by `vite/client` (tsconfig `types`): never a `.d.ts` file for them.

## Tools of our own

- **Every new script, hook or agent tool is a Rust crate of `tools/`** (ADR-0073), so that it starts fast and is checked like the rest: no crates.io dependency but the reviewed ones of `[workspace.dependencies]` (`docs/tooling/dependencies.md`, Crates), its tests beside it, a member of the root workspace by its folder alone with `[lints] workspace = true`, checked by `check:rust` (`cargo fmt`, Clippy with warnings denied, the RustSec advisories) and `rust:coverage` (93 % of lines and functions covered). Scripts run it with `cargo run --release -q -p <crate> [--bin <binary>] --`, never `--manifest-path` (EN-068); the Claude Code hooks, the status line, the Git hooks and the agent call its release binary by its path, `target/release/<binary>`, rebuilt at `prepare`, after `git-sequences sync`, after `story:finish` and after a pull that changed a Rust input (`post-merge`, `post-rewrite`, EN-131), and `npm run rust:fresh` fails on one older than its sources (ADR-0078).
- The one TypeScript script left, `scripts/default-font-metrics.ts`, runs the library itself (F21.AC2): it runs with `node` and type stripping, so a relative import names the `.ts` file (lint scope `node-run`, `tests/node-run-imports.test.ts`), its pure functions are exported and tested, and its side effects sit under `if (import.meta.main)`.
- A tool that ports a package's code or copies its test cases names the package and its version in its module, and adds it to `NOTICE` with its copyright lines and to `THIRD-PARTY-LICENSES.md` with its licence text as published at that version (ADR-0076, `tests/third-party-licences.test.ts`); only a permissive licence, never into `src/`.
- `unicorn/prefer-import-meta-properties` turns `new URL(".", import.meta.url)` into `import.meta.dirname`, without its trailing slash: build paths with `join()`.
- Text written into a file by `replace` or `replaceAll` is returned by a function (`() => text`), never passed as the replacement string, where `$'`, `$&` and `$1` are patterns (AUD-021: a `$'` copied the rest of a page into it). `target/release/replace` and the Edit tool already do so.

## Tests of tooling

- A timing test never reads the clock (the lint forbids `performance.now()` and `Date.now()`): it bounds the time with Vitest's per-test `timeout`, at least four times the test's duration alone, since a loaded `check:all` ran tests up to 2.5 times slower (EN-036, EN-054), and up to 9 times for a test of 128 ms (EN-129). The measure is written beside the limit as `alone: 210 ms`, the median of 5 runs of its file alone; `[F29.AC1]` (`tests/fitness.test.ts`) fails on a limit without it or under four times it. A test whose four times exceed Vitest's 5 s keeps its size, and its comment says so.
- Every test removes every inherited `GIT_*` variable from each command it starts, `git` itself or a binary that runs it (a Rust test starts it through `test_support::command`, EN-103); the binary, run by the agent in the real repository, inherits nothing harmful and removes none: the `pre-push` hook sets `GIT_DIR`, and a leaked one writes to the real repository (a `git init --bare` turned it bare, EN-050). See [tests that run Git](../../docs/tooling/commands.md#tests-that-run-git).

## Git and the shell

- The pre-push hook refuses a branch named outside `<type>/<id>-<topic>`, `feature/f<nn>-<topic>`, `main` and `develop`, so a worktree's own branch cannot be pushed (F14); a side branch carries its story's id (`docs/val-018-remarks`), or the story auto-merge refuses it.
- A remote branch is deleted with `gh api -X DELETE repos/<owner>/<repo>/git/refs/heads/<branch>`: `git push --delete` runs the pre-push `check:all` each time.
- The `guard-bash` hook skips the bodies of here-documents, which are data, unless a shell reads them from its input (`bash <<EOF`, `… | sh`), and reads each command word by word as the shell does: a guarded command inside a quoted text goes through (`-m "… -n …"`), unless a shell runs that text (`bash -c`, `eval`, `$(…)`), whose commands are judged too (AUD-023, AUD-033). A push naming no destination is judged by the branch of every folder it may run in: only the brief's `cd "<folder>" || exit 9` enters one for certain, so push after it (AUD-033). In a worktree, after the brief's `cd`, a chain of Git reads goes through, a Python program writing a file is refused; on `Write` and `Edit`, a worker is refused a file of the main checkout or of another worktree, links and case resolved (EN-119, AUD-033).
- The shell is zsh: an unquoted `$ids` of several lines is **one** word, so `for id in $ids` runs once (a cleanup loop sent malformed requests for 7 minutes, 2026-10-13). Read lines with `while read -r id; do …; done <<< "$ids"` in a `bash` script; never hide a loop's errors (`2>/dev/null`); a round that changes nothing ends the loop and prints the error.
