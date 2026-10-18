# Continuous integration

Every check runs before the code leaves the machine; GitHub checks once more what reaches `main` (ADR-0072). A CI run per story would cost minutes of waiting for checks the pre-push hook already ran.

- **Work branches** (stories, features, `develop`) are checked by the local Git hooks only ([Git workflow](./git-workflow.md#hooks-githooks)): a push that fails them does not leave the machine. No workflow checks them, and nobody waits for one: a push to `develop` runs only `rust-caches.yml`, which saves the [Rust caches](#the-rust-build-and-its-caches) and checks nothing (EN-125, EN-132); the auto-merge's merges, pushed by its `GITHUB_TOKEN`, start none (`REF-GH-TOKEN-TRIGGERS`), so only a merge by a person runs it.
- **A review**, the pull request from `develop` into `main` (ADR-0086), runs the CI below; the ruleset of `main` lets it merge only when every check is green. Nothing is checked again after the merge.

| Workflow | Job | When | Does |
| --- | --- | --- | --- |
| `ci.yml` | lanes | pull request into `main` | one job per lane of `check:all` ([commands](./commands.md#the-lanes-of-the-full-check)), in parallel, each its own check ([the lanes as jobs](#the-lanes-as-jobs)): restores the [Rust caches](#the-rust-build-and-its-caches), installs the npm packages (`npm ci`, whose `prepare` builds the release binaries), then runs the lane's scripts in turn; each step a lane needs is chosen by a field of its matrix entry ([the steps of the lanes](#the-steps-of-the-lanes)): the Rust lane installs `cargo-llvm-cov` and `cargo-deny` on a missed key, the test lane `cargo-deny`, and both fetch every crate of `Cargo.lock` for every target and the RustSec database, which cargo-deny reads offline; the docs lane runs `npm run build` last; the test lane keeps the JUnit test report, the lint lane the `git-hooks` binary |
| `ci.yml` | history | pull request into `main` | after the lanes, with the lint lane's `git-hooks` binary: the branch name (`git-hooks branch --name`) and the message of every commit of the pull request (`git-hooks commits`, [commit messages](./git-workflow.md#commit-messages)) |
| `ci.yml` | dependencies | pull request into `main` | GitHub's dependency review: a new vulnerable dependency fails |
| `codeql.yml` | analyze | pull request into `main`, push to `main`, weekly | CodeQL's security and quality queries on the TypeScript; the push keeps the analysis pull requests are compared with (`REF-GH-CODEQL-TRIGGERS`) |
| `site.yml` | site | push to `main` | builds and publishes the site and the playground on the [Rust caches](#the-rust-build-and-its-caches) `rust-caches.yml` saves under `main`, restored only; no release ([releases](./releases.md)) |
| `release.yml` | release | its button (`workflow_dispatch`), its only trigger | `release publish` refuses a branch other than `main`, computes the version from the last tag and the commits since, and makes the GitHub release on `main`'s head, its notes the bodies of the pull requests merged since that tag, the test report attached; no commit; nothing without a commit since that tag ([releases](./releases.md)) |
| `rust-caches.yml` | rust-caches | push to `develop` or `main` | fetches every target's crates (`cargo fetch --locked`), compiles in release what `check:all` compiles, but the test lane's debug build of `backlog-index`, and saves the [Rust caches](#the-rust-build-and-its-caches) under the branch pushed; checks nothing |
| `links.yml` | links | weekly and on demand | lychee checks every link of the Markdown files (ADR-0022); exclusions in `.lycheeignore` |
| `external-prs.yml` | close | a pull request opened by an outsider | closes it with a pointer to the issue forms (ADR-0026); never checks out its code |
| `story-automerge.yml` | merge | a pull request labelled `autonomous` | merges, always by a merge commit (ADR-0086), the owner's story pull requests into their feature branch, and into `develop` a feature branch or a `REV`; refuses any other branch, and a story straight into `develop` |

- **One workflow per purpose, its trigger its only condition** (ADR-0088): each file says its purpose in its name and its first comment, and no job nor step chooses itself by an `if:` reading the event or a branch (`github.event_name`, `github.ref`, `github.ref_name`, `github.ref_type`, `github.base_ref`, `github.head_ref`, and a ref or branch of the event's payload, such as `github.event.ref`, `github.event.pull_request.head.ref` or `github.event.workflow_run.head_branch`, by either syntax, `github['ref']` too, and in any case, `REF-GH-CONTEXTS`, `REF-GH-EXPRESSIONS`); a step chosen by its matrix lane or a cache missed, and a guard on a pull request's label, author or draft state, stay. `[F28.AC1]` (`tests/workflows.test.ts`) reads every workflow and names the file and the line of a condition that breaks it; `story-automerge.yml` is its exception until EN-134 changes it. So the site, the release and the Rust caches, once three jobs of `release.yml` each under its own `if:`, are three workflows (EN-132); and `ci.yml` runs on a pull request into `main` alone, with no job under a condition (EN-133).
- **A required check is never left unrun by a filter**: GitHub reports a job skipped by its condition as a success, but a workflow filtered out by its branch or path filter stays "Pending" and blocks the merge (`REF-GH-REQUIRED-CHECKS`). Every job of `ci.yml` runs on every pull request into `main`, so each required check reports. `ci.yml` is not the only required workflow: `Analyze`, the job of `codeql.yml`, is the second, run on every pull request into `main` too ([repository settings](#repository-settings-done-once-by-the-owner)). Its `workflow_dispatch`, which timed the lanes on a branch (EN-111), went with EN-133, and with it the `if: github.event_name == 'pull_request'` that skipped `history` and `dependencies` on a run by hand; the lanes' times come from the review's run. A release branch's pull request runs every job (ADR-0079): the conditions that skipped release-please's branches went with it (EN-113).
- **Secrets** are checked three times: the secret scan (`tools/secret-scan`) on staged files at each commit, the same on every tracked file in `check:all`, and GitHub's secret scanning with push protection.
- **Code hygiene** beyond formatting is the lint's job — no `console`, `debugger`, `TODO`, unused code, nor rule disabled without its reason ([lint](../conventions/lint.md)) — not a check of its own.

## The lanes as jobs

`check:all` stays what the pre-push hook runs, its lanes at the same time on one machine. On GitHub's runner, the single job that ran it showed nothing until its end: its lanes took 139 s (`lint`), 265 s (`docs:build`), 326 s (`test`) and over 26 min (`check:rust`, compiled without a cache, not ended when the run was cancelled), run 39131147001 (EN-124, EN-125). So `ci.yml` runs each lane as a job of its own, the entries of one matrix (`REF-GH-JOB-PERMISSIONS`):

| Check | Lane's scripts, in turn |
| --- | --- |
| `Lint and types` | `lint` (oxlint with its type check, then the project's lint) |
| `TypeScript tests` | `test` |
| `Docs, playground and build` | `docs:build`, `playground:build`, then `build` |
| `Format, secrets and dependencies` | `format:check`, `lint:secrets`, `deps:audit`, `deps:unused`, `deps:outdated` |
| `Rust` | `check:rust` |

- **Each job its own check**, named by its matrix entry (`REF-GH-CONTEXTS`), with `fail-fast: false`: a lane failing leaves the others running to their end, each with its result on the pull request. Within a job, the first script failing ends it, as `check:all` ends a lane.
- **Every lane is run by exactly one job**: a lane's scripts start one job's, in their order, and no other job runs one of them; `[F24.AC1]` reads the matrix of `ci.yml` against the lanes of `tools/repo-tools/src/check_all.rs`, so a lane added there fails until it has its job.
- **The output as it comes**: a job runs its scripts itself rather than through `check:all`, which prints a lane's output only once it ends. The test job's Vitest reports each test file as it ends and annotates each failing test at its line on the pull request: the CI asks a JUnit report (`VITEST_JUNIT`), and `vite.config.ts` then sets the `default`, `github-actions` and `junit` reporters, a configured list replacing Vitest's own (`REF-VITEST-REPORTERS`, `[F24.AC2]`).
- **The history job waits for every lane**, whose lint lane passes it the `git-hooks` binary; a lane failing so skips it, the failing lane's check already blocking the merge.

## The steps of the lanes

Each step of a lane is needed by one of its scripts or the tests they run (ADR-0088, EN-133). A step every lane needs has no condition; the others are chosen by a field of the lane's matrix entry, a lane lacking the field leaving the step out, a nonexistent property being empty (`REF-GH-CONTEXTS`), never by a comparison of its scripts' names:

| Field | Lanes | Steps it chooses |
| --- | --- | --- |
| `cargo-llvm-cov` | `Rust` (`rust:coverage`) | `cargo install` of `cargo-llvm-cov`, on a missed tools key |
| `cargo-deny` | `Rust` (`rust:advisories`), `TypeScript tests` (`[F19.AC9]`'s advisories test, which runs `cargo metadata` and cargo-deny offline) | `cargo install` of `cargo-deny` on a missed tools key; `cargo fetch --locked`, every crate of `Cargo.lock` for every target, which `cargo metadata` reads offline (`REF-CARGO-FETCH`); `cargo deny fetch db`, the RustSec database |
| `git-hooks` | `Lint and types` | the upload of the `git-hooks` binary, which the history job runs |
| `test-report` | `TypeScript tests` | the upload of the JUnit test report, which the release attaches (ADR-0027) |

- **The release binaries are built once per lane, by `npm ci`**: its `prepare` runs `npm run rust:build`, and its log names every crate compiled. The step `npm run rust:build` that followed it, kept to name them, found them up to date.
- **Every lane installs the npm packages**: each runs a program npm installs, oxlint, Vitest, VitePress, oxfmt through the formatter (`tools/format`) or the installed packages' manifests (`dependencies unused`); the Rust lane, whose scripts run none, runs the formatter's tests under `cargo llvm-cov`, and they run the project's oxfmt (`tools/format/tests`).
- **One build passed as an artifact is not cheaper**: the lanes start together, and a build job before them would add its build to each lane's wait, while each lane's build already runs beside the others'.
- `[F28.AC3]` (`tests/workflows.test.ts`) reads the matrix and the steps of `ci.yml` against the lanes' scripts expanded through `package.json` and the tests that start cargo: each step chosen by a field, the binaries built once per lane, `npm ci` only in the lanes running a program npm installs, each cargo tool, the RustSec database and every target's crates only in the lanes calling them.

The steps removed or moved by EN-133, timed at run 39145798246, the review of #387, `develop` into `main`, the build and the tools' caches missed:

| Step | Before | After | Time then |
| --- | --- | --- | --- |
| `npm run rust:build` | every lane, after `npm ci` | removed: `npm ci`'s `prepare` builds the binaries | 0 s to 1 s; the build inside `npm ci`: 1 min 35 s (lint), 2 min 07 s (tests), 2 min 12 s (docs), 2 min 08 s (format), 2 min 04 s (Rust) |
| `cargo install` of `cargo-llvm-cov` | Rust and test lanes, one step with `cargo-deny` | the Rust lane alone, a step of its own | the step with both: 2 min 47 s (Rust), 2 min 57 s (tests) |
| `cargo install` of `cargo-deny` | the same step | Rust and test lanes, a step of its own | in the same step |
| `cargo fetch --locked` | every lane | Rust and test lanes | 0 s in each lane |
| `cargo deny fetch db` | Rust and test lanes, by their scripts' names | the same lanes, by `cargo-deny` | 1 s |

Each job's time at that run, before EN-133; the times after it are measured at the next review's CI into `main` and written here by that review, no such run having followed EN-133 yet (F28, criterion 3).

| Job | At run 39145798246 | After EN-133 |
| --- | --- | --- |
| Lint and types | 2 min 33 s | the next review's |
| TypeScript tests | 7 min 44 s | the next review's |
| Docs, playground and build | 4 min 41 s | the next review's |
| Format, secrets and dependencies | 2 min 54 s | the next review's |
| Rust | 7 min 34 s | the next review's |
| Dependency review | 12 s | the next review's |
| Commit messages and branch name | 9 s, after the lanes | the next review's |

## The Rust build and its caches

The checks, the hooks and the docs' generators are Rust binaries of the workspace ([agent tools](./agent-tools.md)), so a job without caches compiles the toolchain's crates, the cargo tools and every dependency from nothing (EN-111). Two caches of the official `actions/cache` (`REF-GH-CACHE`), the same keys and paths in `ci.yml`, `rust-caches.yml`, `site.yml` and `release.yml`, held equal by `tests/ci-caches.test.ts`:

| Cache | Holds | Key |
| --- | --- | --- |
| `rust-tools-…` | the pinned toolchain of `rust-toolchain.toml`, with its components and its Windows target; `cargo-llvm-cov` and `cargo-deny` | the hash of `rust-toolchain.toml` and the two tools' versions (`CARGO_LLVM_COV_VERSION`, `CARGO_DENY_VERSION` in each workflow's `env`, the versions of [dependencies](./dependencies.md#rust-tools)) |
| `rust-build-…` | Cargo's registry (index, crates) and `target/`: the release binaries, Clippy's, the coverage's and the Windows check's builds | the hash of `rust-toolchain.toml` and `Cargo.lock` |

- **Saved on `develop` and `main`, by `rust-caches.yml` alone**, at each push to either branch, under its name (EN-125, AUD-030). A run on `main` restores only `main`'s caches (`REF-GH-CACHE`), so the site and the release find the toolchain, the cargo tools and the build the last push to `main` saved. From EN-132 to AUD-030, `main`'s build alone was saved, by `site.yml`, and the tools' cache by no workflow: the site, which installs no cargo tool, cannot save a cache whose key promises `cargo-llvm-cov` and `cargo-deny`. A push to `main` starts the site and `rust-caches.yml` together, so the site restores the caches of the push before; on a key missed, its toolchain is installed at the first cargo call and its binaries compiled, as in a cold run. A cache saved by a pull request's run belongs to its merge ref and is restored only by re-runs of that pull request; a pull request's run restores the caches of its head branch, of its base branch and of the default branch, its head branch's searched first, and never a sibling's (`REF-GH-CACHE`). So the review's pull request, from `develop` into `main`, restores the caches saved by the last push to `develop`, else those of the last push to `main`; a security update of Dependabot, from its own branch, only `main`'s. The jobs of `ci.yml`, `site.yml` and `release.yml` restore them and never save (`actions/cache/restore`, `REF-GH-CACHE-RESTORE`): no pull request starts from the caches of the previous one. Before EN-125 only a push to `main` saved them, and none had happened since 0.2.0: every pull request into `main` ran cold.
- **`rust-caches.yml` compiles in release what `check:all` compiles, with the same commands and flags**, but the debug build of `backlog-index`, which the test lane makes when `tests/backlog.test.ts` runs it by `cargo run`, after `cargo fetch --locked`, so that the registry it saves holds every target's crates, not the host's alone, which the build fetches (`REF-CARGO-FETCH`, EN-116): `npm run rust:build`, Clippy, the tests under `cargo llvm-cov` given `-- --list`, which compiles them and runs none, and the Windows check. It checks nothing (ADR-0072: nothing is re-checked after the merge); a step that fails only leaves the caches unsaved. Without `restore-keys` there, a new key starts from an empty `target/`, so a cache never carries the build of an older `Cargo.lock`; a pull request may start from an older key's by prefix, compiling only what changed.
- **What a pull request compiles again**: the workspace's own crates, whose checked-out sources are newer than the cached build, never a dependency while `Cargo.lock` and the toolchain are unchanged; the cargo tools are installed only when their key is missed, and each only by the lanes that call it: both by the Rust lane, `cargo-deny` alone by the test lane, which `[F19.AC9]` runs with the advisory database both fetch (EN-126, EN-133). `CARGO_INCREMENTAL=0`: the incremental state of the workspace's crates would grow the cache and never be reused.
- **Built once per lane**: each lane's job builds the release binaries in `npm ci`, whose `prepare` runs `npm run rust:build` and whose log names every crate compiled; its scripts then find them up to date (EN-133). The history job runs the lint lane's `git-hooks` binary, passed as an artifact and made executable again, an artifact keeping no file mode (`REF-GH-ARTIFACT-MODES`).
- **The Rust lane's time, cold and warm**. Cold, with no Rust cache in the repository, run 39131147001 (its first attempt, the single job before EN-124): the cargo tools installed in 2 min 50 s, `npm ci` in 2 min 13 s, its `prepare` building the release binaries; `check:rust`, started with the lanes at 13:26:21, had not ended when the run was cancelled at 13:52:28, 26 min 07 s later, in its coverage step. Warm: measured at the run of the review's pull request, #370, that follows the first push to `develop` with EN-125, once the run of that push has saved the caches, and written here then; that push starts both at once, so the run it starts is still cold.
- **Kept 7 days unused, 10 GB per repository**, the least recently used removed first (`REF-GH-CACHE`): after 7 days without a push to `develop` nor to `main`, the next pull request into `main` runs cold.
- **Never on demand**: `ci.yml` had a `workflow_dispatch` trigger to time the lanes on a branch (EN-111), which made two of its jobs skip themselves on such a run; it went with EN-133 (ADR-0088), the times now read at the review's run.

## Repository settings (done once, by the owner)

All under the repository page → **Settings**.

1. **General → Pull Requests**: check **Allow merge commits**, the only method (stories into features, features and reviews into `develop`, `develop` into `main`, ADR-0086); uncheck _Allow rebase merging_ and _Allow squash merging_; check **Automatically delete head branches**, **Always suggest updating pull request branches** and **Allow auto-merge**. **Features → Pull requests**: _Collaborators only_ (ADR-0026).
2. **Actions → General → Workflow permissions**: keep _Read repository contents and packages permissions_ (each workflow declares its own); **Allow GitHub Actions to create and approve pull requests**, checked for release-please, is no longer needed: no workflow opens a pull request since EN-112, and the Product Owner may uncheck it.
3. **Pages → Build and deployment → Source**: **GitHub Actions**.
4. **Advanced Security**: enable **Dependency graph**, **Dependabot alerts**, **Dependabot security updates**, **Secret Protection** with **Push protection**. Leave CodeQL's _Default setup_ **off**: `codeql.yml` is the advanced setup, and both would conflict.
5. **Default branch**: `main` (the release, CodeQL's baseline, the caches, the public page).
6. **Rules → Rulesets** (ADR-0072):

   | Ruleset | Target | Bypass | Rules |
   | --- | --- | --- | --- |
   | `main` | default branch | none | restrict deletions, block force pushes, require a pull request (0 approval — one cannot approve one's own; merge method **Merge** only), require status checks with _branches up to date_: `Lint and types`, `TypeScript tests`, `Docs, playground and build`, `Format, secrets and dependencies`, `Rust`, `Commit messages and branch name`, `Dependency review`, `Analyze` |
   | `develop` | `refs/heads/develop` | none | restrict deletions, block force pushes; no pull request nor status check required (the agent's pushes, checked by the pre-push hook) |
   | `feature branches` | `refs/heads/feature/*` | Repository admin (the main thread's own pushes: `develop` merged in, its docs and rows) | require a pull request (0 approval, merge method **Merge** only: each story a bubble on the feature's line, ADR-0086); no linear history, no status check required |

Dependabot opens **security** updates only; they target `main`, the one exception to "releases only" (ADR-0072), pass its full CI, and `develop` then catches up; the release leaves its body out of the notes, its change written in the next review's text ([releases](./releases.md)). Version updates are the agent's, after reading the release notes ([versions and security](./versions-and-security.md)).
