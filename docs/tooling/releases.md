# Releases

The project's own tool, `tools/release/` (`target/release/release`), drafts and checks the features' text of each pull request into `main`, and makes a release by a button (ADR-0086, superseding ADR-0083 and the release branch of ADR-0079): the version lives only in the tags `vx.y.z` and their GitHub releases, never in a commit, and `package.json` holds `0.0.0-development`. The tool replaced release-please on 2026-10-18 (EN-112) and became the button with EN-117.

| Step | Branch | Who | What happens |
| --- | --- | --- | --- |
| 1. the features' text | `develop`, at a review | the agent | `target/release/release notes` prints the draft of the text from the commits `origin/main` lacks, each line marked `[draft]`, and on its error output the commits behind each line; it writes no file and makes no branch. The agent rewrites each line ([below](#what-a-release-text-holds)) into the body's file, in its scratch folder, its evidence and the attribution after a line `---`, and runs `target/release/release check <file>` until it passes |
| 2. the review's pull request | `develop` → `main` | the agent opens it, the Product Owner merges | its body that file (`gh pr create --body-file <file>`); the CI runs; a merge commit ([Git workflow](./git-workflow.md)) |
| 3. a push to `main` | `main` | `release.yml`, `rust-cache` and `docs` jobs | the Rust caches saved, the site and the playground deployed to GitHub Pages; no tag, no release |
| 4. a release | `main` | the Product Owner, by the button (_Actions → Release → Run workflow_, on `main`), when they choose | `release publish` reads the last tag behind `main`'s head and the commits since, computes the version ([below](#the-version)), joins into the notes the bodies of the pull requests into `main` merged since that tag, each checked as `release check` does, and makes the GitHub release `vx.y.z` on `main`'s head, which GitHub tags, the test report of the last pull request's CI attached (ADR-0027); no commit, no number typed |

- The release text of a body ends at its first line `---`: what follows stays on the pull request. GitHub's `\r\n` are read as `\n`.
- The notes join the bodies group by group, in the order of Common Changelog, each group's breaking changes first, then the lines of the latest pull request first. A body's own notice, an upgrade guide, comes first; two bodies holding different notices are refused, a release holding one at most. The notice of no change is kept only when no body lists a change.
- `release publish` does nothing without a commit since the last tag: a second press, or a press with nothing merged, releases nothing. It refuses a body that breaks a rule, naming its pull request and its line: edit the body on GitHub, then press again.
- The button on a branch other than `main` releases nothing: the `release` job runs only for `workflow_dispatch` on `main`, the `docs` and `rust-cache` jobs only for a push.
- The workflow runs on `GITHUB_TOKEN` alone (`contents: write` to make the release, `actions: read` to download the report, `pull-requests: read` to read the bodies, `REF-GH-JOB-PERMISSIONS`): no personal token, no third-party action. The token `RELEASE_PLEASE_TOKEN` release-please needed is no longer read; **the Product Owner deletes its secret** (_Settings → Secrets and variables → Actions_) and the token itself in their GitHub account (_Settings → Developer settings → Fine-grained tokens_).
- The notes of 0.1.0 and 0.2.0 stay in their GitHub releases; `CHANGELOG.md` is gone.
- The library stays below `1.0.0` until the Product Owner calls it stable. It is not published to npm yet; once it is, the version is written into the package at publish time, never committed (ADR-0086).

## The version

