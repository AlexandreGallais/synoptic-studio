# ADR-0028 — Feature branches and autonomous runs: stories merge into their feature, the Product Owner merges the feature

**Status**: Superseded by ADR-0072 (2026-10-17, review of the ADRs, EN-094), which gathers how a run works today. Accepted (Product Owner, 2026-10-08), partly superseded by ADR-0038 (feature branches start from and merge into `develop`; no required checks on them) — amends `CLAUDE.md` and `docs/tooling/git-workflow.md` on two points: each feature has a short-lived integration branch, and story pull requests merge into it without the Product Owner.

## Context

Every story pull request waited for the Product Owner's merge into `main`, even when nothing in it needed their judgment: the checks (lint, types, 100 % coverage, properties, docs build), the sources and the independent auditor carry the technical acceptance. What needs the Product Owner is business judgment: trying what the user stories deliver, the feature validation, the epic review and retrospective, the business questions. The Product Owner wants to start a run in the evening and find the work done up to the next point that needs them.

Merging story pull requests straight into `main` without a person was rejected: nothing should reach `main`, and therefore a release, without the Product Owner's review (the agent's own safety classifier also refuses it). The Product Owner does not read the code; their review is of what they see: demos, test cards, audit report.

GitHub can merge a pull request automatically once the required checks pass (`REF-GH-AUTO-MERGE`), and rulesets can require those checks on branches matching a pattern (`REF-GH-RULESETS`).

## Decision

1. **One integration branch per feature**: `feature/f01-<topic>`, created from `main` when the feature starts (after its refinement), deleted when merged. It is the only exception to "no long-lived branch", and lives as long as one feature. Ruleset `feature branches`: the same five required checks as `main`, branch up to date, rebase only.
2. **Story branches** start from the up-to-date feature branch and their pull requests target it. One branch per story and one commit per task, unchanged.
3. **Autonomous run** (`/run`): when the Product Owner starts a run for a scope, the agent records it in the session journal, then chains the stories. Before each pull request, the `auditor` subagent reviews the story's diff against its criteria and sources. The pull request carries the label `autonomous`; the workflow `story-automerge.yml` enables auto-merge when the author is the owner, the base is a feature branch and the head is a `US`, `EN`, `SP`, `CHK` or `AUD` story. GitHub merges once every required check is green. The agent never merges (guard hook).
4. **Outside a run**, story pull requests into the feature branch wait for the Product Owner, as before.
5. **Dev stories and Product Owner stories**: enablers, spikes, checkpoints and audits are developers' work; each **user story** (`US`) also gets a **Product Owner test card** in its file (steps in the playground, expected result of each, words explained).
6. **Checkpoint** (`CHK`) in the middle of each feature: documentation and methods brought up to date, consistency checked, light evolvability check (ADR-0023), test cards collected.
7. **Validation** (`VAL`): the agent rebases the feature branch on `main`, starts the playground and the docs site of the feature branch **locally** (`npm run dev`, `npm run docs:dev`) and gives the Product Owner the links; the Product Owner runs the demo and the test cards and gives their account; the agent compares it with what the demo meant to show and records the gaps. Then the agent opens the **feature pull request** into `main`; the Product Owner merges it (rebase, so every task commit reaches `main`). That merge is the feature's acceptance and produces a release: one version per validated feature, the public site and playground updated with it.
8. **Stops**: a run stops at the first of: `VAL`, `REV` or `RET`; a business question; a research request; a guardrail of `CLAUDE.md`; a check still red after three attempts; an auditor finding that changes a requirement; the end of the scope. At each stop: a journal entry and a message to the Product Owner (`docs/conventions/writing.md`).
9. **After `VAL`**, the next feature is prepared with the Product Owner: interview, then research spike, then its feature branch.

## Consequences

- Work advances overnight; the Product Owner's time goes to what only they can judge, and nothing reaches `main` without them.
- Releases are per feature rather than per story; the public site shows the last validated feature, and intermediate progress is seen locally or in the run reports.
- An error can propagate over several stories inside a feature before the Product Owner sees it: the per-story auditor, the checkpoint, the audit and small stories limit it; the feature pull request can be held until fixed.
- A published preview of feature branches (shared link, other devices) is deferred to the first retrospective (improvement log).

## References

`REF-GH-AUTO-MERGE`, `REF-GH-RULESETS`, ADR-0020, ADR-0021, ADR-0023, ADR-0026, ADR-0027, `REF-CC-BEST-PRACTICES` (verification, independent reviewer)
