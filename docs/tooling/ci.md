# Continuous integration

Checks run in two places (ADR-0038):

- **Locally, on every work branch** (stories, features, `develop`): the Git hooks — `pre-commit` (lint-staged), `commit-msg` (commitlint, then the docs of a rule changed, at a `done` commit), `pre-push` (no shared file on a worker's story branch, branch name, `npm run check:all`, then `golden:check` when a golden file is written). A push that fails them does not leave the machine. No workflow runs on these branches and the agent never waits for one.
- **On GitHub, once per release**: the pull request from `develop` into `main` runs the CI below; it can be merged only when every check is green (ruleset of `main`). Nothing is re-checked after the merge.

| Workflow | Job | When | Does |
| --- | --- | --- | --- |
| `ci.yml` | quality | pull request into `main` | `npm run check:all`: the formatter (oxfmt), the linters (oxlint with TypeScript's errors, the project's linter, ESLint; 0 warning), secretlint, Vitest (library, playground, ESLint's configuration audit, dependency and backlog tests), `npm audit`, knip, latest versions, `check:rust` (the Rust workspace's `cargo fmt`, Clippy, RustSec advisories, tests under the coverage floor, after the release build that `check:all` runs before its lanes and after `cargo install` of cargo-llvm-cov and cargo-deny and `cargo deny fetch db`: the runner needs rustup, which installs the toolchain pinned by `rust-toolchain.toml`, not yet seen in this CI), documentation and playground builds; then `npm run build`; keeps the JUnit test report |
| `ci.yml` | history | pull request into `main` | branch name (`validate-branch-name`) and commitlint on every commit of the pull request |
| `ci.yml` | dependencies | pull request into `main` | GitHub dependency review: a new vulnerable dependency fails |
| `codeql.yml` | analyze | pull request into `main`, push to `main`, weekly | CodeQL security and quality queries on the TypeScript code; the push keeps the analysis of `main` that pull requests are compared with (`REF-GH-CODEQL-TRIGGERS`) |
| `release.yml` | release-please | push to `main` or `develop` | on `develop` the release pull request, on `main` the tag and the GitHub release (ADR-0067); attaches the test report of the merged pull request; publishes the site ([releases](./releases.md)) |
| `links.yml` | links | weekly and on demand | lychee checks every link of the Markdown files — bibliography sources and internal links (ADR-0022); exclusions in `.lycheeignore` |
| `external-prs.yml` | close | pull request opened by an outsider | closes it with a pointer to the issue forms (ADR-0026); never checks out its code |
| `story-automerge.yml` | merge | story pull request labeled `autonomous` | merges owner story pull requests into a feature branch (US, EN, SP, CHK, AUD), and into `develop` a feature validated from its `feat/val-` branch or an epic's `NAM`, `REV` or `RET` the Product Owner validated, already checked by the pre-push hook (ADR-0028, ADR-0038, ADR-0045, ADR-0050) |

The release pull request goes into `develop` (`release-please--branches--develop`, ADR-0067), where no CI runs; its commit reaches `main` inside the pull request from `develop` and is checked there. On a release pull request into `main` (the flow of ADR-0038, kept for a rollback), every job of `ci.yml` and `codeql.yml` is skipped by its `if:`; GitHub reports a skipped job as a success, so the required checks pass without running (`REF-GH-REQUIRED-CHECKS`). Required workflows are never filtered by branch or path at the job's place: a workflow skipped that way stays "Pending" and blocks the merge.

Secrets are checked three times: `secretlint` on staged files before each commit, `secretlint` in `check:all` (pre-push hook and CI), and GitHub's secret scanning with push protection (repository setting).

Code hygiene beyond lint is covered by lint rules: no `console`, no `debugger`, no `TODO`/`FIXME` comments, no unused code, no disabled rule without justification (see [lint](../conventions/lint.md)).

## Repository settings (done once, by the owner)

All under the repository page → **Settings**.

1. **General → Pull Requests**: check **Allow merge commits** (features into `develop`, `develop` into `main`, ADR-0038) and **Allow rebase merging** (stories into features); uncheck _Allow squash merging_; check **Automatically delete head branches**, **Always suggest updating pull request branches** and **Allow auto-merge** (autonomous runs). **Features → Pull requests**: _Collaborators only_ (ADR-0026).
2. **Actions → General → Workflow permissions**: keep _Read repository contents and packages permissions_ (each workflow declares its own); check **Allow GitHub Actions to create and approve pull requests** (release-please).
3. **Pages → Build and deployment → Source**: **GitHub Actions** (the `docs` job deploys the site).
4. **Advanced Security**: enable **Dependency graph**, **Dependabot alerts**, **Dependabot security updates**, **Secret Protection** with **Push protection**. Code scanning: leave the CodeQL _Default setup_ **off** — the `codeql.yml` workflow is the advanced setup, both would conflict.
5. **Default branch**: `main` (release-please, CodeQL's baseline, the public page).
6. **Rules → Rulesets** (ADR-0038):

   | Ruleset | Target | Bypass | Rules |
   | --- | --- | --- | --- |
   | `main` | default branch | none | restrict deletions, block force pushes, require a pull request (0 approval — one cannot approve one's own pull request; merge method **Merge** only), require status checks with _branches up to date_: `Format, lint, secrets, types, tests, audit, versions, docs, build`, `Commit messages and branch name`, `Dependency review`, `Analyze` |
   | `develop` | `refs/heads/develop` | none | restrict deletions, block force pushes; no pull request nor status check required (direct pushes of the agent, checked by the pre-push hook) |
   | `feature branches` | `refs/heads/feature/*` | Repository admin (rebase on `develop` before the feature pull request) | require linear history, require a pull request (0 approval, rebase only); no status check required |

Dependabot opens **security** update pull requests only (repository setting); they target `main`, the one exception to releases only (ADR-0038), and pass its full CI; `develop` then catches up. Version updates are the agent's job, after reading the release notes ([versions and security](./versions-and-security.md), ADR-0038).
