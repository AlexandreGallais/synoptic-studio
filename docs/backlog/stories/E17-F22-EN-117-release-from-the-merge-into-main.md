---
id: EN-117
epic: E17
feature: F22
title: The release by a button, from the tags
status: done
points: 3
---

# E17 · F22 · EN-117 — The release by a button, from the tags

ADR-0086, superseding ADR-0083: the Product Owner runs `release.yml` on `main` from GitHub's Actions tab; `release publish` computes the version from the last tag and the commits since, tags `main`'s head and makes the GitHub release, its notes the bodies of the pull requests into `main` merged since the last tag; no release branch, no commit, no `CHANGELOG.md`. A push to `main` deploys the site only.

## Acceptance criteria

- Given `release.yml` run by its button on `main`, when it ends, then the tag `vx.y.z` is on `main`'s head and the GitHub release holds the bodies of the pull requests into `main` merged since the last tag and the CI's test report, and no commit was made.
- Given a push to `main`, when `release.yml` runs, then it deploys the site and makes no tag nor release.
- Given a feature's pull request into `main`, when `release check` reads its body, then it passes only when it follows the rules of `docs/tooling/releases.md`.
- Given `package.json`, when the tests run, then its version is `0.0.0-development`; `CHANGELOG.md` no longer exists and nothing links to it.
- Given a release with no change of the library nor the playground, or no commit since the last tag, when `release publish` runs, then the first is a patch with its notice and the second does nothing.

## Tasks

- [x] T1 — The interface and its failing tests: `release notes` drafts a feature's text without a branch nor a file written, `release check` reads a body, `release publish` computes, tags and joins the notes (2 h)
- [x] T2 — The code that passes them; `release.yml` by its button, the push deploying only; `package.json` at `0.0.0-development`; `CHANGELOG.md` deleted; `docs/tooling/releases.md` on ADR-0086 (2 h)

One task = one commit, referenced as `EN-117.T1` in the commit footer.

## Decisions taken in the story

Within ADR-0086, for what it leaves open; written in `docs/tooling/releases.md`.

- **`release prepare` is gone**: `release notes [<base>]` drafts the features' text from the commits `origin/main` lacks, the draft on its output and the commits behind each line on its error output, writing no file and making no branch, as T1 asks. ADR-0086's consequences name both commands; one is kept, since both would draft the same text.
- **A body's release text ends at its first line `---`**: the evidence and the attribution of the pull request follow it and stay out of the notes; GitHub's `\r\n` are read as `\n`. `release check <file>` reads a body's file so, and `release publish` each body.
- **A text holds no heading**: the version and the date are the GitHub release's, made by the button; `### …` groups only.
- **The notes join the bodies group by group**, a group's breaking changes first, then the latest pull request's lines first; a body's own notice first, two different ones refused; the notice of no change only when no body lists a change.
- **The version counts the commits of the library and the playground only**, as the draft's selection does: a tooling commit marked `!` is a patch.
- **No release without a commit since the last tag**; a second press so does nothing. The release made by `gh release create` on `main`'s head, which GitHub tags: no tag by Git.
- **`release.yml`**: `release` runs only for `workflow_dispatch` on `main`, with `fetch-depth: 0` (`REF-GH-CHECKOUT`) and `pull-requests: read` (`REF-GH-JOB-PERMISSIONS`); `docs` and `rust-cache` only for a push; `rust-cache` runs `cargo fetch --locked` before its builds (EN-116's finding).

## Evidence

- `cargo test -p release`: 35 tests, among them the fake GitHub of `tests/publish.rs` and the binary on temporary repositories with a fake `gh` first on the `PATH`; nothing ran against GitHub.
- `target/release/release notes origin/main` in this worktree: 57 lines drafted, as REV-004 counted with `release prepare`, no file written.
- On a clone of the history in the scratch folder, `main` at its real head (`v0.2.0`, `339afd7c`) merging this branch as pull request #999, a fake `gh` giving the draft above rewritten as its body: `release publish` printed `released v0.3.0`, called `gh release create v0.3.0 --target <the merge> --title 0.3.0`, its notes the 57 lines without the attribution; no tag made in the clone.
- Each new test broken once: the break line, the breaking order, the notices' join, the notice of no change, the heading rule, the first line of the empty text's finding (a case added after it survived), `release_text` in `publish`, `Nothing` without a commit, the draft without a commit, the pull requests' numbers; `[F22.AC2]` of `release.yml`, `package.json` and `CHANGELOG.md` failed before T2.

## For the main thread

- The `done` commit's footer: `Rule-docs: none affected, tooling, its workflow and its docs only, no rule of docs/domain/`.
- Before the first press: the bodies the button reads are those of the pull requests into `main` merged after `v0.2.0` (`339afd7c`), so the first review's pull request alone; its body must pass `release check`.
- `tools/format`'s `[F19.AC5]` test proved the project's `.oxfmtrc.json` skips an ignored page with `CHANGELOG.md`: it failed once that entry went, and now writes its page under `docs/api/`, broken once by removing `docs/api` from the configuration. `tools/lint/src/markdown/mod.rs` still ignores `CHANGELOG.md` (`IGNORED`), and `tools/format/src/ignore.rs`'s unit tests use the name as an example: harmless with the file gone, left for a review (ADR-0086).
