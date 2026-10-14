# Journal

What each working session dealt with, in the agent's words: the Product Owner's requests in neutral words, decisions, where they were recorded. Newest first. Written by the agent at the end of each session or before a long pause (ADR-0024); searched with `grep` (tags written `#tag`).

## 2026-10-14 — F10 and F13 built, ready for one validation

- `#run` F10 (round shape) into `feature/f10-round-shape`: SP-013 (#180, ADR-0048: four cubic sides, not elliptical arcs), EN-027 (#181), US-029 (#182), CHK-012 (#183), US-030 (#184), AUD-013 (#185); each reviewed (math expert, auditor or verifier), two playground bugs found by the verifier and fixed. VAL-012 prepared on `feat/val-012-validate-f10`: seven guided steps, the demo page.
- `#run` F13 (every calculation named), from F10's validation branch: SP-014 (#186, research note 0016), EN-028 (#187), EN-029 (#188), CHK-013 (#189), EN-030 (#190), AUD-014 (#191). 33 new named modules, the duplicates merged; a golden file of 400 results and the auditor's comparison of 70 970 results found no difference. VAL-013 prepared on `feat/val-013-validate-f13`, which holds both features.
- `#rule` The Product Owner asked that F13's work become a standing rule: every calculation or process that a field names (mathematics, geometry, physics, industry, the laboratory) is a named function with its source, checked at each story's review and by a naming pass over the whole code, in two passes, at the end of each epic. Recorded as ADR-0049, a `NAM` story type before `REV`, checked by `backlog.test.ts` (E01 and E14 exempt: F13 is E14's pass).
- `#questions` For the Product Owner: the decisions the agent took for F10 (cubic sides, point 1 at the top, the two conversion buttons' names); Q35 settled by the agent (nothing visible).

## 2026-10-14 — F12 validated; F10, then F13

- `#val` Second session of VAL-011: the Product Owner accepted every option as built — the reduced radius beside a curve (47.7 and 46.9), Q34 (the whole curve split at its middle), Q33 (a lens stays refused). F12 done; its pull request into `develop` merges by itself (ADR-0045).
- `#run` The Product Owner asked for F10 (round shape) next, then F13 (every calculation named), without further questions: F10's refinement was done in E01 and at the start of E14 (an ellipse in a long box; both kinds of points after conversion). Both are presented to the Product Owner together at their end; the order of E14's features changed accordingly.

## 2026-10-14 — F12 after its first validation, VAL-011 ready again

- `#val` First session of VAL-011: the Product Owner kept x, y on the page and the angle from the side, the three mirror modes (Q32), and asked that a corner beside a curve keep its rounding and that a polygon's point take handles. Routed to EN-026 (#174), SP-012 (#175, ADR-0047), US-028 (#176), US-027 (#177), AUD-012 (#178); each reviewed, US-027 twice.
- `#tooling` On `develop`: the status line waits for background tasks; the expert agents have the library's tools.
- `#val` VAL-011 prepared again on `feat/val-011-validate-f12`: nine steps, one per option, and the demo page as a list. Open for the Product Owner: Q33, Q34, the reduced radius as built (47.7 and 46.9, not 48.6 and 36.3).
- `#backlog` F13 (every calculation named, two passes over the code) drafted, to start once F12 is merged, as the Product Owner asked.

## 2026-10-13 — Run F12: stopped before VAL-011

- `#run` Merged into `feature/f12-curves-with-handles`: SP-011 (#164), EN-024 (#165), EN-025 (#166), US-022 (#167), US-023 (#168), CHK-011 (#169), US-024 (#170), US-025 (#171), US-026 (#172), AUD-011 (#173); each story reviewed by the auditor (and the verifier or math-expert where it added math) before its last commit.
- `#audit` AUD-011: the criteria F12.AC1–AC6 traced; the radii shown in the playground now match the drawing next to curves; `interaction.md` and Q28 follow the F12 rules; the handles section of the domain rewritten as tables; curve modules in `geometry/curve`.
- `#val` VAL-011 prepared on `feat/val-011-validate-f12`: 13 guided steps (`npm run dev`), tested end to end, and the demo page `docs/guide/f12-curves-with-handles.md`.
- `#questions` For the Product Owner at the validation: Q32 (which methods of typing a handle to keep), Q33 (a lens of points on one line), ADR-0046 "typed as integers" against the decimals of pasted and computed handles.

## 2026-10-13 — Run F12 authorized

- `#run` The Product Owner started a run without scope after F11: F12 (points on a curve with handles), first in E14's order, from its spike SP-011. `/run` is now the go-ahead (ADR-0045): the agent set F12 in progress and SP-011 ready. The release pull request of F11 was closed by the Product Owner: releases are opened when they ask.

## 2026-10-13 — F11 resumed: folders by domain, VAL-010 prepared

- `#adr` ADR-0043 supersedes ADR-0042: folders named after the business domains of `docs/domain/`, the same name in each layer, modules and folders never side by side; 15 modules stay a signal. The Product Owner refused the layout of EN-022 (math held a folder beside loose modules; folders cut to meet a count) and the agent's question on `playground/page/`, a choice of code to decide alone (feedback log).
- `#run` EN-023 (#161): `math/` and `geometry/` hold domain folders only, `rounded-corners/` and `radii/` became `corner-radius/`, `box/` split into `layout-box/` and `placement/`, the playground's wiring in `mount/`; rule `local/folder-layout` in the editor. Reviewed by the auditor and the verifier, their findings fixed.
- `#tooling` On `develop`: the private records moved into the project as `records/`, ignored by Git (ADR-0044); the status line's turn clock moves every second (the tokens still arrive when each answer ends).
- `#val` VAL-010 prepared on `feat/val-010-validate-f11`: one test per criterion, the demo page of figures, the closing commit ready; waiting for the Product Owner's reading.
- `#questions` `unicorn/prefer-iterator-concat` writes code Node.js 24 cannot run: turn it off? (improvement log).

## 2026-10-13 — Run F11: stopped before VAL-010

- `#run` Merged into `feature/f11-code-base-ready-for-curves`: SP-010 (#155), EN-020 (#156), EN-021 (#157), CHK-010 (#158), EN-022 (#159), AUD-010 (#160). Stopped before VAL-010, the Product Owner's validation.
- `#adr` ADR-0042: folders by business theme, 15 modules a signal with listed exceptions, a shape's type in its folder; the review of SP-010 found a name clash and a false premise, and the Product Owner chose the type in its folder before the merge.
- `#tests` A property for every math and geometry function in its own test file; shared playground actions and geometry fixtures; one path writer (`contourToPathData` removed, next version 0.2.0); one pass per corner; a folder test. 816 → 838 tests, no value changed (audit's scripted comparison).
- `#review` Each story reviewed by the auditor and the verifier (and the doc-editor at the audit): mutations that survived the suite were killed with new cases (concave fillet, 3-4-5 triangle, unequal edge factors, non-vacuous checks).
- `#tooling` A worktree linked to the main `node_modules` ran no Git hook: EN-020's push went out unchecked, `check:all` was run by hand (green); `docs/tooling/agent.md` now requires `npm ci`.
- `#questions` For the Product Owner: ADR-0042 rule 5 assumed math under 15 modules (it had 17, split by rule 3); is `playground/page/` a theme; the unused-imports rule reads JSDoc words as uses (improvement log).

## 2026-10-13 — Run F11 authorized

- `#run` The Product Owner started an autonomous run on F11 (`/run F11`), after agreeing to its plan: SP-010, EN-020, EN-021, CHK-010, EN-022 and AUD-010 set `ready`, F11 and E14 `in-progress`; a stop at the end of SP-010 for the folder layout choice; a stop before VAL-010.

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
