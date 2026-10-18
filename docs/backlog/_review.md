---
id: REV-000
epic: E00
title: Review of YYYY-MM-DD, F00 to F00
status: draft
points: 3
---

# E00 · REV-000 — Review of YYYY-MM-DD, F00 to F00

Held at a return of the Product Owner, its branch from `develop` (ADR-0086, `/review` and `/retro` skills): everything done since the last merge into `main`, reviewed together. `epic` names the epic of the first feature it reviews.

## Acceptance criteria

- Given the features finished since the last merge into `main`, when the review is held, then the Product Owner has read each feature's text for the release and tried what it left in the playground, each remark recorded in neutral words and turned into a backlog item or a domain update.
- Given the code changed since the last merge into `main`, when the naming pass ends, then every calculation a field names is a call to a function of that name with its `@see`, the golden results unchanged bit for bit (ADR-0049).
- Given the tool incident cards and the improvement and feedback logs, when the review ends, then each card is fixed or decided, each row decided with the Product Owner, the method's changes written as ADRs, rules or stories, and the docs brought up to date.

## Tasks

One task = one commit, referenced as `REV-000.Tn`.

- [ ] T1 — Naming pass over the code changed since the last merge into `main`; golden results compared (1 h)
- [ ] T2 — Session with the Product Owner: the features, the logs, the cards, the method; feedback and decisions recorded (1 h)
- [ ] T3 — The cards fixed, the decisions written, the docs brought up to date (2 h)
