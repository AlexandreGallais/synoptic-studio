# Commands

| Command | Role |
| --- | --- |
| `npm run dev` | Vite server on `playground/` (`http://localhost:5173`) |
| `npm run build` | `npm audit`, then the library (`dist/editor.js`) and its declarations (`dist/types/`, [build](#build)) |
| `npm run test` / `test:watch` | Vitest under the heavy lock, on half the machine's cores: the library, the playground, `tests/`, the scripts and hooks; coverage of `src/` 100 %, of `playground/` a floor |
| `npm run lint` | oxlint with its type-aware rules and TypeScript's own errors (`--type-check`, `.oxlintrc.json`, `REF-OXLINT-CLI`), then the project's linter (`tools/lint`) with the project's rules and those oxlint lacks, on TypeScript and Markdown; zero warning tolerated |
| `npm run lint:secrets` | secretlint on every file Git tracks: no key, token or password |
| `npm run format` / `format:check` | the formatter, `tools/format`: oxfmt, then Markdown tables without padding (ADR-0068); `format:check` changes nothing |
| `npm run typecheck` | `tsc --noEmit` (TypeScript 7); outside `check` and `check:all`, where the lint reports the same errors |
| `npm run fix` | the formatter, oxlint `--fix`, the project's linter `--fix`: **before each commit** |
| `npm run check` | format, lint, secretlint, Vitest, `npm audit`, knip and `check:rust` one after the other, without the builds |
| `npm run check:all` | `rust:build`, then the [five lanes](#the-lanes-of-the-full-check) at once: what the pre-push hook and the CI run; about 40 s on a code story, a minute with the docs build |
| `npm run check:rust` | the Rust workspace: `cargo fmt --check`, Clippy, `rust:advisories`, `rust:coverage`, and a check for Windows without linking, so that a Unix-only call outside `cfg(unix)` fails on the Mac, then `rust:fresh`; warnings are errors (`.cargo/config.toml`) |
| `npm run rust:advisories` | cargo-deny against the local copy of the RustSec database (`deny.toml`, `REF-CARGO-DENY-ADVISORIES`): unmaintained and yanked crates fail too; `cargo deny fetch db` refreshes the copy |
| `npm run rust:coverage` | the workspace's tests under `cargo llvm-cov` (`REF-CARGO-LLVM-COV`), failing under the [floor](./agent-tools.md#rust) |
| `npm run rust:build` | the Rust tools' release binaries into `target/release/`; run by `prepare` and `worktree:new`, and the same build by `git:sync` and `story:finish`: the hooks, the status line, the Git hooks and the agent call these binaries by their path (ADR-0078), and the first `cargo run --release -q -p <crate>` of a script is warm |
| `npm run rust:fresh` | each release binary older than a file Cargo built it from, or missing, then exit 1 (`tools/stale-binaries/`, ADR-0078) |
| `prepare` (run by `npm install`) | `rust:build`, then `target/release/git-hooks install`: the hooks folder `.githooks` set once ([Git workflow](./git-workflow.md#hooks-githooks)) |
| `npm run deps:audit` | `npm audit`: any severity fails |
| `npm run deps:outdated` | fails when a package is not on its latest version ([versions](./versions-and-security.md)) |
| `npm run deps:unused` | knip: unused files, exports and dependencies (`knip.json`) |
| `npm run deps:tools` | fails when a GitHub Action is not on its latest major, or `.nvmrc` not on the latest Node.js LTS |
| `npm run docs:dev` / `docs:build` / `docs:preview` | the documentation site, its API pages written first ([documentation site](./documentation-site.md)) |
| `npm run docs:api` | `docs/api/` from the TSDoc |
| `npm run playground:build` | the playground, built into the site (`docs/.vitepress/dist/playground/`) |
| `npm run branches:clean` | deletes the local branches merged into `develop` or gone from GitHub, with their worktrees, and reports what is left; never `main`, `develop`, the current branch, a story just started, nor a worktree holding changes or a running process (`branches-clean` of `tools/repo-tools/`, EN-103) |
| `npm run records:save` | copies the metrics, the transcripts and the source library into the private records repository `records/` (ADR-0041, ADR-0044); at the end of each session and retrospective. A file whose copy already has its size and modification time is not copied again, so the transcripts' gigabytes are written once; the library, the file `AGENT_LIBRARY_FILE` names as for the `library` tools, is copied by the system's `sqlite3 -readonly … "VACUUM INTO …"`, whose bytes differ from Node.js's SQLite but not its rows (`records-save` of `tools/repo-tools/`, EN-103) |
| `npm run metrics`, `metrics:session` | the report of the agent's work, the session report ([metrics](./metrics.md)) |
| `npm run library -- …` | the local source library ([sources](./sources.md)) |
| `npm run font:metrics -- <folder>` | writes `src/model/font/inter-metrics.json` from Inter's static TrueType files unpacked in the folder, which the repository does not keep |
| `npm run replace`, `backlog:status`, `backlog:index`, `worktree:new`, `git:sync`, `git:commit`, `run:start`, `run:end`, `run:should-stop`, `context:size`, `page`, `story:finish`, `mutate`, `val:prepare`, `golden:write`, `golden:check`, `activity`, `epic:check` | the agent's tools ([agent tools](./agent-tools.md)) |

**When to run what**: a task ends with `npm run fix` and the tests it touched (`npx vitest run <paths>`); `check:all` runs at the push, through the pre-push hook. `check` is not run per task: it repeats what the push runs.

## The lanes of the full check

`check:all` builds the Rust binaries first, then runs the lanes at once, the scripts of a lane in order.

| Lane | Runs | Skipped when |
| --- | --- | --- |
| lint | `lint` | never |
| test | `test` | never |
| docs | `docs:build`, then `playground:build` | the branch changed nothing the site is built from against its base — `docs/`, `src/`, `playground/`, `tools/api-pages/`, `Cargo.toml`, `Cargo.lock`, `rust-toolchain.toml`, `package.json`, `package-lock.json`, `vite.config.ts` — committed or not, a moved file counting under its old path too |
| checks | `format:check`, `lint:secrets`, `deps:audit`, `deps:unused`, `deps:tools`, `deps:outdated` | never |
| rust | `check:rust` | never |

The base is the nearest of `origin/develop` and `origin/feature/*` for a story, `origin/develop` for a feature. When the base is unclear — `main`, `develop`, a detached head, the CI, a Git error — every lane runs: skipping is allowed only when it is certain the lane has nothing to check. A line says when the docs lane is skipped. The lanes run inside `check:all`'s heavy lock, and no release binary is built once they start: two lanes starting the same binary while `cargo run` rewrites it fail with `could not execute process`.

`check:all` reads the main checkout's `.git/config` byte for byte before the build and after the lanes. A lane that changed it fails the run, whatever its checks said: a line `✗ <path> changed while the checks ran, restored nothing:` names each line removed (`- `) and added (`+ `), and the last line names `.git/config` among the failures. Nothing is restored: the change is a test's leak to find (a `core.bare = true` written by a test that inherited the pre-push hook's `GIT_DIR`, 2026-10-17), and the report holds the lines to put back. The tool is `check-all` of `tools/repo-tools/` (EN-103); its own work outside the lanes, on a repository whose every script is `true`, takes about 120 ms in the CI's mode against 230 ms for its TypeScript predecessor (median of 20, load 7 to 9).

## Explicit targets

Every check tool is given the folders and files it checks by name — the scripts, `knip.json`'s `project`, `tsconfig.json`'s `include`, Vitest's `include` and coverage — and never walks the repository from `.`. So the worktrees' copies in `.claude/worktrees/` and the generated folders (`records/`, `coverage/`, `dist/`, `target/`, `docs/api/`, VitePress's caches) stay out without each tool knowing them. A new tracked folder is added to the targets; `tests/check-targets.test.ts` fails until it is.

## Tests that run Git

Every test removes every inherited `GIT_*` variable from each command it starts, `git` itself or a binary that runs it, then sets only those it needs (`GIT_CONFIG_GLOBAL` for a test configuration, for instance). The pre-push hook sets `GIT_DIR` before `check:all` runs the tests: inherited, it points the test's commands at the real repository instead of the temporary one, and a `git init --bare` run that way once turned the real repository bare (2026-10-17). It applies to every test language — Vitest's `env` of `spawnSync`; in Rust, one helper for every crate, `test_support::command` of `tools/test-support/` (EN-103), whose own test plants a `GIT_DIR` and proves it does not reach the command — and to every test, even one that passes when run alone: the variable exists only under the hook. The rule binds the tests, not the binaries: run by the agent in the real repository, a binary inherits no `GIT_DIR` of a hook, so `git-sequences`, `check-handback` and `run-totals` remove none.

## One heavy command at a time

`check:all`, `npm run test`, `golden:check`, `docs:build` and `mutate` take the heavy lock themselves (`tools/heavy-lock/`, ADR-0072): `.git/heavy.lock` in the Git common directory, shared by every worktree. Several at once, from parallel worktrees, froze the machine in E15. The lock is the system's file lock (`flock`, `REF-MACOS-FLOCK`), freed when the process ends, even after a force-quit; the file only names its owner — process, command, worktree, start — for the waiting message.

| Situation | What happens |
| --- | --- |
| no lock | the command takes it, runs, then removes it |
| the lock held by a running command | the command waits, and says once who holds it and for how long |
| a lock file no process holds | the command says so and takes it over |
| a command started by the lock's owner | it runs inside that lock: `check:all` runs `npm run test` and `docs:build` itself, a push inherits it through its pre-push |
| a folder `heavy.lock`, or a file held by another tool | the command waits, its owner unknown: remove it (`rmdir`, `rm`) once its command ended |

Never take the lock by hand (`mkdir`) nor wrap a command in a lock of one's own: the command inside waits for a lock it cannot see the owner of, and a `git push` waits forever for its own pre-push `check:all`. Single test files (`npx vitest run <paths>`) take no lock.

`story:finish` takes a second lock the same way, `.git/merge.lock` (`heavy-lock --lock merge`, ADR-0065): two stories finishing at once merge one after the other, the second rebasing on the base the first updated. Each lock names its owner in its own variable (`HEAVY_LOCK_PID`, `MERGE_LOCK_PID`), so a push under the merge lock still waits for the heavy one.

## Playground

`playground/index.html` loads `playground/index.ts`, which imports the public API only (`../src`): the playground uses the library as a user would. It is published with the site at each release: <https://alexandregallais.github.io/synoptic-studio/playground/>.

## Build

Vite's library mode, one unminified ESM file; `package.json` has `"sideEffects": false` and `exports` pointing to `dist/`. `tools/declarations` writes the `.d.ts` of each module alone with Oxc's isolated declarations (ADR-0071), so every exported constant needs a type it can read: one written, or a plain literal; otherwise the build fails (TypeScript's TS9010).

## IDE

[Format on save](../conventions/formatting.md#format-on-save); WebStorm and VS Code settings are in the repository (`.idea/`, `.vscode/`).
