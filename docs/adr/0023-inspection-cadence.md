# ADR-0023 — Inspection cadence: demo at each feature, review and retrospective at each epic

**Status**: Accepted

## Context

The project is run in "Product Owner + coding agent" mode: Claude Code designs and implements, the Product Owner questions, tests and decides. The Product Owner is not a geometry specialist and may express a need imperfectly; the agent may build the wrong thing correctly. Without iterations (ADR-0017), nothing forces a regular look back. The project will grow: tools and patterns that are overkill today (mutation tool, API report, size budget, board) become necessary later. The Product Owner also wants every lesson kept for future projects.

Scrum inspects the product with stakeholders (Review) and the way of working (Retrospective) (`REF-SCRUM-GUIDE`). Kanban, without iterations, relies on regular cadences whose purpose matters more than their names (`REF-KANBAN-GUIDE`). Architecture goals can be checked continuously by fitness functions (`REF-FITNESS-FUNCTIONS`).

## Decision

Three cadences, triggered by delivery, not by the calendar:

| When                | Story               | Content                                                                                                                                                                                                                                              |
| ------------------- | ------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| continuously        | —                   | both sides write ideas, irritants and lessons in the [improvement log](../process/improvements.md)                                                                                                                                                   |
| end of each feature | `AUD` then `VAL`    | audit (ADR-0020) including a light evolvability check; the validation includes a **plain-language demo page** in the [guide](../guide/) and a guided test in the playground; the Product Owner's feedback is recorded, drift becomes stories         |
| end of each epic    | `REV` epic review   | an **epic report** in plain language (what was built, terms explained, pictures, limits), a guided user test of the whole epic, the Product Owner's feedback, backlog and domain adjustments                                                         |
| end of each epic    | `RET` retrospective | both sides: improvement log, what helped, what hurt; **evolvability review** (size, tools, patterns, fitness functions, "considered for later" list, agent configuration); **playbook** update for future projects; decisions become ADRs or stories |

`REV` and `RET` stories belong to the epic (no feature): files `E01-REV-001-….md`, checked by `backlog.test.ts` like the feature frame.

Backlog tool: the Markdown files stay the source of truth (versioned with the code, reviewed in pull requests, checked by tests, read by the agent without network). GitHub Projects (`REF-GH-PROJECTS`) would duplicate the state and need a synchronization; not adopted. A generated board page is a candidate for the first retrospective.

## Consequences

- The Product Owner sees and tests something concrete at the end of each feature, in words they master; misunderstandings surface early.
- The guide grows into user-facing explanations of the library; the playbook into a reusable method.
- Two more stories per epic; an epic cannot be `done` without its review and retrospective.

## References

`REF-SCRUM-GUIDE`, `REF-KANBAN-GUIDE`, `REF-FITNESS-FUNCTIONS`, `REF-GH-PROJECTS`, ADR-0017, ADR-0020, ADR-0021
