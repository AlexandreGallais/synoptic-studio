# 0026 — Sources of the tooling features of 2026-10-18: one workflow per purpose, a CI on `develop`

Spike SP-024, 2026-10-18, for F28 and the tooling features cut the same day (E17). Every page cited was read through the local library; the runs and pull requests were read with `gh` on the repository the same day. Nothing in `.github/workflows/` was changed: the stories do it.

## 1. A workflow after each merge into `develop`

### What happens today

Every feature branch and review enters `develop` by `story-automerge.yml`, whose merge runs `gh pr merge --auto --merge` with the built-in token. The eight pull requests merged into `develop` on 2026-10-18 (#360, #361, #368, #369, #375, #379, #383 and #387) were all merged by `app/github-actions`, and `gh run list --event push --branch develop` lists no run at all: `release.yml`'s `rust-cache` job, which EN-125 set on a push to `develop`, never ran. The CI of `develop` → `main` that followed (run 39145798246) missed the Rust tools' cache: `cargo install` took 2 min 47 s in the Rust lane and 2 min 57 s in the test lane.

GitHub states the cause: "events triggered by the `GITHUB_TOKEN` will not create a new workflow run", a push included, except `workflow_dispatch` and `repository_dispatch`, which "always create workflow runs" (`REF-GH-TOKEN-TRIGGERS`, stated again on the events page). The observation above is the second, independent source: GitHub's documentation is the only text on its own platform, and the repository's runs agree with it.

### The mechanisms, read

| Mechanism | What the docs say | For a CI on `develop` |
| --- | --- | --- |
| `push` to `develop` | no run when the push is the `GITHUB_TOKEN`'s (`REF-GH-TOKEN-TRIGGERS`) | runs only for a merge made by a person: an epic's refinement `SP` merged by the Product Owner |
| `pull_request` `closed`, merged | the same rule: `closed` is not among the activity types the token's events start (`REF-GH-TOKEN-TRIGGERS`) | never runs after the auto-merge |
| `workflow_run` after `Story auto-merge` | runs the default branch's file on the default branch's last commit, `GITHUB_REF` the default branch (`main` here); "regardless of the conclusion of the previous workflow"; three levels of chaining at most (`REF-GH-WORKFLOW-DISPATCH`); a run of this trigger resolving to the default branch restores its caches and cannot save them, the save failing as a warning only (`REF-GH-CACHE`) | set aside: it would test `main`'s code unless it checks `develop` out, could not save the caches the pull request into `main` needs, would start after every auto-merge run, a story's into its feature branch included, and choose by an `if:` on the triggering run, which the rule of one trigger per workflow forbids (ADR-0088) |
| `workflow_dispatch`, sent by the auto-merge once its merge is done | always creates a run, even from the `GITHUB_TOKEN` (`REF-GH-TOKEN-TRIGGERS`); `GITHUB_REF` is the branch dispatched (`REF-GH-WORKFLOW-DISPATCH`); the REST endpoint takes the workflow's file name and a `ref`, and accepts an installation token, which the `GITHUB_TOKEN` is, with `actions: write` (`REF-GH-REST-WORKFLOW-DISPATCH`); `gh workflow run <file> --ref develop` sends it (`REF-GH-CLI-WORKFLOW-RUN`); a `workflow_dispatch` run writes its branch's cache scope (`REF-GH-CACHE`), and the pull request `develop` → `main` restores its head branch's caches first (`REF-GH-CACHE`) | **chosen**: the run is on `develop`'s head, its caches saved under `develop`, the one the review's pull request reads |
| a personal access token or a GitHub App for the merge | the docs' way to start workflows from a workflow (`REF-GH-TOKEN-TRIGGERS`, Note) | set aside: a secret to keep and renew, and a personal token acts as the Product Owner's account |

So the `develop` CI has two triggers, `push: branches: [develop]` for a person's merge and `workflow_dispatch` for the auto-merge's, and no run is started twice: the auto-merge's push starts nothing. Two limits follow from the docs:

- `workflow_dispatch` "will only trigger a workflow run if the workflow file exists on the default branch" (`REF-GH-WORKFLOW-DISPATCH`): the new workflow runs from the first merge into `main` that brings it. F30's validation reads its first run after that merge.
- With `--auto`, `gh pr merge` merges at once a pull request already mergeable, and otherwise only enables the auto-merge (`REF-GH-CLI-MERGE`): the dispatch waits until the pull request reads `MERGED`, or it would test `develop` before the merge.

## 2. Each workflow's use

Runs counted on 2026-10-18 with `gh run list` (the last 200 of each workflow).

| Workflow | Started by | Use, measured | Proposal |
| --- | --- | --- | --- |
| `ci.yml` (`CI`) | pull request into `main`; by hand | the blocking check of the review; 7 min 59 s at run 39145798246, `TypeScript tests` and `Rust` the longest | **keep**, without its `workflow_dispatch` and the two `if: github.event_name == 'pull_request'` it needs (the `develop` CI gives the timing on caches it served for, EN-111); its steps cut to what its lanes call (§3) |
| `codeql.yml` (`CodeQL`) | pull request into `main`, push to `main`, weekly | the required `Analyze` check, 1 min 09 s; the push keeps the analysis a pull request is compared with (`REF-GH-CODEQL-TRIGGERS`) | **keep** as it is, one purpose (security analysis); CodeQL also analyses Rust and the workflows themselves (`REF-GH-CODEQL-TRIGGERS`): a question for the Product Owner (below) |
| `external-prs.yml` (`External pull requests`) | every pull request opened or reopened | 200 runs of 200 skipped: no outsider ever opened one; the setting _Collaborators only_ already keeps them out (`REF-GH-PR-ACCESS`), the workflow being ADR-0026's second barrier | **remove**, with ADR-0026's second barrier: a question for the Product Owner, since ADR-0026 decided it |
| `story-automerge.yml` (`Story auto-merge`) | every pull request opened, labelled, reopened or made ready | 173 runs merged, 25 skipped, 2 failed; two runs for each pull request opened with its label (`opened` and `labeled`) | **split** by base, the trigger's `branches` filter replacing the job's base condition and the script's `if [ "$BASE" = develop ]`: into a feature branch (stories), into `develop` (features and reviews, then the dispatch of §1); one run per pull request |
| `links.yml` (`Links`) | weekly; by hand | one run, by hand on 2026-10-15, failed on 2 dead links of `docs/references.md` among 826; the schedule runs the default branch's file, on `main` only since 0.3.0's merge | **keep**, non-blocking: the outside links rot without a change of ours, and `tests/links.test.ts` checks only the internal ones |
| `release.yml` (`Release`) | push to `develop` or `main`; by hand | three purposes under three conditions: the site (push to `main`, 4 min 42 s), the Rust caches (push, 6 min 11 s), the release (button on `main`) | **split** into the site's deployment (push to `main`), the release (button only, the branch checked by `release publish`), the Rust caches (push to `develop`, then F30's `develop` CI) |

