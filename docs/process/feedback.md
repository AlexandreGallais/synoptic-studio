# Feedback on the agent's work

Every critique the Product Owner makes of how the agent worked, written as soon as it is said: what happened, what the agent should have done, and what now prevents it — in neutral words, never quoted. Requests and ideas go to the [improvement log](./improvements.md).

At each retrospective, the rows are grouped into the habits sheet of [Working with the agent](../playbook/agent.md#lessons-from-the-product-owner), then the log is emptied. E01's rows: [E01 retrospective](./retrospectives/E01-retrospective.md); E14's: [E14 retrospective](./retrospectives/E14-retrospective.md); E15’s: [E15 retrospective](./retrospectives/E15-retrospective.md); E17’s: [E17 retrospective](./retrospectives/E17-retrospective.md).

| Date | What happened | What the agent should have done | Now prevented by |
| --- | --- | --- | --- |
| 2026-10-24 | The time estimates ran two to three times the real time: E02 announced for an evening ended the one before, 1 h 30 given for stories of about 25 minutes (measured this run: a spike 17 to 32 minutes, a story 14 to 37, an audit's fixes 44 to 86) | estimate from the measures by kind, read from the run's agents, and say what the method adds | estimates from the measured medians by kind; a timer per story and task for the review |
| 2026-10-24 | 1 h 30 announced for US-085 and 1 h for US-087, where the change was to reuse several shapes' fields for every number field | say with the estimate that the method's weight (tests migrated, docs) is most of it, and build one shared mechanism reused by a list | the review's method part, on the weight of a small change |
| 2026-10-24 | E02's features F35 to F44 were stacked on each other's branches, none merged into `develop` as it ended | merge each feature into `develop` once it ends | the review on Git; for E02, the merge into `develop` once the Product Owner validates the epic, then a retrospective of the epic's mistakes before `main` |
| 2026-10-24 | A question on the sign of a value typed was put in terms the Product Owner could not follow, and a fraction in a field of integers was proposed as refused | ask by the plain rule: a value relative to the field, taken at its bound or wrapped, any number accepted and rounded | the rule written in `interaction.md` §2 (AUD-046) |
| 2026-10-24 | The Product Owner was given the whole list of checks again after each correction, F35 to F45 | ask to test only what a correction broke or changed | the test lists of F46 and after: the changed items alone |
| 2026-10-24 | F47 was opened for Space left held after Cmd, where the Product Owner had pressed Option: the agent never asked which key | ask for the exact gesture before opening a fix of a bug seen once | the E02 retrospective |
| 2026-10-24 | The epic held several subjects; the agent had judged at its start that they fitted together | one subject per epic, the epic split at its refinement when it holds two | ADR-0100, decision 1 |