`release publish` computes it from the last tag and the commits since, the commits of the library and the playground alone counting (`REF-SEMVER`; [how the draft is made](#how-the-draft-is-made) says which):

| Commits since the last tag | Next version before 1.0.0 | After 1.0.0 |
| --- | --- | --- |
| a breaking change (`!`, `BREAKING CHANGE:`) | minor | major |
| a `feat` | minor | minor |
| anything else, or no commit of the library nor the playground: then its notes are the notice of no change | patch | patch |
| no commit since the last tag | no release: `release publish` does nothing | no release |

**Rolling back** to the release branch of ADR-0079: `tools/release/`, `release.yml`, `package.json`, `package-lock.json` and `CHANGELOG.md` from the commit before EN-117's. To release-please (ADR-0067): `release-please-config.json`, `.release-please-manifest.json`, the `release-pr`, `release-please` and `test-report` jobs of `release.yml`, the `release-please--` branch pattern and the commit-message exception of `tools/git-hooks/`, from the commit before EN-112's, and a personal token in the secret `RELEASE_PLEASE_TOKEN`.

## What a release text holds

The rules `release notes` drafts by and `release check` checks, from the sources on good release notes, read again through the library on 2026-10-18 ([EN-112](../backlog/stories/E17-F21-EN-112-release-tool-in-rust.md), [research note 0025](../research/notes/0025-f21-agent-tools-in-rust.md)). Where Keep a Changelog and Common Changelog differ, the text follows Common Changelog, the stricter: no `Unreleased` section, the text written at the review with its references; no `Deprecated` nor `Security` group, a deprecation listed under `Changed` (`REF-COMMON-CHANGELOG`, 6.2). A release's notes are the texts of its pull requests joined, so they follow the same rules.

| Rule | Source | Checked by `release check` |
| --- | --- | --- |
| Written for the people who use the library and the playground: the noteworthy difference, often across several commits, never the commit log | `REF-KEEP-A-CHANGELOG` (Commit log diffs), `REF-COMMON-CHANGELOG` (4.1) | — the draft's selection, then the agent |
| The version and the date are the GitHub release's, its title and its day, made by the button: the text holds no heading but its groups' | `REF-COMMON-CHANGELOG` (2.2), ADR-0086 | no heading other than `### …` |
| Changes grouped by impact, in the order `Changed`, `Added`, `Removed`, `Fixed`; deprecations, removals and breaking changes always listed | `REF-COMMON-CHANGELOG` (2.4), `REF-KEEP-A-CHANGELOG` (Types of changes, Ignoring deprecations) | the group names, their order, each once, a list under each |
| A breaking change first in its group, marked `**Breaking:**` | `REF-COMMON-CHANGELOG` (2.4.4) | no breaking line after another line of its group |
| Each change one line, in the imperative mood, self-describing without its heading, then its references in parentheses, each a Markdown link, a pull request rather than its commits when there is one | `REF-COMMON-CHANGELOG` (2.4.1, 2.4.2, 3.6) | one line, its references last; the mood is the agent's |
| Noise left out — dotfiles, development-only dependencies, formatting — refactorings and new documentation kept | `REF-COMMON-CHANGELOG` (3.2) | — the draft's selection |
| Related changes merged into one line, changes that undo each other left out | `REF-COMMON-CHANGELOG` (3.4, 3.5) | — the draft: one line per story, a revert and what it reverts left out |
| At most one notice, a single sentence, for a release without a change of its own or an upgrade guide | `REF-COMMON-CHANGELOG` (2.3) | one notice at most, in emphasis, before the groups, and only lists after it; `release publish` refuses two |
| Conventional Commits only draft the categories; each line is then rephrased: a draft generated, then edited | `REF-COMMON-CHANGELOG` (3.1, 4.2) | no line still marked `[draft]` |
| The version follows Semantic Versioning; before 1.0.0 a release raises the minor version for a feature or a breaking change | `REF-SEMVER` (Summary, FAQ) | — `release publish` computes it |

What release-please's 0.2.0 notes did against them, their 163 lines of changes, still in its GitHub release: one line per commit, 29 under Features and 96 under Documentation; a feature's commits beside its own merge commit; the tooling's commits among the library's; scopes in bold instead of words; a commit referenced where a pull request exists; the breaking change in a block of its own, far from its group. GitHub's generated notes list the merged pull requests by label (`REF-GH-RELEASE-NOTES`): here stories and features, without what they change for a user.

### How the draft is made

`release notes [<base>]` (`tools/release/`) reads the commits behind the head that its base lacks, `origin/main` by default, each with the pull request that brought it (GitHub's merge commit `Merge pull request #n` on the first-parent line, or a squash merge's `(#n)`); on `develop`, a feature's pull request into `develop`:

- **kept**: `feat`, `fix`, `perf`, `refactor` and `revert` in a scope of the library or the playground (`geometry`, `interaction`, `io`, `math`, `model`, `playground`, `render`, `routing`) or without a scope; **left out**: `docs`, `test`, `chore`, `ci`, `build`, `style`, and the `tooling`, `ci`, `deps`, `docs`, `backlog` and `adr` scopes, and a revert with the commit it reverts when both are in the range;
- **one line per story**, read from the commit's `Refs:` footer, titled by the story when it is a `US` or an `EN`, else by its oldest commit's description; a story's removals on a line of their own, so that a removal is always listed; a commit of no story is a line alone;
- **its group**: `Removed` for a `feat` that starts with `remove` or `drop`, else `Added` when the line holds a `feat`, else `Fixed` when it holds a `fix`, else `Changed`; marked `**Breaking:**` when a commit has `!` or a `BREAKING CHANGE:` footer;
- **the order**: the groups, then the breaking lines first, then by feature, the latest first, then the latest change first;
- **its references**: its pull requests, else its commits, the best one alone beyond two (`REF-COMMON-CHANGELOG`, 2.4.2);
- **no change**: a range whose commits are all left out drafts only the notice `_No change to the library nor the playground: documentation and tooling only._`; a range without a commit drafts nothing and fails.

It then prints on its error output, for each line, its group, feature, story and commits, the text of a breaking change with them. The agent rewrites each `[draft]` line from them in the imperative mood, as a user reads it, merges or splits lines that say one change or two, and runs `target/release/release check <file>` until it passes.