What the rule (ADR-0088) leaves as conditions: a step's `if:` on its matrix lane (`ci.yml` installs the cargo tools only in the lanes that call them, `REF-GH-CONTEXTS`), and the auto-merge's guard on the pull request's label, author and draft state, which no trigger expresses; neither chooses by a branch or an event.

## 3. The blocking CI, measured

Run 39145798246 (#387's review, `develop` → `main`), each lane a job:

| Job | Total | Cargo tools | `npm ci` | Its scripts |
| --- | --- | --- | --- | --- |
| Lint and types | 2 min 33 s | not installed | 1 min 51 s | 29 s |
| Format, secrets and dependencies | 2 min 54 s | not installed | — | — |
| Docs, playground and build | 4 min 41 s | not installed | — | — |
| TypeScript tests | 7 min 44 s | 2 min 57 s (cache missed) | 2 min 13 s | 2 min 20 s |
| Rust | 7 min 34 s | 2 min 47 s (cache missed) | 2 min 13 s | 2 min 19 s |
| Dependency review | 12 s | — | — | — |
| Commit messages and branch name | 9 s, after the lanes | — | — | — |

- `npm ci` holds the release binaries' build: its `prepare` runs it, so the step `npm run rust:build`, kept "so that its log names every crate compiled", takes 0 s in every lane (EN-133 finds whether the build moves to its step or the step goes).
- The cargo tools' 2 min 47 s to 2 min 57 s is the missing `develop` cache of §1: F30 removes it, F28 does not.
- Every lane runs `npm ci` and builds the binaries; whether the Rust lane needs Node.js at all, and whether one build passed as an artifact (as the history job already gets `git-hooks`) costs less than five, are measured by EN-133 before any change.

## 4. Per feature

| Feature | Its stories' sources | Open |
| --- | --- | --- |
| F29, the local checks stable and fast | `.claude/rules/tooling.md` (a timing limit at least four times the test alone, EN-036, EN-054); the cards of 2026-10-18 (`[F18.AC19]` at 12.3 s against 5 s, the binaries of the main checkout after a merge on GitHub); RET-004's rows I51, I97, I106, I112 | which files count as a Rust change: EN-130 lists them from `Cargo.toml`'s workspace and the crates' dep-info, as `stale-binaries` does |
| F28, one workflow per purpose | §2 and §3; `REF-GH-REQUIRED-CHECKS` (a job skipped by its `if:` passes, a workflow filtered out blocks); `REF-GH-CACHE` (the site's push to `main` saves `main`'s caches) | the Product Owner's answers on `external-prs.yml` and CodeQL's languages |
| F30, a non-blocking CI on `develop` | §1; AUD-027's row (a lane's tests needing another lane's tools, #370) | none |
| F23, the path of a story | REV-006's findings; the cards C1, C3, C4, C6, and those of `worktree-new` from the main checkout and of an epic's `SP` | none |
| F31, the guard and the agent's commands | the card C2, AUD-023's cases; the cards of `replace` without an argument and of `npm run fix` on a checklist barrel | the rule that turns `export type *` into `export *`, found by EN-138's first test |
| F32, reports | the card C5 and RET-004's rows of EN-120 | none |
| F33, RET-004's checks | RET-004's rows of EN-121 | none |

EN-119's criterion on a feature branch pushed with lease after its rebase onto `develop` is dropped: ADR-0086 merges a feature into `develop` by a merge commit, so no feature is rebased (its decision 3 supersedes ADR-0084's).
