# Process

The method on one page is in [agile.md](../process/agile.md); this page keeps what made it work, to reuse.

| Level | Opens with | Closes with |
| --- | --- | --- |
| Epic | its refinement: the Product Owner's account, the documentary research, the features cut | none: a list of features; each is reviewed at the next return of the Product Owner, in a `REV` that also holds the naming pass and the method (ADR-0086) |
| Feature | refinement questionnaire, `SP` research spike for all its stories | `AUD` audit by independent agents, `VAL` demo page and a checklist page |
| Story | `ready` set by the Product Owner, its own branch | one commit per task, `done` in the last commit, pull request into the feature |

What made it work:

- **No iterations**: cadences triggered by deliveries instead of sprints (ADR-0017, `REF-KANBAN-GUIDE`).
- **Write everything, then implement**: epics, features and stories exist before code; refinement adjusts them.
- **Status in three places checked by a test**: the item, its folder index, its parent's table.
- **Research before code, audit after**: a spike reads every source of a feature; an audit re-derives, recomputes and re-reads.
- **Plain language at every demo**: the Product Owner tests what they understand, on one checklist page per epic, a line per option with a check box, tried in any order; the epic review reruns it whole.
- **Feature branches and autonomous runs**: stories chain on the feature's branch and merge themselves once checked locally and reviewed; the Product Owner validates the feature and merges it, then the release (ADR-0072, ADR-0086).
- **Logs, emptied at each review**: ideas in the improvement log, critiques in the feedback log; the method's part of the review opens with one table of them all, decided row by row.
- **Every four to five epics, every rule questioned**: harden by default.
