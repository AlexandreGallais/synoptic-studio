# Process

The method on one page is in [agile.md](../process/agile.md); this page keeps what made it work, to reuse.

| Level | Opens with | Closes with |
| --- | --- | --- |
| Epic | its refinement: the Product Owner's account, the whole documentary research, every question at once, the features cut and their stories counted (ADR-0100) | none: one subject in a few features; each is reviewed at the next return of the Product Owner, in a `REV` that also holds the naming pass and the method (ADR-0086) |
| Feature | its stories, known from its epic's refinement | `AUD` audit by independent agents, then the feature into `develop` (ADR-0100) |
| Story | `ready` set by the Product Owner, its own branch | one commit per task, `done` in the last commit, pull request into the feature |

What made it work:

- **No iterations**: cadences triggered by deliveries instead of sprints (ADR-0017, `REF-KANBAN-GUIDE`).
- **Write everything, then implement**: epics, features and stories exist before code; refinement adjusts them.
- **Status in three places checked by a test**: the item, its folder index, its parent's table.
- **Research before code, audit after**: an epic's refinement reads every source of its features; an audit re-derives, recomputes and re-reads.
- **Plain language at every demo**: the Product Owner tests what they understand, on one checklist page per epic, a line per option with a check box, tried in any order; the epic review reruns it whole.
- **Feature branches and autonomous runs**: stories chain on the feature's branch and merge themselves once checked locally; an audited feature joins `develop`; at each return the Product Owner tries what changed, merges `develop` into `main`, then releases by a button (ADR-0072, ADR-0086, ADR-0100).
- **Logs, emptied at each review**: ideas in the improvement log, critiques in the feedback log; the method's part of the review opens with one table of them all, decided row by row.
- **Every four to five epics, every rule questioned**: harden by default.
