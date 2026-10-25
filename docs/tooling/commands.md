# Commands

| Command | Role |
| --- | --- |
| `npm run dev` | Vite server on `playground/` (`http://localhost:5173`) |
| `npm run preview` | the playground's build served by Vite (`vite preview`) |
| `npm run build` | `npm audit`, then the library (`dist/editor.js`) and its declarations (`dist/types/`, [build](#build)) |
| `npm run test` | Vitest under the heavy lock, on half the machine's cores: the library, the playground, `tests/`, the scripts and hooks, but the timing tests; coverage of `src/` 100 %, of `playground/` a floor |
| `npm run test:timings` | the timing tests alone, under the heavy lock: those tagged `timing` (`--tags-filter=timing`, `REF-VITEST-TAGS`) in the files the script names, about 4 s; `check:all` runs it after its lanes, and the CI after the tests in their job (REV-008) |
| `npm run lint` | oxlint with its type-aware rules and TypeScript's own errors (`typeAware` and `typeCheck` in the `options` of `.oxlintrc.json`, the flags `--type-aware` and `--type-check` of `REF-OXLINT-CLI`, `REF-OXLINT-CONFIG-SCHEMA`), then the project's linter (`tools/lint`) with the project's rules and those oxlint lacks, on TypeScript and Markdown; zero warning tolerated |
| `npm run lint:secrets` | the secret scan (`tools/secret-scan`, secretlint's recommended rules in Rust) on every file Git tracks, less those `.secret-scan-ignore` and the `.gitignore` files name: no key, token or password; exits 1 naming each file, line and rule, the secret masked |
| `npm run format:check` | the formatter, `tools/format`, with `--check`: oxfmt, then Markdown tables without padding (ADR-0068), changing nothing; `fix` runs it with `--write` |
| `npm run fix` | the formatter, oxlint `--fix`, the project's linter `--fix`, then for the Rust code Clippy's fixes (`cargo clippy --fix`, the pedantic lints included) when a `.rs` or Cargo file changed, and `cargo fmt`: **before each commit** |
| `npm run check:all` | `rust:build`, then the [six lanes](#the-lanes-of-the-full-check) at once: what the pre-push hook runs, but for a push that only deletes branches ([hooks](./git-workflow.md#hooks-githooks)), and the CI one lane per job ([CI](./ci.md#the-lanes-as-jobs)); on a story or a bubble branch, only the checks its change needs ([the push of a story](#the-push-of-a-story)) |
| `npm run check:rust` | the Rust workspace: `cargo fmt --check`, Clippy, `rust:advisories`, and a check for Windows without linking, so that a Unix-only call outside `cfg(unix)` fails on the Mac, then `rust:fresh`; warnings are errors (`.cargo/config.toml`) |
| `npm run rust:advisories` | cargo-deny against the local copy of the RustSec database (`deny.toml`, `REF-CARGO-DENY-ADVISORIES`): unmaintained and yanked crates fail too; `cargo deny fetch db` refreshes the copy |
| `npm run rust:coverage` | the workspace's tests under `cargo llvm-cov` (`REF-CARGO-LLVM-COV`), failing under the [floor](./agent-tools.md#rust) of 95 % of lines or of functions: a lane of `check:all` of its own, beside `check:rust` (EN-130) |
| `npm run rust:build` | the Rust tools' release binaries into `target/release/`; run by `prepare` and `worktree-new`, and the same build by `git-sequences sync` and `story:finish`: the hooks, the status line, the Git hooks and the agent call these binaries by their path (ADR-0078), and the first `cargo run --release -q -p <crate>` of a script is warm |
| `npm run rust:fresh` | each release binary older than a file Cargo built it from, or missing, then exit 1 (`tools/stale-binaries/`, ADR-0078) |
| `prepare` (run by `npm install`) | `rust:build`, then `target/release/git-hooks install`: the hooks folder `.githooks` set once ([Git workflow](./git-workflow.md#hooks-githooks)) |
| `npm run deps:audit` | `npm audit`: any severity fails |
| `npm run deps:outdated` | lists each npm package, GitHub Action, Node.js major, crate and cargo tool behind its latest version at least 7 days old, each locked crate under 7 days, and each cargo tool written at two versions or without its row, then exits 1; exits 2 naming a source that does not answer (`dependencies outdated` of `tools/deps/`, [versions](./versions-and-security.md)) |
| `npm run deps:unused` | the unused files, exports, exported types and dev packages, the unlisted packages and the unresolved imports, then exits 1 (`dependencies unused` of `tools/deps/`, on Oxc's parser, EN-109) |
| `npm run docs:dev` / `docs:build` | the documentation site, its API pages written first ([documentation site](./documentation-site.md)) |
| `npm run docs:api` | `docs/api/` from the TSDoc |
| `npm run playground:build` | the playground, built into the site (`docs/.vitepress/dist/playground/`) |
| `target/release/branches-clean` | deletes the local branches merged into `develop` or gone from GitHub, with their worktrees, and reports what is left; never `main`, `develop`, the current branch, a story just started, nor a worktree holding changes or a running process (`branches-clean` of `tools/repo-tools/`, EN-103) |
| `target/release/records-save` | copies the metrics, the transcripts and the source library into `<AGENT_RECORDS_ROOT>/<project>`, a local folder outside the repository shared by every project, never sent (ADR-0081); at the end of each session and retrospective. A file whose copy already has its size and modification time is not copied again, so the transcripts' gigabytes are written once; the library, the file `AGENT_LIBRARY_FILE` names as for the `library` tools, is copied by the system's `sqlite3 -readonly … "VACUUM INTO …"`, whose bytes differ from Node.js's SQLite but not its rows; then the day's snapshot of the records into the synced folder, as `knowledge snapshot` takes it, a line saying so when `AGENT_KNOWLEDGE_SNAPSHOT` is not set ([the snapshot](./knowledge.md#the-snapshot)) (`records-save` of `tools/repo-tools/`, EN-103, EN-114, EN-153) |
| `target/release/metrics`, `target/release/session-report` | the report of the agent's work, the session report ([metrics](./metrics.md)) |
| `target/release/library …` | the local source library ([sources](./sources.md)) |
| `node scripts/default-font-metrics.ts <folder>` | writes `src/model/font/inter-metrics.json` from Inter's static TrueType files unpacked in the folder, which the repository does not keep |
| `npm run backlog:index`, `story:finish`, `mutate`, `golden:check` | the agent's tools kept as scripts, each run by a tool, a hook or under a lock ([agent tools](./agent-tools.md)) |
| `target/release/<binary>` | every other tool of the agent, by its binary ([agent tools](./agent-tools.md#the-binaries-the-agent-calls)) |
| `npx vitest`, `npx tsc --noEmit`, `npx vitepress preview docs` | Vitest watching, TypeScript 7's own check (the lint reports the same errors), the built site served: for a person at the screen, no script (EN-115) |

`package.json` keeps the scripts a person runs and those something calls by their name — npm's lifecycle, a Git hook, the CI, a lane of `check:all`, a tool, another script — and no other (ADR-0082).

**When to run what**: a task ends with `npm run fix` and the tests it touched (`npx vitest run <paths>`); `check:all` runs at the push, through the pre-push hook.

## The lanes of the full check

`check:all` builds the Rust binaries first, then runs the lanes at once, the scripts of a lane in order, then the timing tests alone (`test:timings`). A timing test is one whose Vitest limit, under the 5 s of Vitest's own, measures its speed; the load of the other lanes slowed the 60 redraws of `[F44.AC3]`, 73 ms alone, past their 960 ms twice in one run, so such a test carries the tag `timing` (`{ tags: ["timing"], timeout: 960 }`), `npm run test` leaves it out (`--tags-filter='!timing'`) and it runs once the lanes have ended, its measure kept (decided 2026-10-24, REV-008). `tests/fitness.test.ts` fails on a limit under 5 s without the tag, and on a file holding a tagged test that `test:timings` does not name, or the reverse.

| Lane | Runs | Skipped when |
| --- | --- | --- |
| lint | `lint` | never |
| test | `test`; on a story branch, only the tests related to the change (below) | never |
| docs | `docs:build`, then `playground:build` | the branch changed nothing the site is built from against its base — `docs/`, `src/`, `playground/`, `tools/api-pages/`, `Cargo.toml`, `Cargo.lock`, `rust-toolchain.toml`, `package.json`, `package-lock.json`, `vite.config.ts` — committed or not, a moved file counting under its old path too |
| checks | `format:check`, `lint:secrets`, `deps:audit`, `deps:unused`, `deps:outdated` | never |
| rust | `check:rust` | the branch changed none of the Rust inputs against its base, as for the docs lane (below) |
| rust coverage | `rust:coverage` | with the rust lane, the two skipped or run together |

The Rust inputs (`RUST_INPUTS` of `tools/stale-binaries/src/lib.rs`, EN-130) are what Cargo, rustup and cargo-deny read — `tools/`, so a new crate by its folder alone, `Cargo.toml`, `Cargo.lock`, `rust-toolchain.toml`, `.cargo/`, `deny.toml` — and the files the crates' tests read outside `tools/`: `package.json` and `.github/workflows/`, `docs/tooling/ci.md` (the lanes against the scripts, the CI's matrix and the checks `main` requires, `[F19.AC9]`, `[F24.AC1]`), `package-lock.json` and `.oxfmtrc.json` (the oxfmt the formatter's tests run), `docs/conventions/files.md` (the lint's folders), `.claude/settings.json` (the hooks), `playground/checklists/` (`val-prepare`) and every `*.test.ts` file (the binaries a test starts, `[F19.AC9]`). `[F29.AC2]` names each entry at the root as a Rust input or one no Rust lane reads whole, so a file Cargo reads added there (a `clippy.toml`) fails until it joins one of them. A story of Markdown or of a script skips both Rust lanes; one that changes a test file runs them. One input stays outside the list: the local copy of the RustSec database that `check:rust`'s advisories read (`npm run rust:advisories`) lives outside the repository, so an advisory published against an unchanged `Cargo.lock` fails no push that skipped the lane; the CI into `main` runs every lane every time and fails on it there (AUD-031).

On a story branch (`<type>/<id>-<topic>`) whose push runs the lanes — its change touching an input every check reads ([the push of a story](#the-push-of-a-story)) —, `deps:outdated` is left out, `rust:coverage` runs the workspace's tests without their coverage (`cargo test --workspace`), and the test lane runs only the tests the change can affect (decided 2026-10-24, REV-008): `vitest related --run --passWithNoTests --tags-filter='!timing'` (`REF-VITEST-RELATED`) on the files changed against the base, committed or not, whose test files Vitest finds through the static imports of the module graph, and on every test file of `tests/` and `scripts/`, which read the repository's files beside their imports (a page of `docs/` checked by `tests/references.test.ts` imports nothing). A line says so: `- test: only the tests related to the change against <base>, without coverage; …`. Coverage is not measured on that run: a part of the tests cannot hold the floor of 100 % of `src/`, which the full `test` keeps on a feature branch, `develop`, `main` and the CI into `main`, so an uncovered line of a story fails at its feature's push. A change to an input every test reads — `vite.config.ts`, `package.json`, `package-lock.json`, `tsconfig.json`, `tests/font-files-setup.ts` and the font file it reads — runs the whole `test` (`related_tests_command` of `tools/repo-tools/src/check_all.rs`). Through the barrels, a change of `src/` reaches most test files (470 of 642 for one file of `src/math/`); one of the playground about 80, one of `tests/` or the docs only the repository's tests.

The base is the nearest of `origin/develop` and `origin/feature/*` for a story, `origin/develop` for a feature. When the base is unclear — `main`, `develop`, a detached head, the CI, a Git error — every lane runs: skipping is allowed only when it is certain the lane has nothing to check. A line says when the docs lane is skipped, another when the Rust lanes are: `- check:rust, rust:coverage skipped: nothing changed in their inputs against <base>`. The lanes run inside `check:all`'s heavy lock, and no release binary is built once they start: two lanes starting the same binary while `cargo run` rewrites it fail with `could not execute process`.

`check:all` reads the main checkout's `.git/config` before the build and after the lanes, and compares the lines a test could break: every key of `[core]`, `[user]`, `[commit]`, `[url "…"]` (an `insteadOf` sends the pushes elsewhere), `[include]`, `[includeIf "…"]` (another file read) and `[extensions]`, and each remote's `url` and `pushurl`, names read without case and a key on its section's header line too (`[core] bare = true`, AUD-023) (`guarded_lines`). The rest is let through: a parallel worktree's `git push -u` writes its branch's `[branch "…"]` tracking lines into the same file while the lanes run (EN-113). A lane that changed a guarded line fails the run, whatever its checks said: a line `✗ <path> changed while the checks ran, restored nothing:` names each line removed (`- `) and added (`+ `), and the last line names `.git/config` among the failures. Nothing is restored: the change is a test's leak to find (a `core.bare = true` written by a test that inherited the pre-push hook's `GIT_DIR`, 2026-10-17), and the report holds the lines to put back. The tool is `check-all` of `tools/repo-tools/` (EN-103); its own work outside the lanes, on a repository whose every script is `true`, takes about 120 ms in the CI's mode (median of 20, load 7 to 9).

## The push of a story

On a story or a bubble branch (`<type>/<id>-<topic>`) whose change is known, `check:all` runs only what the files it changed against its base need (decided 2026-10-25, AUD-050; `scope` of `tools/repo-tools/src/check_scope.rs`): a push took 3 to 4 minutes, its lanes building the site and measuring the Rust coverage whatever it changed. The pre-commit hook already formatted, linted and scanned each file at its commit ([hooks](./git-workflow.md#hooks-githooks)); the push adds what reads beyond one file, the lanes at the same time, a line naming them first: `- scoped push: only what the change against <base> needs (lint, test); …`.

| Changed | Runs |
| --- | --- |
| TypeScript | oxlint and the project's linter on its files, `tsc --noEmit` on the project, the unused files and exports (`target/release/dependencies unused`), then, alone after the other checks, the timing tests related to it |
| Markdown | the project's linter on its files |
| a file outside `tools/` | the Vitest tests related to the change (`vitest related`, as [above](#the-lanes-of-the-full-check)), with the repository's own tests of `tests/` and `scripts/`, which read the docs |
| a crate's folder in `tools/` | `cargo fmt --check`, Clippy and `cargo test` of that crate and of every crate depending on it, directly or not, dev-dependencies included (read from each `tools/*/Cargo.toml`) |

Not run on such a push: the release build, the docs build, `rust:coverage`, the check for Windows, the advisories, the secret scan of every file, `deps:audit` and `deps:outdated`: the dependency updates are grouped at the review. A change to an input every check reads — `Cargo.toml`, `Cargo.lock`, `rust-toolchain.toml`, `.cargo/`, `deny.toml`, `package.json`, `package-lock.json`, `tsconfig.json`, `vite.config.ts`, `.oxlintrc.json`, `tests/font-files-setup.ts` and its font file (`FULL_INPUTS`) — runs the lanes instead, without `deps:outdated`, and with the workspace's tests without coverage in place of `rust:coverage`. The full `check:all`, unchanged, runs on the push of a feature branch, which its pull request into `develop` relies on, and on `develop`, `main` and in the CI.

What a scoped push leaves to the feature's: a Rust test reading a file outside `tools/` (a hook's test reading `.githooks/`, a test reading `package.json` or a `*.test.ts`), a Vitest test starting a release binary older than its crate, the coverage of `src/` at 100 %, the site's dead links. They fail at the feature's push, where the full run holds.

| Push | Before (every lane) | After (scoped) |
| --- | --- | --- |
| a page of `docs/` | 71.9 s: `docs:build` 69.0 s, `test` 23.1 s, `lint` 21.8 s | 8.0 s: `test` 7.8 s, `lint` 0.0 s |
| a line of `tools/shared-files/`, which 8 crates depend on | 181.3 s: `rust:coverage` 164.0 s, `check:rust` 58.7 s, `test` 33.7 s, `rust:build` 14.8 s | 96.9 s, 92.8 s again: `cargo test` of 9 crates 89.2 s (about 45 s to run their 105 test binaries, the rest to build them), Clippy 7.1 s, rustfmt 0.5 s |
| a line of `tools/page/`, which `knowledge` depends on | — | 28.0 s: `cargo test` 24.5 s, Clippy 3.2 s |
| a line of `playground/listeners/` | — | 39.6 s: `test` 28.0 s, `test:timings` 11.4 s, `tsc` 1.2 s, oxlint 0.8 s |

Measured on 2026-10-25 by `check:all` itself on an uncommitted change of one line, its last line's time, once the heavy lock was free; the machine's load was 10 to 34 before, 4 to 10 after. A crate many others depend on stays the slowest push: its tests and its dependents' run one test binary after another.

## Explicit targets

Every check tool is given the folders and files it checks by name — the scripts, the unused check's `PROJECT` (`tools/deps/src/unused.rs`), `tsconfig.json`'s `include`, Vitest's `include` and coverage — and never walks the repository from `.`. So the worktrees' copies in `.claude/worktrees/` and the generated folders (`coverage/`, `dist/`, `target/`, `docs/api/`, VitePress's caches) stay out without each tool knowing them. A new tracked folder is added to the targets; `tests/check-targets.test.ts` fails until it is.

## Tests that run Git

Every test removes every inherited `GIT_*` variable from each command it starts, `git` itself or a binary that runs it, then sets only those it needs (`GIT_CONFIG_GLOBAL` for a test configuration, for instance). The pre-push hook sets `GIT_DIR` before `check:all` runs the tests: inherited, it points the test's commands at the real repository instead of the temporary one, and a `git init --bare` run that way once turned the real repository bare (2026-10-17). It applies to every test language — Vitest's `env` of `spawnSync`; in Rust, one helper for every crate, `test_support::command` of `tools/test-support/` (EN-103), whose own test plants a `GIT_DIR` and proves it does not reach the command — and to every test, even one that passes when run alone: the variable exists only under the hook. The rule binds the tests, not the binaries: run by the agent in the real repository, a binary inherits no `GIT_DIR` of a hook, so `git-sequences`, `check-handback` and `run-totals` remove none.

## One heavy command at a time

`check:all`, `npm run test`, `golden:check`, `docs:build` and `mutate` take the heavy lock themselves (`tools/heavy-lock/`, ADR-0072): `.git/heavy.lock` in the Git common directory, shared by every worktree. Several at once, from parallel worktrees, froze the machine in E15. The lock is the system's file lock (`flock`, `REF-MACOS-FLOCK`), freed when the process ends, even after a force-quit; the file only names its owner — process, command, worktree, start — for the waiting message.

| Situation | What happens |
| --- | --- |
| no lock | the command takes it, runs, then removes it |
| the lock held by a running command | the command waits, looking again each second, and says once who holds it and for how long |
| a lock file no process holds | the command says so and takes it over |
| a command started by the lock's owner | it runs inside that lock: `check:all` runs `npm run test` and `docs:build` itself, a push inherits it through its pre-push |
| a folder `heavy.lock`, or a file held by another tool | the command waits, its owner unknown: remove it (`rmdir`, `rm`) once its command ended |

Never take the lock by hand (`mkdir`) nor wrap a command in a lock of one's own: the command inside waits for a lock it cannot see the owner of, and a `git push` waits forever for its own pre-push `check:all`. Single test files (`npx vitest run <paths>`) take no lock.

`story:finish` takes a second lock the same way, `.git/merge.lock` (`heavy-lock --lock merge`, ADR-0065): two stories finishing at once merge one after the other, the second rebasing on the base the first updated. Each lock names its owner in its own variable (`HEAVY_LOCK_PID`, `MERGE_LOCK_PID`), so a push under the merge lock still waits for the heavy one.

## Playground

`playground/index.html` loads `playground/index.ts`, which imports the public API only (`../src`): the playground uses the library as a user would. It is published with the site at each push to `main` (`site.yml`): <https://alexandregallais.github.io/synoptic-studio/playground/>.

## Build

Vite's library mode, one unminified ESM file; `package.json` has `"sideEffects": false` and `exports` pointing to `dist/`. `tools/declarations` writes the `.d.ts` of each module alone with Oxc's isolated declarations (ADR-0071), so every exported constant needs a type it can read: one written, or a plain literal; otherwise the build fails (TypeScript's TS9010).

## IDE

[Format on save](../conventions/formatting.md#format-on-save); WebStorm and VS Code settings are in the repository (`.idea/`, `.vscode/`).
