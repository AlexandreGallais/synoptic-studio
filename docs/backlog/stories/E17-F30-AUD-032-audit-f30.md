---
id: AUD-032
epic: E17
feature: F30
title: Audit F30
status: ready
points: 1
---

# E17 · F30 · AUD-032 — Audit F30

Audit F30 following [audit.md](../../conventions/audit.md) (ADR-0020), the `auditor` alone (ADR-0086).

## Acceptance criteria

- Given F30's criteria, when the audit ends, then each is traced to a test titled `[F30.ACn] …` or `f30_acn_…`, broken once on purpose and seen failing.

## Tasks

- [x] T1 — The findings, and the tests of F1, F2, F3, F5 and F6, seen failing (1 h)
- [x] T2 — The findings fixed, the criteria traced (1 h)
- [ ] T3 — ADR-0090: the `develop` CI beside `rust-caches.yml` (30 min)

One task = one commit, referenced as `AUD-032.T1`.

## Findings

Reviewed by the `auditor` alone (ADR-0086), each finding reproduced before it is recorded.

| Id | Check | Result | Evidence | Action |
| --- | --- | --- | --- | --- |
| F1 | E3 — `develop-ci.yml` saves the tools' cache after a failed `cargo install`, under the full key: a saved cache never changes (`REF-GH-CACHE`), so every later run of the `develop` CI and of the review's lanes hits the key, skips the install and fails on the missing tool, until the key changes | reproduced by reading: the save's condition is `!cancelled()` and the key missed, nothing of the install | `.github/workflows/develop-ci.yml`, the first save step | the install step named `install`, the tools saved only when its outcome is `success` (`REF-GH-CONTEXTS`); `[F30.AC1] saves in the develop CI the tools once installed, …` fails on the current file (T1) |
| F2 | E3 — the build's cache saved after a failed `npm ci` or `check:rust`, its `target/` without their compilations for the life of its `Cargo.lock` | reproduced by reading: a step after a failure is skipped (`REF-GH-EXPRESSIONS`), `rust:coverage` runs by `!cancelled()`, and the save follows | `.github/workflows/develop-ci.yml`, the second save step | the checks named `checks`, the build saved only when their outcome is `success`, which a step skipped after `npm ci`'s, a fetch's or the install's failure is not; a failing test still saves it; `ci.md` says why; the same test fails (T1) |
| F3 | E3 — a pull request into `develop` not mergeable within the five attempts keeps the auto-merge `--auto` enabled: GitHub merges it later as the `GITHUB_TOKEN`'s actor, whose push starts no workflow (`REF-GH-TOKEN-TRIGGERS`), and nothing dispatches the `develop` CI | reproduced by reading: the step exits 1 after the loop, the auto-merge left on (`REF-GH-CLI-MERGE`) | `.github/workflows/story-automerge.yml`, the merge step | into `develop`, the attempts run out, `gh pr merge --disable-auto` before `exit 1`, so that a person merges and that push starts the `develop` CI; into a feature branch the auto-merge is kept, no `develop` CI following it; `ci.md` says so; three `[F30.AC1]` tests with the stub `gh`, two failing (T1) |
| F4 | E3 — `ci.md` says the review's pull request restores "the caches saved by the last run of the `develop` CI": a saved key is never replaced, a later run hitting it saves nothing, and a run whose exact key `main` holds hits it there and writes nothing under `develop` | reproduced: "You cannot change the contents of an existing cache" (`REF-GH-CACHE`, cache action usage) | `docs/tooling/ci.md`, The develop CI and The Rust build and its caches | the text: the first run for a key, and the case of `main`'s key; `REF-GH-CACHE` gains the sentence; no code (T2) |
| F5 | A4 — two mutations of `develop-ci.yml` survive: a `concurrency` group with `cancel-in-progress`, and `continue-on-error: true` on `check:rust` | reproduced: each made again, the group before `env:`, every earlier test of `tests/ci-caches.test.ts` and `tests/workflows.test.ts` passed | the auditor's mutation copy; the mutations made again here | `[F30.AC1] lets no run of the develop CI cancel another, nor a failing check pass`: fails on each, the group before `env:` and before `permissions:` (T1) |
| F6 | A3 — `[F30.AC2]` leaves `ci.yml`'s lanes to `[F28.AC3]`, which checks that a lane has a step, not that it comes before the call; the cargo tools are a closed list, so a `cargo nextest` added to a script is never named | reproduced: the lanes skipped by `missingTools`; `toolsCalled("cargo nextest run")` returns nothing | `tests/workflows.test.ts` | `missingTools` reads each lane of a matrix as a run of its own, the steps its fields choose (`runsIn`), the scripts' loop given the lane's scripts; any `cargo <subcommand>` neither built into Cargo nor a component the toolchain pins is a tool to install (`REF-CARGO-COMMANDS`); the skip argument removed; tests on made-up lanes and on `cargo nextest`, failing (T1) |
| F7 | E1 — ADR-0088's consequence says F30 makes the Rust caches' workflow the `develop` CI; F30 kept `rust-caches.yml` for `main` and added `develop-ci.yml` | reproduced | `docs/adr/0088-…`, Consequences; `ls .github/workflows` | ADR-0090 records the split and its reason: one purpose per file, and no `if:` on the branch; the index's rows (T3) |
| F8 | E3 — stale: the F28 guide's row gives `rust-caches.yml` a push to `develop` or `main`; EN-136's first criterion lists the jobs without `develop-ci.yml` | reproduced | `docs/guide/f28-one-workflow-per-purpose.md:13`; EN-136, Acceptance criteria | the note for the F30 guide in VAL-031, which writes that page; EN-136's list completed (T2) |
| C1 | A1, A2 — F30 holds no formula: its code is two workflows, a step's script and the helpers of `tests/workflows.test.ts` | no finding | `.github/workflows/develop-ci.yml`, `.github/workflows/story-automerge.yml` | — |
| C2 | A4 — `isCargoTool`: `!components.includes(…)` made `components.includes(…)` | caught | five `[F30.AC2]` tests fail, the CI's own jobs among them | restored |
| C3 | A4 — `runsOf`: a lane's steps not chosen by `runsIn` | caught | `[F30.AC2] reads each lane of a matrix …` fails | restored |
| C4 | A4 — `ci.yml`: the lanes' `cargo-deny` install moved after the scripts' loop | caught: `ci.yml lanes (TypeScript tests): cargo-deny`, `… (Rust): cargo-deny`; `[F28.AC3]` passes | `[F30.AC2] installs or receives in each job …` and `[F30.AC2] runs in the history job …` fail | restored |
| C5 | A4 — `story-automerge.yml`: the `--disable-auto` line removed; the base's test made `true` | caught | the two `[F30.AC1]` tests of the disabled auto-merge fail; then `[F30.AC1] keeps the auto-merge of a story …` | restored |
| C6 | A4 — `develop-ci.yml`: the saves' conditions as before T2 | caught | `[F30.AC1] saves in the develop CI the tools once installed, …` and `[F30.AC1] runs the Rust checks …` fail (T1) | fixed by T2 |
| C7 | A5 — properties: F30 adds no `math` nor `geometry` function | no finding | `src/` unchanged by F30 | — |
| C8 | B — `REF-GH-CACHE`, `REF-GH-CONTEXTS` and `REF-GH-CLI-MERGE` read again through the library, the Cargo Book's command pages read for `REF-CARGO-COMMANDS` | done | cache action usage; the `steps` context; `merge.go`'s `--disable-auto` flag | the three rows extended, the new row added |
| C9 | C, D — no third-party code; `commandsOf` and the lanes' runs read a lane's scripts by one `scriptsRunBy` | no other finding | `tests/workflows.test.ts` | — |
| C10 | `[F30.AC1]` traced | caught | C5, C6 and F5's mutations | restored |
| C11 | `[F30.AC2]` traced | caught | C2, C3 and C4 | restored |
| C12 | F — `check:all` green on the head | run by the pre-push hook | the push of T3 | — |
