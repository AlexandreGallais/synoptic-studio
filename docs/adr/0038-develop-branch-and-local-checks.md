# ADR-0038 — A `develop` branch, checks run locally, one CI on the pull request into `main`

**Status**: Accepted (Product Owner, RET-001, 2026-10-13). Supersedes, in ADR-0028: the feature branch created from `main` and merged into `main` (points 1 and 7), the required checks on feature branches (point 1) and the auto-merge waiting for them (point 3); and the rule "no `develop` branch" of `CLAUDE.md` and `docs/tooling/git-workflow.md`. Points 1, 5 and 7, on the release pull request into `main`, superseded by ADR-0067 (2026-10-17): the release pull request is on `develop`.

## Context

During E01 every change ran the same checks several times on GitHub's servers, and the agent waited for them:

| Change                                      | Workflows run on GitHub                                                                                                         |
| ------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------- |
| a story pull request into a feature branch  | CI (about 2 min), CodeQL (about 1 min 10), story auto-merge; the agent polled them before the next story                        |
| any push to `main` (docs, tooling, feature) | CI again (about 1 min 50), CodeQL, Release (about 1 min 20)                                                                     |
| the release pull request that followed      | CI and CodeQL again, on a change of `CHANGELOG.md` and `package.json` only; then Release, which re-ran the tests for the report |

Averages of the last 100 successful runs. The pre-push hook already ran the full `check:all` locally before each of these pushes. Every documentation push to `main` also made a release nobody needed (improvement log, 2026-10-11).

Sources read:

- The git-flow model keeps `master` as "production-ready" (each merge into it a release) and `develop` as the integration branch from which feature branches start (`REF-GITFLOW`). Its author advises a simpler flow for continuous delivery, and keeps git-flow for "explicitly versioned software": this library's versions are the Product Owner's validated features.
- "Rebase and merge" "always updates the committer information and creates new commit SHAs" (`REF-GH-MERGE-METHODS`): `develop` merged into `main` that way would never again be an ancestor of `main`.
- Required checks: a job skipped by its `if:` condition reports "Success" and does not block; a workflow skipped by a branch or path filter stays "Pending" and blocks (`REF-GH-REQUIRED-CHECKS`).
- "Require branches to be up to date before merging": the branch must contain the base's latest commit before it merges (`REF-GH-PROTECTED-BRANCHES`); the merge queue gives the same guarantee on busy branches (`REF-GH-MERGE-QUEUE`), not needed with one writer.
- CodeQL scans pull requests by comparing them with the analysis of the target branch, which its default workflow refreshes on every push to the default branch and weekly (`REF-GH-CODEQL-TRIGGERS`).
- `gh pr merge --auto` merges at once when the pull request is already mergeable (`REF-GH-CLI-MERGE`).

## Decision

1. **`main` holds released versions only.** It receives pull requests only: `develop` → `main` (a release, opened by the agent, merged by the Product Owner), release-please's release pull request, which merges itself once its checks have reported, and Dependabot's security updates, which pass the full CI. No direct push: the `main` ruleset has no bypass.
2. **`develop` is the integration branch.** Tooling, documentation and backlog work is pushed to it directly by the agent; feature branches `feature/f<nn>-<topic>` start from it; at `VAL`, the feature pull request goes into `develop` and the Product Owner merges it. Several validated features may wait in `develop` for one release.
3. **Merges draw the history** (`REF-GITFLOW`, `REF-GH-MERGE-METHODS`):

   | Pull request        | Method                  | Why                                                                                            |
   | ------------------- | ----------------------- | ---------------------------------------------------------------------------------------------- |
   | story → feature     | rebase                  | a story is a few task commits; they line up on the feature branch                              |
   | feature → `develop` | merge commit            | the feature stays visible as one branch and one merge, git-flow's `--no-ff`                    |
   | `develop` → `main`  | merge commit (only one) | a rebase copies commits under new SHAs, and `develop` would no longer be an ancestor of `main` |
   | release → `main`    | merge commit            | the only method `main` allows                                                                  |

   `git log --first-parent main` then lists the releases, one merge each, and the graph shows each feature as a bridge on `develop`.

4. **Checks of work branches run locally**, never on GitHub: `pre-commit` (lint-staged), `commit-msg` (commitlint), `pre-push` (branch name, `npm run check:all`). No workflow runs on a story pull request, a feature pull request or a push to `develop`. The rulesets of `develop` and `feature/*` require no status check; story pull requests still exist (one per story, the record of its review) and, in an autonomous run, `story-automerge.yml` merges them as soon as they are labeled. The agent never waits for a CI on a work branch.
5. **One CI, on the pull request into `main`**: quality (`check:all`, build, test report), commit messages and branch name (one job), dependency review, CodeQL. On the release pull request (`release-please--branches--main`), every job is skipped by its `if:` and reported "Success" (`REF-GH-REQUIRED-CHECKS`): it only changes the version and the changelog.
6. **Nothing is re-checked after the merge.** A push to `main` runs release-please only, plus CodeQL, which keeps the default-branch analysis that pull requests are compared with (`REF-GH-CODEQL-TRIGGERS`). The test report attached to a GitHub release (ADR-0027) is the one produced by the CI of the merged pull request. The documentation site and the playground are still built and published at each release: a deployment, not a check.
7. **After a release**, the agent brings `develop` up to `main` (`git merge --ff-only origin/main`, else a merge), so that the release commit is in `develop` before the next work.
8. **Dependencies**: versions at least 7 days old (`npm run deps:outdated` with a cooldown, `REF-NCU-COOLDOWN`); the agent upgrades them itself, after reading their release notes (`docs/tooling/versions-and-security.md`). Dependabot version updates are removed: `check:all` already fails on an outdated package. Dependabot alerts and security updates stay on (repository settings).

## Consequences

- Per story: no GitHub workflow to wait for; the pre-push hook is the gate (`check:all`, about 50 s). Per release: one CI, about 2 minutes, then Release.
- Documentation and tooling changes no longer produce a release: they reach `main` with the next feature.
- History is no longer linear: `main` and `develop` carry merge commits, read with `git log --graph` or `--first-parent`.
- A commit that skipped the local hooks could reach `develop`; `--no-verify` stays forbidden (guard hook) and the CI into `main` catches it before a release.
- Local `check:all` is now the only check of a story: its duration matters more, and is measured (RET-001 metrics).

## References

`REF-GITFLOW`, `REF-GH-MERGE-METHODS`, `REF-GH-REQUIRED-CHECKS`, `REF-GH-PROTECTED-BRANCHES`, `REF-GH-RULESETS`, `REF-GH-MERGE-QUEUE`, `REF-GH-CODEQL-TRIGGERS`, `REF-GH-CLI-MERGE`, `REF-GH-AUTO-MERGE`, `REF-NCU-COOLDOWN`, `REF-DEPENDABOT-OPTIONS`, ADR-0027, ADR-0028
