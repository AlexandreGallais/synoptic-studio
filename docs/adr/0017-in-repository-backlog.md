# ADR-0017 — Agile backlog kept in the repository

**Status**: Accepted

## Context

The Product Owner asked to split the project into epics, features, user stories and tasks, with a rigorous method but without Jira or Azure DevOps: everything stays in the repository. Everything is written first, then implemented.

## Decision

- Method: the four-level hierarchy Epic → Feature → Story → Task of the user's methodological note (SAFe levels on top of Scrum, as in Azure Boards' Agile process), with Scrum's rules (`REF-SCRUM-GUIDE`), SAFe story types (`REF-SAFE-STORY`), INVEST and vertical slices (`REF-HUMANIZING-SPLITTING`), Given / When / Then acceptance criteria.
- Storage: one Markdown file per epic, feature and story in `docs/backlog/`, with a flat front-matter block; tasks are a checklist inside their story. The backlog is part of the documentation site.
- Integrity: `backlog.test.ts` checks IDs, statuses and parent links — the checks a Scrum Master would run in Jira (orphan stories) are automatic.
- **No iterations**: the project is run at home, without sprints; items flow through their statuses, one story in progress at a time. Scrum's events reduce to refinement and acceptance.
- Traceability: commits cite backlog items in footers (`Refs:`, `Closes:`).
- Roles: the user is the Product Owner; Claude Code acts as Scrum Master and developers, and never validates on the Product Owner's behalf.

## Consequences

- The plan is versioned with the code; a commit can close a story and its code together.
- No burndown chart, board view nor velocity; status is read from the files (a generated summary can be added later).
- Moving to a tool later stays possible: the files map one-to-one to work items.

## References

`REF-SCRUM-GUIDE`, `REF-SAFE-STORY`, `REF-HUMANIZING-SPLITTING`, [backlog method](../backlog/)
