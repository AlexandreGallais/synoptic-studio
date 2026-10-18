---
id: AUD-024
epic: E17
feature: F22
title: Audit F22
status: ready
points: 1
---

# E17 · F22 · AUD-024 — Audit F22

Audit F22 in depth and the project in breadth, following [audit.md](../../conventions/audit.md) (ADR-0020), with the `/audit` skill.

## Acceptance criteria

- Given F22's criteria, when the audit ends, then each is traced to a test titled `f22_acN_…` or `[F22.ACn] …`, broken once on purpose and seen failing.
- Given the release tool, when audited, then a release made from a copy of the repository's history gives the version and the notes ADR-0086 describes.

## Tasks

One task = one commit, referenced as `AUD-024.Tn`.

- [ ] T1 — A to G. The auditor's findings verified, the findings table; the tests of F-03 and F-04, failing (1 h)
- [ ] T2 — `release publish` refuses notes that disagree with the commits and leaves out Dependabot's bodies, `docs/tooling/releases.md` with it (1 h)
- [ ] T3 — The other findings fixed, the criteria tagged, the mutation spot-checks, the release from a copy of the history, H (1 h)

## Findings

The `auditor` reviewed sections A to E alone, in the background (ADR-0086, decision 1); each finding below was reproduced before it was recorded.

| Id | Finding | Evidence | Result |
| --- | --- | --- | --- |
| F-01 | `/audit` step 2 starts four reviewers in parallel, and `CLAUDE.md` lists the `doc-editor` among the second opinions with no condition, against one agent at a time and specialists only where their independence pays | `.claude/skills/audit/SKILL.md:14`, `CLAUDE.md:15`, ADR-0086 decision 1 | reproduced; to fix in T3 |
| F-02 | an epic's refinement `SP`, its branch from `develop`, is refused by `story-automerge.yml`, which lets into `develop` only a feature branch or a `REV`: `story:finish` waits 3 minutes and fails | `.github/workflows/story-automerge.yml:33-35` (`^(feature/f[0-9]{2}-\|[a-z]+/rev-[0-9]{3}-)`), `.claude/skills/spike/SKILL.md:13` | reproduced; the docs only, in T3: the Product Owner merges an epic's `SP`, its auto-merge a question for their next review; a tool incident card |
| F-03 | the version comes from the commits and the notes from the bodies, and nothing compares them: a range of the tooling only with a body listing changes is published as a patch with those changes, a `feat(model)` range whose bodies hold only a notice as a minor version with no change, both against `docs/tooling/releases.md` (The version) | `tools/release/src/publish.rs:119-124`; the AC3 test fed the notice to both bodies, from the code's own constant, so it held whatever the commits | reproduced by `f22_ac3_refuses_notes_that_disagree_with_the_commits`, failing (T1); to fix in T2 |
| F-04 | a Dependabot security update merged into `main` (`docs/tooling/ci.md`) puts Dependabot's body on the first-parent line, which `release check` refuses: the button fails until it is edited | `docs/tooling/ci.md:58`, `tools/release/src/history.rs:59-83`, `tools/release/src/publish.rs:56-61` | reproduced by `f22_ac2_leaves_out_the_body_of_a_dependabot_pull_request` and `reads_the_head_branch_of_a_merge_its_owner_left_out`, failing (T1); to fix in T2 |
| F-05 | no test titled for AC1 nor AC5, and AC4's half on the docs untested: the backlog test tracing a done feature's criteria would fail at F22's `done` | `grep -rn -e f22_ac -e 'F22\.AC'` lists AC2, AC3, AC4 only | reproduced; to fix in T3 |
| F-06 | `/spike` starts the `functional-analyst` and the `math-expert` in parallel, by default | `.claude/skills/spike/SKILL.md:17` | reproduced; to fix in T3 |
| F-07 | EN-122 and EN-123 `done` with tasks unticked, their commits made | EN-122's T2 and T3, EN-123's T1 and T2; `ad1a1109`, `6914314d`, `8f99912d`, `506e432c` | reproduced; to fix in T3 |
| F-08 | AUD-024's criteria and task cite ADR-0083, superseded by ADR-0086 | this file, lines 17 and 23 before T1 | fixed in T1 |
| F-09 | ADR-0086's Consequences name `release prepare`, which EN-117 removed | `docs/adr/0086-one-story-at-a-time-into-develop-the-release-by-a-button.md:35`; `tools/release/src/main.rs` has `notes`, `check`, `publish` | reproduced; the ADR is accepted, so a note in its row of `docs/adr/README.md`, in T3 |
| F-10 | the old flow's words left: a release "once EN-117 lands", a release commit `chore: release x.y.z`, a "changelog", the epic retrospective and an epic's end for the method's point, ADR-0086's "decision 8" for a review | `docs/tooling/git-workflow.md:17,74`, `docs/tooling/README.md:12`, `docs/conventions/audit.md:54`, `docs/conventions/rule-catalogue.md:3`, `.github/workflows/story-automerge.yml:3` (decision 4 holds the review) | reproduced; to fix in T3 |
| F-11 | EN-122.T2's tests and code in one commit, twice, against tests first in their own commit | `2f662b57`, `ad1a1109` | reproduced; no action on a pushed history; a row for the next review's method point, under For the main thread |
