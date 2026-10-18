# ADR-0084 — Small features, tests first, a linear history, and agents that work apart

**Status**: Accepted (Product Owner, 2026-10-18, RET-004, the retrospective's P3 to P7), from E02. Supersedes, in ADR-0072, the feature's checkpoint (`CHK`) required midway (The run, decision 6) and the branch table's line "`feature/f<nn>-<topic>` starts from the up-to-date `develop`" where it allowed a feature to start before the previous one merged; supersedes RET-003's rule "no force push on a feature branch, even with lease" for the one case of decision 3. The rest of ADR-0072 stands.

## Context

- E17's features held 17 to 28 stories: F19 took 28 stories, 10 h 37 of active time and 1,967 M tokens, and the Product Owner validated it after the whole of it. An agent "tended to try to do too much at once", and one feature at a time was "critical" (`REF-ANTHROPIC-HARNESSES`).
- ADR-0022 asks tests first, and ADR-0064 asks the worker to break each new test once to see it fail; nothing shows in the history that the tests came first.
- The pre-commit hook runs the type check with the lint (ADR-0071): a test importing a function that does not exist yet is refused at its commit.
- One Claude may write the tests and another the code that passes them (`REF-CC-BEST-PRACTICES`); a test file is a right-sized task for one agent, two agents editing one file overwrite each other, and the tokens grow with the agents (`REF-CC-AGENT-TEAMS`). Research is the strongest parallel use.
- A rebased branch is force-pushed; `--no-ff` keeps a feature visible as one group (`REF-GITFLOW`, `REF-GH-MERGE-METHODS`).

## Decision

1. **A feature holds 2 to 4 stories** (`US`, `EN`), its research done before its refinement, by a spike or by the retrospective that decided it; a checkpoint (`CHK`) only for a feature of 5 stories or more, as `tests/backlog.test.ts` checks from F22; its audit (`AUD`) and its validation (`VAL`) stay (ADR-0064, ADR-0072). A capability larger than that is split into features, each validated on its own.
2. **Tests first, in their own commit.** A story's `T1` commits the interface — the files, the function names, their signatures, `@kind`, `@see` and TSDoc, each body a placeholder — and its tests from the criteria, which fail; `T2` writes the bodies that make them pass. The tests seen failing at `T1` stand for ADR-0064's test broken once; the pre-push `check:all` runs on the story's head.
3. **One feature at a time, a linear history.** A feature branch starts from `develop` only once the previous feature merged; nothing is pushed to `develop` while it is open: the main thread's docs and log rows go to the feature branch. A feature merges into `develop` by a merge commit, one loop per feature on one line. If `develop` moved anyway, the main thread rebases the feature branch on it before the validation's pull request and pushes it with lease: the only force push on a feature branch. At the epic's end, `develop` → `main` and the release (ADR-0083).
4. **Two agents per story, a trial.** On two stories of E02's first library feature, one agent writes the tests from the criteria and the hand-computed values while another writes the code, both from the interface the story names at its refinement; the implementer reconciles, a disputed value settled by its derivation or stopped as a business question. Measured against two stories done by one worker under decision 2: tokens, active time, disagreements, the audit's findings. Three subagents at once stay the limit (ADR-0072).
5. **Researchers in parallel only on separate topics**, each writing its own note. **The main thread stays light**: merges, `done` commits and log moves go to the `clerk` agent.

## Reasons

- A small feature is validated the day it is built, and a mistake in its framing costs a few stories, not 28.
- A first commit of failing tests is evidence the tests can fail and came before the code, and the interface committed with them lets a second agent work from it.
- A history with one loop per feature reads as the list of features delivered, and no rebase is needed when nothing moves `develop` meanwhile.

## Consequences

- The guard allows a push with lease on a feature branch only through `git-sequences sync`'s rebase of that branch onto `develop`: a story of E02's first feature, [F23](../backlog/features/E17-F23-the-path-of-a-story.md).
- `AGENTS.md` (Work, Never), the run skill, the worker brief, `docs/process/agile.md` and the backlog's templates say it.
- The trial of decision 4 is measured at E02's retrospective, which keeps, changes or drops it.

## References

ADR-0022, ADR-0064, ADR-0071, ADR-0072, ADR-0083, `REF-ANTHROPIC-HARNESSES`, `REF-CC-BEST-PRACTICES`, `REF-CC-AGENT-TEAMS`, `REF-GITFLOW`, `REF-GH-MERGE-METHODS`, `docs/process/retrospectives/E17-retrospective.md`
