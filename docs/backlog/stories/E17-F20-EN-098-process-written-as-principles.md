---
id: EN-098
epic: E17
feature: F20
title: The process pages written as principles
status: done
points: 2
---

# E17 · F20 · EN-098 — The process pages written as principles

Asked by the Product Owner on 2026-10-17: many rules an agent reads were written close to how a decision was first explained in conversation — its examples, its order, its wording — so an agent has to infer the principle and may doubt where it applies. Each rule is rewritten to state the idea in general terms: the principle, its reason, how to tell a case, what to do on conflict, with an example only where it removes a doubt. The agent is critical of how a decision was explained, gives a wider range of examples when they help, and leaves out what does not matter: shorter docs that serve the reader beat exhaustive ones. Split at F20's refinement into one story per family of pages, each measured in tokens before and after (2026-10-17). A later idea, not for these stories: keep superseded ADRs out of the agent's context.

This story covers the pages of `docs/process/` other than its logs.

## Acceptance criteria

- Given each page of the pages of `docs/process/` other than its logs, when the pass ends, then every rule reads as a principle with its reason and its limits; dated decision notes stay as the record of when and by whom, never as the rule's body.
- Given an accepted ADR whose wording follows a conversation too closely, when a page applies it, then the ADR is left as written and the page states the principle.
- Given a page or a section that guards nothing an agent needs, when the pass finds it, then it is cut, not rewritten.
- Given the pages before and after, when they are compared, then their length in tokens is written in the story, and no rule was lost: each old rule is traced to its new sentence.

## Tasks

One task = one commit, referenced as `EN-098.Tn`.

- [ ] T1 — Measure the pages in tokens and list their rules (1 h)
- [ ] T2 — Rewrite the pages as principles, each old rule traced (4 h)
- [ ] T3 — Measure again and record the figures (1 h)

## Measures

Tokens are each page's characters divided by **2.30 characters per token**, the ratio EN-097 measured the same day on Markdown of the same kind, rounded to the hundred; words from `wc -w`. The story first estimated characters / 4; the audit restated its figures at 2.30, so that they compare with EN-096's and EN-097's (AUD-022). Pages in scope: the rule pages of `docs/process/` (`README.md`, `agile.md`, `retrospectives/README.md`); the logs (`journal.md`, `improvements.md`, `feedback.md`, `history.md`) and the retrospective records stay as written.

| Page | Words before | Characters before | Tokens before (est.) |
| --- | --- | --- | --- |
| `README.md` | 867 | 5,404 | 2,300 |
| `agile.md` | 1,080 | 6,097 | 2,700 |
| `retrospectives/README.md` | 44 | 373 | 200 |

## Rules of the pages before

README: R1 cadences table (13 rows); R2 evolvability review (6 questions); R3 folders table; R4 V-model mapping.
agile: A1 flowchart of a feature and an epic; A2 four levels; A3 nine story types; A4 EN or US tip; A5 who does what; A6 cadences beyond the epic; A7 the agent's own method and tools.

### After

| Page | Words | Characters | Tokens (est.) | Change |
| --- | --- | --- | --- | --- |
| `README.md` | 836 | 5,067 | 2,200 | -3.6 % words, -6.2 % tokens |
| `agile.md` | 1,009 | 5,781 | 2,500 | -6.6 % words, -5.2 % tokens |
| `retrospectives/README.md` | 44 | 373 | 200 | unchanged |
| Total | 1,889 | 11,221 | 4,900 | -5.1 % words, -5.5 % tokens |

Total before: 1,991 words, 11,874 characters, 5,200 tokens; the changes in tokens are those in characters, the ratio being the same before and after. The saving is small because these pages were already short, tabular and mostly principle-shaped; the cuts below are what guarded nothing.

## Trace of the rules

| Old rule | New place | Change |
| --- | --- | --- |
| R1 cadences | README, Cadences | review and retrospective rows merged into one; the run's row shortened |
| R1 run stops (EN-086) | README, "A run stops only on one of four things" | stated as a principle: the four stops, the reason, a question never ends a run; meaning kept |
| R2 evolvability review | README, Evolvability review | numbered list became a table; same six questions |
| R3 folders | README, Pages | renamed, same rows, shorter descriptions |
| R4 V-model mapping | README, Coming from the V-model | test plan and test cases merged in one row; the "Function" column cut |
| A1 flowchart | agile.md | kept |
| A2 four levels | agile.md | "Example" column cut |
| A3 story types | agile.md | "Example" column cut; NAM row shortened; a test added to tell `US` from `EN` (who would notice) |
| A4 EN or US tip | agile.md | shortened |
| A5 who does what | agile.md | unchanged |
| A6 cadences beyond the epic | agile.md | the 2026-10-13 decision kept as a dated record after the rule |
| A7 agent's own method | agile.md | purpose stated first; same two rows |

Removed or merged, with reasons: the two "Example" columns (stories a reader can search; a cell per row); the "Function" column of the V-model table (repeated the first column); the review and retrospective rows (one owner, one moment). No rule's meaning changed; EN-086's run rule is kept. `retrospectives/README.md` is a bare index and was left as is.

## Model trial (EN-066, Sonnet 5.5)

- Duration: about 3 minutes of wall clock from first read to last commit, measured by `date`.
- Failed tool calls: 2 (`git:sync` refused with a dirty tree; a commit refused for the scope `process`, which `commitlint.config.ts` does not list).
