# Continuous integration

Every check runs before the code leaves the machine; GitHub checks once more what reaches `main` (ADR-0072). A CI run per story would cost minutes of waiting for checks the pre-push hook already ran.

- **Work branches** (stories, features, `develop`) are checked by the local Git hooks only ([Git workflow](./git-workflow.md#hooks-githooks)): a push that fails them does not leave the machine. No workflow runs on them, and nobody waits for one.
- **A release**, the pull request from `develop` into `main`, runs the CI below; the ruleset of `main` lets it merge only when every check is green. Nothing is checked again after the merge.

| Workflow | Job | When | Does |
| --- | --- | --- | --- |
| `ci.yml` | quality | pull request into `main`; on demand | restores the [Rust caches](#the-rust-build-and-its-caches), builds the release binaries once (`npm run rust:build`), fetches the RustSec database, runs `npm run check:all` ([commands](./commands.md#the-lanes-of-the-full-check)) then `npm run build`; keeps the JUnit test report and the `git-hooks` binary |
| `ci.yml` | history | pull request into `main` | after the quality job, with its `git-hooks` binary: the branch name (`git-hooks branch --name`) and the message of every commit of the pull request (`git-hooks commits`, [commit messages](./git-workflow.md#commit-messages)) |
| `ci.yml` | dependencies | pull request into `main` | GitHub's dependency review: a new vulnerable dependency fails |
| `codeql.yml` | analyze | pull request into `main`, push to `main`, weekly | CodeQL's security and quality queries on the TypeScript; the push keeps the analysis pull requests are compared with (`REF-GH-CODEQL-TRIGGERS`) |
| `release.yml` | release-please | push to `main` or `develop` | the release pull request on `develop`, the tag and the GitHub release on `main`, the test report attached, the site published on the Rust caches ([releases](./releases.md)) |
| `release.yml` | rust-cache | push to `main` | compiles what `check:all` compiles and saves the [Rust caches](#the-rust-build-and-its-caches); checks nothing |
| `links.yml` | links | weekly and on demand | lychee checks every link of the Markdown files (ADR-0022); exclusions in `.lycheeignore` |
| `external-prs.yml` | close | a pull request opened by an outsider | closes it with a pointer to the issue forms (ADR-0026); never checks out its code |
| `story-automerge.yml` | merge | a pull request labelled `autonomous` | merges the owner's story pull requests into a feature branch, and into `develop` a feature validated from its `feat/val-` branch or an epic's `NAM`, `REV` or `RET` the Product Owner validated |

- **A required check is skipped by its job's `if:`, never by the workflow's branch or path filter**: GitHub reports a job skipped by its condition as a success, but a workflow filtered out stays "Pending" and blocks the merge (`REF-GH-REQUIRED-CHECKS`). So a release pull request into `main` (ADR-0038's flow, kept for a rollback) skips every job of `ci.yml` and `codeql.yml` and still merges; release-please's pull request into `develop` runs no CI, its commit checked inside the next pull request into `main`.
- **Secrets** are checked three times: the secret scan (`tools/secret-scan`) on staged files at each commit, the same on every tracked file in `check:all`, and GitHub's secret scanning with push protection.
- **Code hygiene** beyond formatting is the lint's job — no `console`, `debugger`, `TODO`, unused code, nor rule disabled without its reason ([lint](../conventions/lint.md)) — not a check of its own.

## The Rust build and its caches

The checks, the hooks and the docs' generators are Rust binaries of the workspace ([agent tools](./agent-tools.md)), so a job without caches compiles the toolchain's crates, the cargo tools and every dependency from nothing (EN-111). Two caches of the official `actions/cache` (`REF-GH-CACHE`), the same keys and paths in `ci.yml` and `release.yml`, held equal by `tests/ci-caches.test.ts`:

| Cache | Holds | Key |
| --- | --- | --- |
| `rust-tools-…` | the pinned toolchain of `rust-toolchain.toml`, with its components and its Windows target; `cargo-llvm-cov` and `cargo-deny` | the hash of `rust-toolchain.toml` and the two tools' versions (`CARGO_LLVM_COV_VERSION`, `CARGO_DENY_VERSION` in each workflow's `env`, the versions of [dependencies](./dependencies.md#rust-tools)) |
| `rust-build-…` | Cargo's registry (index, crates) and `target/`: the release binaries, Clippy's, the coverage's and the Windows check's builds | the hash of `rust-toolchain.toml` and `Cargo.lock` |

- **Saved on `main` only**, by the `rust-cache` job of `release.yml` at each push to `main`. A cache saved by a pull request's run belongs to its merge ref and is restored only by re-runs of that pull request; a run restores the caches of its own branch and of the default branch (`REF-GH-CACHE`). So the jobs of `ci.yml` and the `docs` job restore them and never save (`actions/cache/restore`, `REF-GH-CACHE-RESTORE`): every pull request into `main` starts from the caches of the last push to `main`, never from those of the previous pull request.
- **`rust-cache` compiles what `check:all` compiles, with the same commands and flags**: `npm run rust:build`, Clippy, the tests under `cargo llvm-cov` given `-- --list`, which compiles them and runs none, and the Windows check. It checks nothing (ADR-0072: nothing is re-checked after the merge); a step that fails only leaves the caches unsaved. Without `restore-keys` there, a new key starts from an empty `target/`, so a cache never carries the build of an older `Cargo.lock`; a pull request may start from an older key's by prefix, compiling only what changed.
- **What a pull request compiles again**: the workspace's own crates, whose checked-out sources are newer than the cached build, never a dependency while `Cargo.lock` and the toolchain are unchanged; the cargo tools are installed only when their key is missed. `CARGO_INCREMENTAL=0`: the incremental state of the workspace's crates would grow the cache and never be reused.
- **Built once per run**: the quality job builds the release binaries in a step of their own, whose log names every crate compiled; `check:all` and `npm run build` then find them up to date. The history job runs the quality job's `git-hooks` binary, passed as an artifact and made executable again, an artifact keeping no file mode (`REF-GH-ARTIFACT-MODES`).
- **Kept 7 days unused, 10 GB per repository**, the least recently used removed first (`REF-GH-CACHE`): with a release every week or two, the caches of `main` may be gone when the next pull request into `main` starts, which then runs cold.
- **On demand**: `ci.yml` has a `workflow_dispatch` trigger, which runs the quality job alone on a branch, the jobs that read a pull request skipped, to time it cold or on the caches. It runs only once `ci.yml` with that trigger is on the default branch (`REF-GH-WORKFLOW-DISPATCH`): `gh workflow run ci.yml --ref <branch>`.

## Repository settings (done once, by the owner)

All under the repository page → **Settings**.

1. **General → Pull Requests**: check **Allow merge commits** (features into `develop`, `develop` into `main`) and **Allow rebase merging** (stories into features); uncheck _Allow squash merging_; check **Automatically delete head branches**, **Always suggest updating pull request branches** and **Allow auto-merge**. **Features → Pull requests**: _Collaborators only_ (ADR-0026).
2. **Actions → General → Workflow permissions**: keep _Read repository contents and packages permissions_ (each workflow declares its own); check **Allow GitHub Actions to create and approve pull requests** (release-please).
3. **Pages → Build and deployment → Source**: **GitHub Actions**.
4. **Advanced Security**: enable **Dependency graph**, **Dependabot alerts**, **Dependabot security updates**, **Secret Protection** with **Push protection**. Leave CodeQL's _Default setup_ **off**: `codeql.yml` is the advanced setup, and both would conflict.
5. **Default branch**: `main` (release-please, CodeQL's baseline, the public page).
6. **Rules → Rulesets** (ADR-0072):

   | Ruleset | Target | Bypass | Rules |
   | --- | --- | --- | --- |
   | `main` | default branch | none | restrict deletions, block force pushes, require a pull request (0 approval — one cannot approve one's own; merge method **Merge** only), require status checks with _branches up to date_: `Format, lint, secrets, types, tests, audit, versions, docs, build`, `Commit messages and branch name`, `Dependency review`, `Analyze` |
   | `develop` | `refs/heads/develop` | none | restrict deletions, block force pushes; no pull request nor status check required (the agent's pushes, checked by the pre-push hook) |
   | `feature branches` | `refs/heads/feature/*` | Repository admin (rebase on `develop` before the feature pull request) | require linear history, require a pull request (0 approval, rebase only); no status check required |

Dependabot opens **security** updates only; they target `main`, the one exception to "releases only" (ADR-0072), pass its full CI, and `develop` then catches up. Version updates are the agent's, after reading the release notes ([versions and security](./versions-and-security.md)).
