# Continuous integration

Every check runs before the code leaves the machine; GitHub checks once more what reaches `main` (ADR-0072). A CI run per story would cost minutes of waiting for checks the pre-push hook already ran.

- **Work branches** (stories, features, `develop`) are checked by the local Git hooks only ([Git workflow](./git-workflow.md#hooks-githooks)): a push that fails them does not leave the machine. No workflow runs on them, and nobody waits for one.
- **A review**, the pull request from `develop` into `main` (ADR-0086), runs the CI below; the ruleset of `main` lets it merge only when every check is green. Nothing is checked again after the merge.

| Workflow | Job | When | Does |
| --- | --- | --- | --- |
| `ci.yml` | lanes | pull request into `main`; on demand | one job per lane of `check:all` ([commands](./commands.md#the-lanes-of-the-full-check)), in parallel, each its own check ([the lanes as jobs](#the-lanes-as-jobs)): restores the [Rust caches](#the-rust-build-and-its-caches), builds the release binaries (`npm run rust:build`), fetches every crate of `Cargo.lock` for every target (`cargo fetch --locked`, which `cargo metadata` needs offline), then runs the lane's scripts in turn; the Rust lane installs the cargo tools on a missed key and fetches the RustSec database; the docs lane runs `npm run build` last; the test lane keeps the JUnit test report, the lint lane the `git-hooks` binary |
| `ci.yml` | history | pull request into `main` | after the lanes, with the lint lane's `git-hooks` binary: the branch name (`git-hooks branch --name`) and the message of every commit of the pull request (`git-hooks commits`, [commit messages](./git-workflow.md#commit-messages)) |
| `ci.yml` | dependencies | pull request into `main` | GitHub's dependency review: a new vulnerable dependency fails |
| `codeql.yml` | analyze | pull request into `main`, push to `main`, weekly | CodeQL's security and quality queries on the TypeScript; the push keeps the analysis pull requests are compared with (`REF-GH-CODEQL-TRIGGERS`) |
| `release.yml` | docs | push to `main` | publishes the site and the playground, built on the Rust caches; no release ([releases](./releases.md)) |
| `release.yml` | release | its button on `main` (`workflow_dispatch`) | `release publish` computes the version from the last tag and the commits since, and makes the GitHub release on `main`'s head, its notes the bodies of the pull requests merged since that tag, the test report attached; no commit; nothing without a commit since that tag ([releases](./releases.md)) |
| `release.yml` | rust-cache | push to `main` | fetches every target's crates (`cargo fetch --locked`), compiles what `check:all` compiles and saves the [Rust caches](#the-rust-build-and-its-caches); checks nothing |
| `links.yml` | links | weekly and on demand | lychee checks every link of the Markdown files (ADR-0022); exclusions in `.lycheeignore` |
| `external-prs.yml` | close | a pull request opened by an outsider | closes it with a pointer to the issue forms (ADR-0026); never checks out its code |
| `story-automerge.yml` | merge | a pull request labelled `autonomous` | merges, always by a merge commit (ADR-0086), the owner's story pull requests into their feature branch, and into `develop` a feature branch or a `REV`; refuses any other branch, and a story straight into `develop` |

- **A required check is skipped by its job's `if:`, never by the workflow's branch or path filter**: GitHub reports a job skipped by its condition as a success, but a workflow filtered out stays "Pending" and blocks the merge (`REF-GH-REQUIRED-CHECKS`). So the jobs that read a pull request, `history` and `dependencies`, are skipped by `if: github.event_name == 'pull_request'` on a run started by hand. A release branch's pull request runs every job (ADR-0079): the conditions that skipped release-please's branches went with it (EN-113).
- **Secrets** are checked three times: the secret scan (`tools/secret-scan`) on staged files at each commit, the same on every tracked file in `check:all`, and GitHub's secret scanning with push protection.
- **Code hygiene** beyond formatting is the lint's job — no `console`, `debugger`, `TODO`, unused code, nor rule disabled without its reason ([lint](../conventions/lint.md)) — not a check of its own.

## The lanes as jobs

`check:all` stays what the pre-push hook runs, its lanes at the same time on one machine. On GitHub's runner, the single job that ran it showed nothing until its end: its lanes took 139 s (`lint`), 265 s (`docs:build`), 326 s (`test`) and over 21 min (`check:rust`, compiled without a cache), run 39131147001 (EN-124). So `ci.yml` runs each lane as a job of its own, the entries of one matrix (`REF-GH-JOB-PERMISSIONS`):

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

## The Rust build and its caches

The checks, the hooks and the docs' generators are Rust binaries of the workspace ([agent tools](./agent-tools.md)), so a job without caches compiles the toolchain's crates, the cargo tools and every dependency from nothing (EN-111). Two caches of the official `actions/cache` (`REF-GH-CACHE`), the same keys and paths in `ci.yml` and `release.yml`, held equal by `tests/ci-caches.test.ts`:

| Cache | Holds | Key |
| --- | --- | --- |
| `rust-tools-…` | the pinned toolchain of `rust-toolchain.toml`, with its components and its Windows target; `cargo-llvm-cov` and `cargo-deny` | the hash of `rust-toolchain.toml` and the two tools' versions (`CARGO_LLVM_COV_VERSION`, `CARGO_DENY_VERSION` in each workflow's `env`, the versions of [dependencies](./dependencies.md#rust-tools)) |
| `rust-build-…` | Cargo's registry (index, crates) and `target/`: the release binaries, Clippy's, the coverage's and the Windows check's builds | the hash of `rust-toolchain.toml` and `Cargo.lock` |

- **Saved on `main` only**, by the `rust-cache` job of `release.yml` at each push to `main`. A cache saved by a pull request's run belongs to its merge ref and is restored only by re-runs of that pull request; a run restores the caches of its own branch and of the default branch (`REF-GH-CACHE`). So the jobs of `ci.yml` and the `release` and `docs` jobs restore them and never save (`actions/cache/restore`, `REF-GH-CACHE-RESTORE`): every pull request into `main` starts from the caches of the last push to `main`, never from those of the previous pull request.
- **`rust-cache` compiles what `check:all` compiles, with the same commands and flags**, after `cargo fetch --locked`, so that the registry it saves holds every target's crates, not the host's alone, which the build fetches (`REF-CARGO-FETCH`, EN-116): `npm run rust:build`, Clippy, the tests under `cargo llvm-cov` given `-- --list`, which compiles them and runs none, and the Windows check. It checks nothing (ADR-0072: nothing is re-checked after the merge); a step that fails only leaves the caches unsaved. Without `restore-keys` there, a new key starts from an empty `target/`, so a cache never carries the build of an older `Cargo.lock`; a pull request may start from an older key's by prefix, compiling only what changed.
- **What a pull request compiles again**: the workspace's own crates, whose checked-out sources are newer than the cached build, never a dependency while `Cargo.lock` and the toolchain are unchanged; the cargo tools are installed only when their key is missed, and only by the Rust lane, the one that runs them. `CARGO_INCREMENTAL=0`: the incremental state of the workspace's crates would grow the cache and never be reused.
- **Built once per lane**: each lane's job builds the release binaries in a step of their own, whose log names every crate compiled; its scripts then find them up to date. The history job runs the lint lane's `git-hooks` binary, passed as an artifact and made executable again, an artifact keeping no file mode (`REF-GH-ARTIFACT-MODES`).
- **Kept 7 days unused, 10 GB per repository**, the least recently used removed first (`REF-GH-CACHE`): after 7 days without a review's merge into `main`, the next pull request into `main` runs cold.
- **On demand**: `ci.yml` has a `workflow_dispatch` trigger, which runs the lanes alone on a branch, the jobs that read a pull request skipped, to time them cold or on the caches. It runs only once `ci.yml` with that trigger is on the default branch (`REF-GH-WORKFLOW-DISPATCH`): `gh workflow run ci.yml --ref <branch>`.

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
