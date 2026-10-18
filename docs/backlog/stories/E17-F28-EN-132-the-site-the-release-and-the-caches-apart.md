---
id: EN-132
epic: E17
feature: F28
title: The site, the release and the Rust caches apart
status: ready
points: 3
---

# E17 · F28 · EN-132 — The site, the release and the Rust caches apart

`release.yml` holds three purposes, each job choosing itself by an `if:` on the event and the branch: the site on a push to `main` (4 min 42 s at run 39149464447), the Rust caches on a push to `develop` or `main` (6 min 11 s), the release by its button on `main` (research note 0026, §2). ADR-0088 gives each purpose its workflow, chosen by its trigger alone. A done feature's criterion changes only by a story citing it: this one rewrites the tests `[F21.AC4]`, `[F22.AC2]` and `[F24.AC3]` of `tests/ci-caches.test.ts` and `tests/release-flow.test.ts` that read `release.yml`'s three jobs.

## Acceptance criteria

- Given `.github/workflows/`, when `[F28.AC1]` reads each workflow, then no job nor step has an `if:` reading `github.event_name`, `github.ref`, `github.ref_name`, `github.base_ref`, `github.head_ref` or `github.event.pull_request.base.ref`, and each file's `name:` and first comment say its purpose; a failure names the file and the line. It fails today on `release.yml`; `ci.yml` and `story-automerge.yml` are listed in the test as exceptions, removed from the list by EN-133 and EN-134 when they change them, and a file listed that no longer breaks the rule fails too.
- Given a push to `main`, when it lands, then the site's workflow alone runs, builds and deploys the site and the playground, and saves the Rust build cache it compiled under `main` (`actions/cache`, `REF-GH-CACHE`), never the cargo tools' cache, whose key promises `cargo-llvm-cov` and `cargo-deny`, which it does not install.
- Given the release's workflow, when it is read, then `workflow_dispatch` is its only trigger and its job has no `if:`; given `release publish` run with `GITHUB_REF_NAME` other than `main`, then it exits 1 naming the branch and makes nothing, tested in `tools/release/tests/publish.rs`.
- Given a push to `develop` by a person, when it lands, then the Rust caches' workflow alone runs and saves both caches as the `rust-cache` job did, under the keys and paths `tests/ci-caches.test.ts` holds equal across the workflows.
- Given `docs/tooling/ci.md` and `docs/tooling/releases.md`, when read, then they name the three workflows and what starts each.

## Tasks

- [ ] T1 — `[F28.AC1]` in `tests/workflows.test.ts`, `[F28.AC2]` in the workflows' tests and `publish.rs`, the done criteria's tests rewritten, failing (1 h)
- [ ] T2 — `release.yml` split into the site, the release and the Rust caches, `release publish`'s branch check, the docs (2 h)

One task = one commit, referenced as `EN-132.T1` in the commit footer.
