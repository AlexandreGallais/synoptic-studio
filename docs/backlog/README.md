# Backlog

The project's work is planned as epics, features, stories and tasks (SAFe levels on top of Scrum). Since EN-154 the backlog lives in the agent's records, one SQLite database outside the repository, read and written by `target/release/knowledge` ([the records](../tooling/knowledge.md)): no story file, no index, no status commit. This page keeps what a person reads: the order of the epics and how work is planned.

## The order of the epics

The Product Owner's priorities, the library first and the editors' interface last (2026-10-19), the agent's knowledge and setup before E21 since they speed up every epic after them (2026-10-25).

| Order | Epic | Status |
| --- | --- | --- |
| 1 | E01 — Draw symbol shapes from numbers | done |
| 2 | E14 — Draw curves with handles, ellipses and round shapes | done |
| 3 | E15 — Place text in symbols | done |
| 4 | E17 — Fast tooling with Oxc and Rust | done |
| 5 | E02 — Select and move shapes on one drawing | done |
| 6 | E49 — Keep the agent's knowledge in a database, the docs light | in-progress |
| 7 | E50 — Share the agent's setup with every coding agent | draft |
| 8 | E21 — Resize shapes by their box | draft |
| 9 | E22 — Turn shapes about a chosen center | draft |
| 10 | E23 — Drag points and handles | draft |
| 11 | E24 — Compute in number fields | draft |
| 12 | E25 — Keep the history fast | draft |
| 13 | E26 — Snap, align and distribute shapes | draft |
| 14 | E27 — Group shapes and order their stacking | draft |
| 15 | E28 — Lay out shapes in boxes | draft |
| 16 | E29 — Combine shapes by boolean operations | draft |
| 17 | E30 — Rebuild shapes from their pieces | draft |
| 18 | E31 — Give shapes colors and strokes | draft |
| 19 | E32 — Refine the text's lines and options | draft |
| 20 | E33 — Choose fonts for synoptic views | draft |
| 21 | E34 — Build symbols with ports | draft |
| 22 | E35 — Animate parts of symbols | draft |
| 23 | E36 — Configure business types | draft |
| 24 | E37 — Publish business symbols and libraries | draft |
| 25 | E38 — Assemble synoptic views | draft |
| 26 | E39 — Connect symbols with orthogonal pipes | draft |
| 27 | E40 — Draw and reuse static drawings | draft |
| 28 | E41 — Open and save files from the web page | draft |
| 29 | E42 — Import SVG from other tools | draft |
| 30 | E43 — Tune settings and keys | draft |
| 31 | E44 — Edit the shape tree in a layers panel | draft |
| 32 | E45 — Design the editor's interface | draft |
| 33 | E46 — Run the editor as a desktop application | draft |
| 34 | E47 — Explain the project to newcomers | draft |
| 35 | E48 — Check each release | draft |

## How work is planned

- **An epic** is one subject; it opens with its refinement, a spike that asks the Product Owner every question at once and cuts its features. **A feature** is a capability the user can see and test, its numbered criteria its requirements (`F48.AC1`, ADR-0027), 2 to 4 stories then an audit. **A story** is one behavior, at most 3 tasks, one commit per task (ADR-0084).
- **No iterations**: items flow through `draft`, `ready`, `in-progress`, `done` (or `parked`, `dropped`) one at a time; the Product Owner's `/run` sets its scope `ready`, then tries what changed on `develop` at each return, before `develop` goes into `main`.
- **Where it is written**: every item's row in the records, `knowledge sql "SELECT … FROM stories WHERE feature = 'F48'"`; a status by `target/release/backlog-status <ID> <status>`; a story's branch made by `worktree-new <ID>` from its row, its rows marked merged by `story-finish` once GitHub merged it. The criteria of the done features are exported to `tests/data/criteria.json`, so that the CI checks each one has a test without the records.
