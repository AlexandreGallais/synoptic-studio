---
paths:
  - "*.config.ts"
  - "*.test.ts"
  - "package.json"
  - ".github/**"
  - ".agents/**"
  - ".claude/**"
  - "tools/**"
---

# Tooling

The traps of this repository's tooling that a check does not catch by itself. The tools: `docs/tooling/`.

## The toolchain

- **TypeScript 7 is the only TypeScript, VitePress 2.0 alpha** (ADR-0075): no tool imports `typescript`; the API pages come from `tools/api-pages`, the declarations from Oxc. Either changes only with a decision.
- **oxlint, then our Rust linter `tools/lint`; oxfmt through `tools/format`**: a rule oxlint lacks is written in `tools/lint`, never with ESLint nor a JavaScript plugin. An oxlint upgrade may rename rules: `tests/oxlint-config.test.ts` fails on one it no longer has.
- A new dev package, crate, Action or cargo tool: its row in `docs/tooling/dependencies.md`, its release notes read, a version at least 7 days old, `npm audit` clean; `npm run deps:outdated` checks them all.
- A check tool is given explicit targets, never `.` nor `**/*`: a new tracked folder joins the targets in `package.json`, or `tests/check-targets.test.ts` fails. A tooling change that makes existing files invalid is committed with their fix.

## Tools of our own

- **Every script, hook or agent tool is a Rust crate of `tools/`** (ADR-0073): no crates.io dependency but those of `[workspace.dependencies]`, its tests beside it, `[lints] workspace = true`, checked by `check:rust` (fmt, Clippy with warnings denied, RustSec) and `rust:coverage` (95 % of lines and functions covered).
- Scripts run a crate with `cargo run --release -q -p <crate> [--bin <binary>] --`; the hooks, the status line, the Git hooks and the agent call its release binary, `target/release/<binary>`, rebuilt at `prepare`, after `git-sequences sync`, after `story:finish` and after a pull that changed a Rust input; `npm run rust:fresh` fails on one older than its sources (ADR-0078).
- The one TypeScript script, `scripts/default-font-metrics.ts`, runs the library itself with `node`: a relative import names the `.ts` file, its side effects sit under `if (import.meta.main)`.
- A tool porting a package's code names the package and its version in its module, with its rows in `NOTICE` and `THIRD-PARTY-LICENSES.md` (ADR-0076); only a permissive licence, never into `src/`.
- Text written by `replace` or `replaceAll` is returned by a function (`() => text`), never passed as the replacement string, where `$'` and `$&` are patterns.

## Tests of tooling

- A timing test never reads the clock: it bounds the time with Vitest's per-test `timeout`, at least four times the test's duration alone, written beside it as `alone: 210 ms` (the median of 5 runs of its file); `tests/fitness.test.ts` checks it. A limit under 5 s takes the tag `timing`.
- Every test removes each inherited `GIT_*` variable from the commands it starts (a Rust test through `test_support::command`): the `pre-push` hook sets `GIT_DIR`, and a leaked one writes to the real repository.
- A test never writes into the real records: its records root is a temporary folder.

## Git and the shell

- The pre-push hook refuses a branch named outside `<type>/<id>-<topic>`, `feature/f<nn>-<topic>`, `main` and `develop`.
- A remote branch is deleted with `gh api -X DELETE repos/<owner>/<repo>/git/refs/heads/<branch>`, never `git push --delete`, which runs the pre-push checks.
- `guard-bash` reads each command word by word as the shell does, skipping here-documents' bodies unless a shell reads them; a command a shell runs from a text (`bash -c`, `eval`, `$(…)`) is judged too. A push naming no destination is judged by its folder: push after the brief's `cd "<folder>" || exit 9`.
- The shell is zsh: an unquoted `$ids` of several lines is one word. Read lines with `while read -r id; do …; done <<< "$ids"`; never hide a loop's errors.
