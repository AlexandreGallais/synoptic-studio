# ADR-0045 — `/run` is the go-ahead; validated features merge by themselves; releases when the Product Owner asks

**Status**: Accepted (Product Owner, 2026-10-13). Supersedes, in ADR-0028 and ADR-0038: the Product Owner setting stories `ready` before a run, the Product Owner merging each feature pull request into `develop`, and a release pull request at the end of every feature.

## Context

After F11 the Product Owner was asked to set stories `ready`, to merge the feature pull request, then to merge a release pull request — three steps that carry no decision of theirs once they have started a run and validated the feature. They want `/run` to mean "go on", and a release only when they choose, typically at the end of a day of work.

## Decision

1. **`/run` is the go-ahead.** The agent sets the run's feature `in-progress` and its stories `ready` itself, at the start of the run or once a spike has written them. It still asks the Product Owner every business question (refinement, a rule missing from `docs/domain/`) and stops at the validation.
2. **A validated feature merges by itself.** Once the Product Owner says the `VAL` is validated, the feature pull request from `feat/val-<nnn>-…` into `develop` gets the `autonomous` label and GitHub merges it with a merge commit (`story-automerge.yml`). The agent still never merges.
3. **A release when the Product Owner asks.** The pull request `develop` → `main` is opened only on their request, typically at the end of a working day; its CI runs then, and they merge it.
4. After every merge, the agent cleans the branches at once (`npm run branches:clean`).

## Consequences

- `AGENTS.md`, the run and story skills, `docs/backlog/README.md`, `docs/process/agile.md` and the session-state hook say so; the workflow accepts `feat/val-` branches into `develop`.
- `develop` may hold several validated features before a release; the release pull request lists them.
- An epic is still never set `ready` by the agent without a run of one of its features.

## References

ADR-0028, ADR-0038, `REF-GH-AUTO-MERGE`
