# Commands

| Command | Role |
| --- | --- |
| `npm run dev` | Vite server on `playground/` (`http://localhost:5173`) |
| `npm run build` | `npm audit`, then library build (`dist/editor.js`) + declarations (`dist/types/`) |
| `npm run test` / `test:watch` | Vitest on half the machine's cores (`maxWorkers`, held by `tests/vitest-workers.test.ts`) under the heavy lock, with coverage of `src/` (100 % required) and `playground/` (a floor): library, playground, custom rules, ESLint audit, `tests/` (backlog, bibliography, barrels, diagrams, dependencies, fitness), scripts and hooks |
| `npm run lint` / `lint:fix` | ESLint on the named targets (root `*.{ts,md}`, `.claude/` folders, `.github`, `docs`, `eslint`, `playground`, `scripts`, `src`, `tests`), zero warning tolerated |
| `npm run lint:secrets` | secretlint on every file tracked by Git (`git ls-files`): no key, token or password |
| `npm run format` / `format:check` | the formatter, `tools/format` (oxfmt, then Markdown tables without padding, ADR-0068), on the named targets (root files, `.cargo`, `.claude/` folders, `.github`, `.husky`, `.vscode`, `docs`, `eslint`, `playground`, `scripts`, `src`, `tests`, `tools`); `format:check` changes nothing |
| `npm run typecheck` | `tsc --noEmit`, TypeScript 7 (`@typescript/native`, EN-071): about 0.9 s; a type error exits with 1, not 2 as TypeScript 6 did |
| `npm run fix` | the formatter then ESLint `--fix`, with ESLint's cache: **run before `check`** |
| `npm run check` | the formatter + ESLint + secretlint + `tsc` + Vitest + `npm audit` + knip + `check:rust`: **must be green to finish a task** |
| `npm run check:rust` | the Rust workspace's checks, its own lane of `check:all`: `cargo fmt --all --check`, `cargo clippy --workspace --all-targets --locked`, `rust:advisories`, `rust:coverage`, `cargo check --workspace --all-targets --target x86_64-pc-windows-msvc` (compiled for Windows without linking, so a Unix-only call outside `cfg(unix)` fails on the Mac, EN-070), then `rust:build` (EN-068, EN-069); warnings are errors through `.cargo/config.toml`; about 17 to 20 s on a built workspace against 11 to 14 s with `cargo test` (2026-10-17, four runs each on a loaded machine), the instrumented build of `rust:coverage` taking most of the difference |
| `npm run rust:advisories` | `cargo deny --offline check advisories` (0.20.2, `REF-CARGO-DENY-ADVISORIES`, configured by `deny.toml`): every crate of the root `Cargo.lock` against the local copy of the RustSec database, unmaintained and yanked crates failing too, in about 0.3 s (EN-069); `cargo deny fetch db` refreshes that copy, and the CI fetches it before `check:all`; without cargo-deny or its database it fails and names the command to run |
| `npm run rust:coverage` | the workspace's tests under `cargo llvm-cov` (0.9.1, `REF-CARGO-LLVM-COV`), failing under 93 % of lines or 93 % of functions of the workspace (EN-069, [agent tools](./agent-tools.md#rust)); about 11.5 s against 8.7 s for `cargo test` before EN-070, as it builds the workspace again instrumented; without `cargo-llvm-cov` it fails and names its install command |
| `npm run rust:build` | the Rust tools' release binaries, `cargo build --release --workspace --locked`, into the root `target/release/`; run by `prepare` after `npm install` and by `worktree:new` in a new worktree; the scripts and Git hooks run each tool by `cargo run --release -q -p <crate>`, whose check that the binary is up to date costs about 0.03 to 0.05 s on a built workspace, so the first call after `prepare` is warm (EN-068) |
| `npm run deps:audit` | `npm audit`, any severity fails |
| `npm run deps:outdated` | fails when a dependency is not on its latest version |
| `npm run deps:unused` | knip: unused files, exports and dependencies (`knip.json`) |
| `npm run deps:tools` | fails when a GitHub Action is not on its latest major version or `.nvmrc` not on the latest Node.js LTS |
| `npm run check:all` | `check` + `deps:outdated` + `deps:tools` + docs build + playground build, in four parallel lanes (what CI and the pre-push hook run) |
| `npm run docs:dev` / `docs:build` / `docs:preview` | documentation site (API pages written first by `tools/api-pages`, `npm run docs:api`) |
| `npm run docs:api` | regenerate `docs/api/` from the TSDoc |
| `npm run branches:clean` | fetches with prune, deletes the local branches merged into `develop` or whose own branch on GitHub was deleted (never `main`, `develop`, the current one, nor a story just started: never pushed and without a commit beyond the branch it was made from) and removes their worktrees (one holding changes or a running process's working folder is kept and named), reports the branches, stashes and worktrees left; after each merged feature and each release |
| `npm run records:save` | copies the metrics, the transcripts and the source library into the private records repository `records/` (ignored by Git), commits and pushes (ADR-0041, ADR-0044); at the end of each session and each retrospective |
| `npm run metrics:session` | time and tokens of the current turn; `--markdown` for the chat, `--all` for the conversation ([metrics](./metrics.md#session-report)) |
| `npm run library -- …` | the local source library: `search`, `get`, `read`, `list` ([sources](./sources.md)) |
| `npm run metrics` | Markdown report of the agent's work: commands by count and time, slowest calls, time and tokens per session, feature, story and task ([metrics](./metrics.md)) |
| `npm run replace`, `backlog:status`, `backlog:index`, `worktree:new`, `story:finish`, `mutate`, `val:prepare`, `golden:write`, `golden:check`, `activity`, `epic:check` | the agent's tools ([agent tools](./agent-tools.md)) |
| `npm run font:metrics -- <folder>` | writes the default font's metrics, `src/model/font/inter-metrics.json`, from Inter 4.1's static TrueType files unpacked in the folder (EN-031); the font files are not kept in the repository |
| `npm run playground:build` | static build of the playground into the docs site (`docs/.vitepress/dist/playground/`) |

## Explicit targets

Every check tool is given the folders and files it checks, by name (EN-045): the scripts above, `knip.json` `project`, `tsconfig.json` `include`, Vitest's `include` and coverage `include`. None walks the repository from `.`, so the copies of the repository in `.claude/worktrees/` and the generated folders (`records/`, `coverage/`, `dist/`, the Rust tools' `target/`, `docs/api/`, `docs/.vitepress/{.temp,cache,dist}`) stay out; each tool's ignore list names them too. A new tracked folder must be added to the targets: `tests/check-targets.test.ts` fails until it is.

## One heavy command at a time

`check:all`, `npm run test`, `golden:check`, `docs:build` and `mutate` run through the Rust tool `tools/heavy-lock/` (`cargo run --release -q -p heavy-lock`, ADR-0063, EN-047, EN-068): each takes `.git/heavy.lock` in the Git common directory, shared by every worktree, writes its process id, command, worktree and start in it, and holds the system's lock on that file (`flock`, `REF-MACOS-FLOCK`) while its command runs. The system frees that lock when the process ends, even after a force-quit; the file only names the owner in the waiting message (AUD-020).

| Situation | What happens |
| --- | --- |
| no lock | the command takes it, runs, then removes it |
| a lock held by a running command | the command waits, and says once which process, command and worktree holds it, and for how long |
| a lock file no process holds (left by a force-quit) | the command says so and takes it over at once, whatever process id the file names; of two commands taking it over at once, one runs, the other waits for it |
| a command started by the lock's owner | it runs inside that lock: `check:all` runs `npm run test` and `docs:build` itself |
| a folder `heavy.lock` (the lock taken by `mkdir`) | the command waits, its owner unknown: remove it with `rmdir` once its command ended; never take it by hand — a `git push` inside it waits forever for its own pre-push `check:all` (2026-10-17) |
| a lock file held by another tool, naming no process | the command waits: remove it with `rm` once that tool's command ended |

A push inherits the lock through its pre-push hook (`check:all`). Single test files (`npx vitest run <paths>`) take no lock.

`story:finish` takes a second lock the same way, `.git/merge.lock` (`heavy-lock --lock merge`, ADR-0065 decision 5, EN-051): two stories finishing at once merge one after the other, the second rebasing on the base the first updated. The two locks are apart: each names its owner to its commands in its own variable (`HEAVY_LOCK_PID`, `MERGE_LOCK_PID`), so the push under the merge lock still waits for the heavy one.

## Playground

`playground/index.html` loads `playground/index.ts`, which may only import the public API (`../src`). It is published with the docs site on each release: <https://alexandregallais.github.io/synoptic-studio/playground/> (menu _Playground_). It will show each pipeline stage (integer model → `d` → `<path>`) as features arrive.

## Build

Vite library mode (`build.lib`), a single unminified ESM output; `tsc -p tsconfig.build.json` emits the `.d.ts`. `package.json`: `"sideEffects": false`, `exports` points to `dist/`.

## CI

See [ci.md](./ci.md).

## IDE

See [formatting](../conventions/formatting.md#format-on-save). WebStorm and VS Code settings are in the repository (`.idea/`, `.vscode/`).
