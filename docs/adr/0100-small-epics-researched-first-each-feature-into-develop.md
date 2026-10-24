# ADR-0100 — Small epics researched first, each feature into `develop`, a history of bubbles

**Status**: Accepted (Product Owner, 2026-10-24, at REV-008, the review of E02). Supersedes ADR-0086's feature frame: the `VAL` per feature, the feature into `develop` only once validated, and the stories of a feature started without the epic's questions asked; ADR-0086's other decisions stand (one story at a time, a merge commit for every merge, the release by a button).

## Context

E02 ran from 2026-10-22 to 2026-10-24 over thirteen features, F35 to F47, and its review listed what went wrong (`docs/process/retrospectives/E02-retrospective.md`):

- The epic held several subjects (shapes, selection, moves, history, zoom, corrections), the agent having judged at its start that they fitted together; each feature opened with its own spike, so that questions reached the Product Owner feature by feature, and a feature's size was learned while it was built.
- The features were stacked, each branch made from the one before, none merged into `develop` until the end: the Product Owner tested on branches that changed under them, the main checkout left behind, and `develop` → `main` waited for the whole epic.
- The `VAL` per feature was dropped on 2026-10-23; the validation came at the end, three tries of the playground, each bringing a feature of corrections (F43 to F47).
- The main thread committed its docs straight on the feature branches, so that the history showed flat runs of commits between merges, where the Product Owner reads one bubble per piece of work (ADR-0091).

## Decision

1. **An epic holds one subject**, small if it must be: a theme the Product Owner can name in a few words. Too many features, or two subjects found at its refinement, make two epics, the second after the first.
2. **An epic starts with its whole research**: its refinement spike, from `develop`, reads every source, the code each feature will touch, and asks every question for the Product Owner at once, so that each feature's stories and tasks, and their number, are known before the first line of code. A feature then starts with its stories, without a spike of its own, unless a question nobody foresaw comes up.
3. **Each feature goes into `develop` when its audit ends**, by its pull request and a merge commit, before the next feature starts from `develop`; the main checkout stays on `develop`, brought up to date after each merge, so that `npm run dev` always shows the latest work.
4. **An audit ends every feature**; there is no `VAL` per feature. At each return the Product Owner tries what changed on `develop`, only what changed, and `develop` goes into `main` at each return, an epic finished or not: the features done are released.
5. **Every commit reaches a long-lived branch through a branch of its own, merged by a merge commit**, the main thread's docs included: a journal entry, a log row or a feature's opening is a small branch with its own merge, never a commit on `develop` or a feature branch, so that the history is bubbles only.
6. **The rigour stays this project's** (the Product Owner, 2026-10-24): one export per file, every function's TSDoc with its `@kind` and its verified `@see`, tests first, an audit per feature, because the editor is meant for companies that ask why each choice was made, and its docs will later trace every action through the functions it calls. Levels of rigour by risk, for other projects, go to the playbook.
7. **An estimate is read from the measures**, by kind of story (`target/release/metrics`, the run's agents), never guessed.
8. **An advisory in a development dependency is the agent's to settle**, without asking: the risk read for this project (what the vulnerable code does, whether the project reaches it), then the fix taken even before its 7 days when the risk is real and the release is the package's own, its notes read, the decision written in `docs/tooling/versions-and-security.md`.

## Reasons

- Questions asked once, at the epic's start, let the run go on without the Product Owner; a feature's size known beforehand keeps it within 2 to 4 stories (ADR-0084) without splitting it while it is built.
- A feature in `develop` at its end gives the Product Owner one place to test and lets each return release what is done.
- Bubbles show what each piece of work was; a flat run hides it.

## Consequences

- `AGENTS.md` (Work), the `run` and `spike` skills, the worker brief and `docs/process/agile.md` say so; `tests/backlog.test.ts` no longer asks for a `VAL` per feature.
- The epics not started are regrouped into small themed epics before the next one starts (REV-008's decision 1), their refinement each a spike with every question.
- The two permission rules `Bash(git switch:*)` and `Bash(git push origin --delete:*)` let the agent keep the main checkout on `develop` and delete the merged branches.

## References

- ADR-0084 (features of 2 to 4 stories), ADR-0086 (one story at a time), ADR-0091 (a history of bubbles).
- `docs/process/retrospectives/E02-retrospective.md`.
