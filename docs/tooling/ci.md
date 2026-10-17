# Continuous integration

Every check runs before the code leaves the machine; GitHub checks once more what reaches `main` (ADR-0072). A CI run per story would cost minutes of waiting for checks the pre-push hook already ran.

- **Work branches** (stories, features, `develop`) are checked by the local Git hooks only ([Git workflow](./git-workflow.md#hooks-husky)): a push that fails them does not leave the machine. No workflow runs on them, and nobody waits for one.
- **A release**, the pull request from `develop` into `main`, runs the CI below; the ruleset of `main` lets it merge only when every check is green. Nothing is checked again after the merge.

| Workflow | Job | When | Does |
| --- | --- | --- | --- |
| `ci.yml` | quality | pull request into `main` | `npm run check:all` ([commands](./commands.md#the-lanes-of-the-full-check)), after installing cargo-llvm-cov and cargo-deny and fetching the RustSec database; then `npm run build`; keeps the JUnit test report |
| `ci.yml` | history | pull request into `main` | the branch name and commitlint on every commit of the pull request |
| `ci.yml` | dependencies | pull request into `main` | GitHub's dependency review: a new vulnerable dependency fails |
| `codeql.yml` | analyze | pull request into `main`, push to `main`, weekly | CodeQL's security and quality queries on the TypeScript; the push keeps the analysis pull requests are compared with (`REF-GH-CODEQL-TRIGGERS`) |
| `release.yml` | release-please | push to `main` or `develop` | the release pull request on `develop`, the tag and the GitHub release on `main`, the test report attached, the site published ([releases](./releases.md)) |
| `links.yml` | links | weekly and on demand | lychee checks every link of the Markdown files (ADR-0022); exclusions in `.lycheeignore` |
| `external-prs.yml` | close | a pull request opened by an outsider | closes it with a pointer to the issue forms (ADR-0026); never checks out its code |
| `story-automerge.yml` | merge | a pull request labelled `autonomous` | merges the owner's story pull requests into a feature branch, and into `develop` a feature validated from its `feat/val-` branch or an epic's `NAM`, `REV` or `RET` the Product Owner validated |

- **A required check is skipped by its job's `if:`, never by the workflow's branch or path filter**: GitHub reports a job skipped by its condition as a success, but a workflow filtered out stays "Pending" and blocks the merge (`REF-GH-REQUIRED-CHECKS`). So a release pull request into `main` (ADR-0038's flow, kept for a rollback) skips every job of `ci.yml` and `codeql.yml` and still merges; release-please's pull request into `develop` runs no CI, its commit checked inside the next pull request into `main`.
- **Secrets** are checked three times: secretlint on staged files at each commit, secretlint in `check:all`, and GitHub's secret scanning with push protection.
- **Code hygiene** beyond formatting is the lint's job — no `console`, `debugger`, `TODO`, unused code, nor rule disabled without its reason ([lint](../conventions/lint.md)) — not a check of its own.

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
