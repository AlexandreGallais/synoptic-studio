# Releases

release-please (`.github/workflows/release.yml`) writes the versions and the changelog from the Conventional Commits. Its pull request lives on `develop`, and `main` only receives the release and tags it, so that `main` never holds a commit `develop` lacks (ADR-0067).

| Step | Branch | Who | What happens |
| --- | --- | --- | --- |
| 1. a push to `develop` | `develop` | release-please (`release-pr` job) | opens or updates the release pull request `chore(develop): release x.y.z`: the next version in `package.json`, the new section of `CHANGELOG.md`; no release |
| 2. a release asked | `develop` | the Product Owner | merges the release pull request into `develop`; no new one opens until it is tagged |
| 3. the release | `main` | the agent opens, the Product Owner merges | `develop` → `main`, CI green, merge commit ([Git workflow](./git-workflow.md)) |
| 4. a push to `main` | `main` | release-please (`release-please` job) | finds the release pull request merged into `develop`, tags its merge commit `vx.y.z` and publishes the GitHub release |
| 5. after the release | `main` | the same workflow | attaches the test report of the `develop` → `main` CI (ADR-0027), publishes the site and the playground to GitHub Pages |
| 6. catch-up | `develop` | the agent | `develop` fast-forwards to `main` |

- Docs and tooling wait in `develop` and travel with the next release.
- The order matters: merged `develop` → `main` first, step 4 finds nothing to tag. Merge the release pull request into `develop`, then open a second `develop` → `main`. The site can also be published by hand: _Actions → Release → Run workflow_.
- release-please runs with the fine-grained token `RELEASE_PLEASE_TOKEN` (Contents, Pull requests, Issues: read and write), because a pull request made with `GITHUB_TOKEN` triggers no other workflow. It **expires 2026-12-31**: renew it before.
- The library stays below `1.0.0` until the Product Owner calls it stable (`initial-version` `0.1.0`). It is not published to npm yet.

| Commits since the last release | Next version before 1.0.0 | After 1.0.0 |
| --- | --- | --- |
| only `fix` | patch | patch |
| at least one `feat` | minor | minor |
| `!` or `BREAKING CHANGE:` | minor (`bump-minor-pre-major`) | major |
| only `chore`, `ci`, `test`, `style` | no release | no release |

Changelog sections: Features, Bug fixes, Performance, Refactoring, Documentation, Build and dependencies (`release-please-config.json`).

**Rolling back** to ADR-0038's flow: in `release.yml`, remove the `release-pr` job and `develop` from the push branches, remove `target-branch` and `skip-github-pull-request` from the `release-please` job, and restore its step `gh pr merge --auto --merge`; close by hand a release pull request still open on `develop`.

## What a release entry holds

The rules `release prepare` drafts by and `release check` checks, from the sources on good release notes, read again through the library on 2026-10-18 ([EN-112](../backlog/stories/E17-F21-EN-112-release-tool-in-rust.md), [research note 0025](../research/notes/0025-f21-agent-tools-in-rust.md)). Where Keep a Changelog and Common Changelog differ, the entry follows Common Changelog, the stricter: no `Unreleased` section, the entry written at release time with its references; no `Deprecated` nor `Security` group, a deprecation listed under `Changed` (`REF-COMMON-CHANGELOG`, 6.2). The changelog's header names the format and Semantic Versioning, as Keep a Changelog asks (`REF-KEEP-A-CHANGELOG`, Guiding principles).

