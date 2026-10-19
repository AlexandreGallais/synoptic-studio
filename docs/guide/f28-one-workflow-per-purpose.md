# F28 — One workflow per purpose

GitHub's Actions tab now holds one workflow per purpose, each named for what it does and started by one condition, its trigger ([ADR-0088](../adr/0088-one-workflow-per-purpose-the-ci-into-main-the-only-blocking-one.md)). A tooling feature, validated by the agent (VAL-029).

## What you see in the Actions tab

| Workflow | Starts on | Does | Blocks |
| --- | --- | --- | --- |
| Checks into main (`ci.yml`) | a pull request into `main` | the five lanes of `check:all`, the commit messages, the dependency review | the merge into `main` |
| CodeQL (`codeql.yml`) | a pull request into `main`, a push to `main`, weekly | the security analysis of the TypeScript code | the merge into `main` (`Analyze`) |
| Site (`site.yml`) | a push to `main` | deploys the documentation site and the playground | nothing |
| Release (`release.yml`) | its button only | computes the version, tags `main`, makes the release | nothing |
| Rust caches (`rust-caches.yml`) | a push to `develop` or `main` | saves the Rust build and tools for the next runs | nothing |
| Story auto-merge, External pull requests, Links | as before | their purpose reviewed in F34, on the Product Owner's answers | nothing |

## What changed

- The release no longer shares a file with the site: merging a review into `main` deploys the site and makes no version; `release publish` refuses any ref but `refs/heads/main`.
- No job is chosen by an `if:` on the event or the branch: `[F28.AC1]` reads every workflow, every form of such a condition caught.
- `ci.yml` runs only what each lane needs: the test lane installs `cargo-deny` alone, the fetches only where `cargo deny` runs, no build step twice (`[F28.AC3]`).

## Evidence

| Criterion | Evidence | Result |
| --- | --- | --- |
| 1. one purpose per file, its trigger its only condition | `[F28.AC1]` in `tests/workflows.test.ts`; the file names, `name:` and first comments read on 2026-10-18 and agreeing (`ci.yml` renamed _Checks into main_ for it) | passed |
| 2. the site, the release and the caches apart | `[F28.AC2]` (`tests/ci-caches.test.ts`, `tests/release-flow.test.ts`, `f28_ac2_…` in `tools/release`) | passed |
| 3. each step of the CI into `main` needed, its times | `[F28.AC3]`; the times before EN-133 in `docs/tooling/ci.md` | passed; the times after measured at the next review's CI into `main` |
