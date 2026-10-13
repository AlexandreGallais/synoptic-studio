# Journal

What each working session dealt with, in the agent's words: the Product Owner's requests in neutral words, decisions, where they were recorded. Newest first. Written by the agent at the end of each session or before a long pause (ADR-0024); searched with `grep` (tags written `#tag`).

## 2026-10-13 — Private records, a pass over the tooling, F11 refined

- `#tooling` ADR-0041: the metrics, transcripts and source library hold raw messages and local paths; they are saved by `npm run records:save` into a private repository beside this one, with a private GitHub remote. A submodule would have put its URL and a moving commit in the public tree; an ignored folder inside would have to be skipped by every tool.
- `#tooling` A pass over the hooks and the docs: the Bash guard also denies `git commit -n`, `HUSKY=0`, `core.hooksPath`, `+branch` pushes and `git -C` forms; the session-start hook sees a feature started by a story in progress and removes stale running-call files; the `autonomous` label's description matches ADR-0038.
- `#docs` The tag `archive/f10-round-shape` was deleted at the restart: note 0010 now points to pull request #139 (`refs/pull/139/head`), also kept as a Git bundle in the private records.
- `#backlog` F11 refined with the Product Owner: `contourToPathData` leaves the public API (0.2.0), the folder limit comes from SP-010, VAL-010 is a page of figures without a guided test, curves stay out of F11 ([feature](../backlog/features/E14-F11-code-base-ready-for-curves.md)). Stories stay `draft` until the Product Owner sets them `ready`.

## 2026-10-13 — RET-001: the retrospective of E01

- `#process` The retrospective opened with one table of every logged item; the Product Owner decided row by row, then asked for one long working session on everything decided ([record](./retrospectives/E01-retrospective.md)).
- `#tooling` ADR-0038 accepted: `develop`, checks run locally, one CI into `main`, merge commits for features and releases, a 7-day cooldown, Dependabot version updates dropped.
- `#tooling` Metrics hooks, a status line of three lines, `npm run metrics` and `npm run metrics:session`; a local library of sources behind an MCP server (ADR-0039); knip, a property fitness function, a playground coverage floor; `package.json` and the configuration files tidied.
- `#agent` `AGENTS.md` for any agent, imported by `CLAUDE.md`; expert agents `math-expert`, `verifier`, `functional-analyst`, `doc-editor`; skills updated.
- `#docs` Writing rules for scanning readers; the Product Owner's words removed from every page; Mermaid diagrams; the method on one page; the playbook rewritten.
- `#playground` A floating toolbar, grouped panels, a pipeline column, a reset; a short guided test per feature, the full recheck with `?guide=epic`.
- `#backlog` F11, a code base ready for curves, first in E14.
- `#git` The history restarted as one commit at 0.1.0; the earlier commits are summarized in [history](./history.md).
- `#git` GitHub reset: old releases and tags deleted, `v0.1.0` released, `develop` created, rulesets for `main` (merge commits, the CI's four checks), `develop` (no deletion, no force push) and `feature/*`. Deleting the old workflow runs and deployments waits for the Product Owner: a first loop failed silently (see the tooling pitfalls).
- `#tooling` The status line counts the current turn, not the terminal; a call over 5 minutes shows in red; `scripts/watch-task.ts` watches a background task and reports a stall; transcripts kept 10 years (`cleanupPeriodDays`).
- `#tooling` Every session report saved and compared in `npm run metrics`; library uses counted; diagram labels kept short enough to fit their boxes.

## 2026-10-07 to 2026-10-12 — E01: shapes from numbers

Built in that order, each feature refined with the Product Owner, researched, implemented in stories, audited and validated in the playground:

| Feature | What it delivered                                            | Key decisions                |
| ------- | ------------------------------------------------------------ | ---------------------------- |
| F01     | a rectangle with a corner radius, reduced to fit             | ADR-0007                     |
| F02     | a regular polygon fitted in its box                          | `DERIV-regular-polygon-fit`  |
| F07     | the polygon anchored in its box, nine places                 | `docs/domain/shapes.md`      |
| F03     | a radius per corner, Points mode, the list of points         | ADR-0029                     |
| F09     | the rectangle as a free shape, stretched without loss        | ADR-0032                     |
| F08     | points moved, added and deleted; corners as point and radius | ADR-0031, ADR-0033, ADR-0034 |
| F04     | any angle, a position for every shape, two boxes             | ADR-0035                     |
| F06     | a polygon stretched to its box, rule C                       | ADR-0037                     |

F10 (round shape) moved to E14 when the ellipse proved needed; F05 (text) to E15. The review (REV-001) confirmed the closure criteria and E14 as the next epic.