| Rule | Source | Checked by `release check` |
| --- | --- | --- |
| Written for the people who use the library and the playground: the noteworthy difference, often across several commits, never the commit log | `REF-KEEP-A-CHANGELOG` (Commit log diffs), `REF-COMMON-CHANGELOG` (4.1) | — the draft's selection, then the agent |
| One entry per version, the latest first, headed `## [x.y.z] - YYYY-MM-DD`, the date in ISO 8601, the version a reference link to its GitHub release, which holds the same content | `REF-KEEP-A-CHANGELOG` (Guiding principles, Confusing dates), `REF-COMMON-CHANGELOG` (2.1, 2.2) | the heading, the date, the link, the version above the previous entry's and equal to `package.json`'s |
| Changes grouped by impact, in the order `Changed`, `Added`, `Removed`, `Fixed`; deprecations, removals and breaking changes always listed | `REF-COMMON-CHANGELOG` (2.4), `REF-KEEP-A-CHANGELOG` (Types of changes, Ignoring deprecations) | the group names, their order, each once, a list under each |
| A breaking change first in its group, marked `**Breaking:**` | `REF-COMMON-CHANGELOG` (2.4.4) | no breaking line after another line of its group |
| Each change one line, in the imperative mood, self-describing without its heading, then its references in parentheses, each a Markdown link, a pull request rather than its commits when there is one | `REF-COMMON-CHANGELOG` (2.4.1, 2.4.2, 3.6) | one line, its references last; the mood is the agent's |
| Noise left out — dotfiles, development-only dependencies, formatting — refactorings and new documentation kept | `REF-COMMON-CHANGELOG` (3.2) | — the draft's selection |
| Related changes merged into one line, changes that undo each other left out | `REF-COMMON-CHANGELOG` (3.4, 3.5) | — the draft: one line per story, a revert and what it reverts left out |
| At most one notice, a single sentence, for a release without a change of its own or an upgrade guide | `REF-COMMON-CHANGELOG` (2.3) | one notice at most, before the groups, and only lists after it |
| Conventional Commits only draft the categories; each line is then rephrased: a draft generated, then edited | `REF-COMMON-CHANGELOG` (3.1, 4.2) | no line still marked `[draft]` |
| The version follows Semantic Versioning; before 1.0.0 a release raises the minor version for a feature or a breaking change | `REF-SEMVER` (Summary, FAQ) | — `release prepare` computes it |

What release-please's 0.2.0 entry did against them, its 163 lines of changes in `CHANGELOG.md`: one line per commit, 29 under Features and 96 under Documentation; a feature's commits beside its own merge commit; the tooling's commits among the library's; scopes in bold instead of words; a commit referenced where a pull request exists; the breaking change in a block of its own, far from its group. GitHub's generated notes list the merged pull requests by label (`REF-GH-RELEASE-NOTES`): here stories and features, without what they change for a user.

### How the draft is made

`release prepare` (`tools/release/`) reads the commits since the last tag `vx.y.z` behind the head, each with the pull request that brought it (GitHub's merge commit `Merge pull request #n` on the first-parent line, or a squash merge's `(#n)`):

- **kept**: `feat`, `fix`, `perf`, `refactor` and `revert` in a scope of the library or the playground (`geometry`, `interaction`, `io`, `math`, `model`, `playground`, `render`, `routing`) or without a scope; **left out**: `docs`, `test`, `chore`, `ci`, `build`, `style`, and the `tooling`, `ci`, `deps`, `docs`, `backlog` and `adr` scopes, and a revert with the commit it reverts when both are in the release;
- **one line per story**, read from the commit's `Refs:` footer, titled by the story when it is a `US` or an `EN`, else by its oldest commit's description; a story's removals on a line of their own, so that a removal is always listed; a commit of no story is a line alone;
- **its group**: `Removed` for a `feat` that starts with `remove` or `drop`, else `Added` when the line holds a `feat`, else `Fixed` when it holds a `fix`, else `Changed`; marked `**Breaking:**` when a commit has `!` or a `BREAKING CHANGE:` footer;
- **the order**: the groups, then the breaking lines first, then by feature, the latest first, then the latest change first;
- **its references**: its pull requests, else its commits, the best one alone beyond two (`REF-COMMON-CHANGELOG`, 2.4.2);
- **the version**: a breaking line raises the minor version before 1.0.0 and the major one after, a `feat` the minor one, anything else the patch; a release whose commits are all left out is a patch whose entry holds only the notice `_No change to the library nor the playground: documentation and tooling only._`.

It then prints, for each line, its group, feature, story and commits, the text of a breaking change with them. The agent rewrites each `[draft]` line from them in the imperative mood, as a user reads it, merges or splits lines that say one change or two, and runs `target/release/release check` until it passes.
