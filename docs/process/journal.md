# Journal

What each working session dealt with, in the agent's words: the Product Owner's requests in neutral words, decisions, where they were recorded. Newest first. Written by the agent at the end of each session or before a long pause (ADR-0024); searched with `grep` (tags written `#tag`).

## 2026-10-17 — F18 validated, F19 started

- `#run` The Product Owner answered the queued questions: the subagent limit set in the settings (EN-063); ADR-0067's release order, the tables moved to F19 (ADR-0068) and the refused face dropped (US-061) confirmed; EN-062's macOS-only font tests not accepted, so EN-067 made every font test run on every system; the font list to offer only fonts read whole and checked (US-062).
- `#run` They left F18's tool lines to the agent and declared the validation finished; F18 is done with EN-063, EN-067 and US-062 merged into its branch.
- `#run` They asked to move on to F19 at once, without waiting for them: F19's branch starts from F18's, SP-021 rebased on it.
- `#run` EN-068: the Rust tools form one Cargo workspace at the root (one `Cargo.lock`, one `target/`, Rust 1.99.1 pinned, warnings denied, `unsafe_code` forbidden); scripts and Git hooks run them with `cargo run --release -q -p <crate>` (0.03 to 0.05 s per call on a built workspace), `prepare` and `worktree:new` building them; `check:rust` is its own lane of `check:all` (9.4 s warm, 11.2 s before).

## 2026-10-17 — SP-021 answered, F19's run started

- `#refinement` SP-021, first round: `check:all` set to option B, about 70 to 90 s most of the time (Oxc instead of ESLint and Prettier, the docs build only when the docs change); none of the 8 rules that need types dropped without a stated reason; a coverage floor and an advisory check for the Rust tools; TypeScript 7 now; the library stays TypeScript, every tool around it moves to Rust; one clean-up story at the end of E17; the library's tests never depend on macOS, the tools use Rust's portable standard calls where one exists (note 0023).
- `#refinement` SP-021, second round: F19 holds the checks and F21 the agent tools, the `library` MCP server and E17's clean-up, last before NAM-002; the 6 rules without types rewritten in their strict form, nothing kept in ESLint; TypeScript 7 only, TypeDoc replaced by a Rust generator on Oxc; the project's rules in a Rust linter on Oxc's crates; three F20 stories drafted (push notification, progress page, `SubagentStop` hook); no cloud agents; Q40, Q41, Q42 recorded.
- `#refinement` SP-021, third and fourth rounds: oxlint is the one linter, `tsgolint` kept for the 55 rules that need types; every one of the 265 rules oxlint lacks triaged (232 kept, 33 dropped with their reasons) and the kept ones rewritten in the Rust linter within F19, ESLint leaving at its end; the dev packages reviewed in F21.
- `#run` The Product Owner started F19's run on 2026-10-17 and asked to move on to it at once, without waiting for them: SP-021 done, F19's 18 stories written `ready` (EN-068 to EN-085), F20's three drafted stories numbered (EN-086 to EN-088).

## 2026-10-17 — Run of F18 continued, up to its audit

- `#run` EN-061 turned ADR-0043's 15-module signal into a test that counts every folder on disk, and the spread found in F14 into a lint selector; two places in the tooling copied their accumulator at every step and now build into a local list.
- `#run` EN-052: `npm run worktree:new -- <ID>` (Rust, `tools/worktree/`) makes `.claude/worktrees/<id>` on the story's branch from its feature branch, links `node_modules` and installs Husky's hooks folder so a push runs the branch's own `pre-push`; `branches:clean` removes the worktrees of merged or deleted branches; `guard-bash` answers a chained Git command in a worktree with the plain command, and refuses `golden:write` there, which the golden test refuses too.
- `#run` EN-056: the checklist became a column beside the drawing, a test caps each feature at 12 lines (E15's F05 cut from 14 to 12), and `val:prepare` (Rust) drafts a feature's lines from its criteria; the guided steps of E15 stay for `?guide=epic`.
- `#run` EN-058: `npm run mutate` (`tools/mutate/`, Rust, std only) tries textual mutations — comparison, operator, constant, condition, return — in a temporary worktree outside the repository, under the heavy lock, and reports caught and surviving ones; the audit skill's A4 step points to it.
- `#run` EN-059: a story's `done` commit that changes a rule in `docs/domain/` is refused by the `commit-msg` hook (`tools/rule-docs`, Rust) until its change holds the Q row, the feature's VAL decisions to confirm and the guide page, or the footer `Rule-docs: none affected, <why>`.
- `#run` US-061: the font list held while the default font is read again (I77), a face refused at that reading dropped (I78); second criterion read as dropping, from the settled rule that every font offered can be exported.
- `#run` EN-060: the documentation pass on E15's long pages split every bullet over 400 characters of the Text section of `shapes.md`, the Colors of `interaction.md` and `DERIV-text-box` into sub-bullets and tables, without changing a rule; a test keeps those parts within the limit.
- `#run` EN-046: the release pull request now lives on `develop` and `main` only tags it (ADR-0067); the Product Owner merges the release pull request into `develop` before #259, checked at the release of 0.3.
- `#run` EN-063: the hook `limit-agents` runs on `Agent` and `SendMessage`, Claude Code's own cap set to 3; a message to a running subagent or to the main conversation goes through, a resume of a fourth is refused.

## 2026-10-17 — Run of F18 stopped before its validation

- `#run` The run of F18 resumed from a plain terminal on 2026-10-16 and stopped on 2026-10-17 before VAL-019: SP-020 (#262), EN-049 (#263), EN-047 and its lock fix, EN-054, EN-062, EN-050 (#268), EN-053 (#269), EN-057 (#270), EN-055 (#271), EN-051 (#272) and its fix (#275), EN-061 (#273), EN-052 (#274), EN-048, EN-056, EN-058, EN-059, US-061, CHK-019 (#282), EN-046, the move-rows fix (#283), EN-060 (#284), and AUD-020 with its fixes (#285, #286); the details are in the feature branch's journal and stories.
- `#decision` The Product Owner asked that every new script, hook or agent tool be written in Rust from now on (E17's goal): eight std-only crates under `tools/`, checked by `check:rust`.
- `#decision` Tables without padding moved to F19 (ADR-0068, to confirm at VAL-019): Prettier pads every table unless prose is joined into long lines; the choice was left to the agent.
- `#run` Stop reason: VAL-019 needs the Product Owner. Waiting for them: the `limit-agents` line in `.claude/settings.json` (EN-063, F18.AC2), the decisions to confirm in VAL-019, F19's four spike questions (SP-021, pushed on `docs/sp-021-research-f19`, merged at F19's start), and the rule GitHub bypasses for the main thread's direct commits on the feature branch.

## 2026-10-16 — Run of E17 started: F18, with F19's spike in parallel

- `#run` The Product Owner started a run on 2026-10-16: scope F18 (a safe and cheap parallel run, with E15's leftovers), and F19's spike (SP-021) in parallel. E17 and F18 are `in-progress`, F18's stories `ready`, on `feature/f18-safe-and-cheap-parallel-run`.
- `#decision` Refinement of E17: order F18, F19, F20; the `check:all` target is set after F19's spike; Rust is already installed on the Product Owner's machine; four Claude Code settings recommended by research become stories (EN-063 in F18, EN-064 to EN-066 in F20), each shown to the Product Owner before it is applied; the model per task is measured from E17. Recorded in the epic file.
- `#run` EN-049 wrote the first Rust tool of the repository, the stories index generator, std-only.
- `#run` EN-053 found the status line's wrong states in the transcript — a subagent resumed by a message was never counted — and fixed both the TypeScript status line and its Rust twin.
- `#run` EN-055: `epic:check` names a failed `check:all` by its failing checks, their lanes and failing tests (Vitest, `cargo test`), counts the worktree isolation's refusals apart (549 in E15) and reports the calls started over an hour ago and never ended as lost, never as a failure; the pre-push runs `golden:check` after `check:all` whenever `.git/metrics/golden.json` exists (about 13 s, through the heavy lock); `npm run metrics -- --since <day>` covers the calls, requests and turns from 00:00 UTC that day on.

## 2026-10-16 — F17 validated, E15 reviewed

- `#decision` VAL-018's third pass: US-059 and US-060 accepted, F17 done. Three points left by US-060 went to the improvement log for RET-003.
- `#rev` A tour of the whole Text tool, taken as REV-003: every option works; italic leaving the box unchanged kept. The text direction is kept for now and goes at the later UI/UX epic (E02, ideas). Report: `docs/guide/e15-place-text.md`.
- `#stop` E15 stays `in-progress` until RET-003, run with the Product Owner (ADR-0023); then the `develop` → `main` release.

## 2026-10-16 — Run of E15 through F17, up to the validation

- `#run` 2026-10-15 afternoon: the Product Owner's remarks before validation led to US-037 to US-041, EN-042, EN-043 and ADR-0057, ADR-0058, ADR-0059. F15 finished (EN-041, US-036, AUD-018). Pull requests #224–#235.
- `#decision` 2026-10-15 evening: the Product Owner validated all of E15 (F05, F14, F16, F15 done). Every provisional decision was confirmed except three: a long word always stays whole, no balanced wrap, no HTML layer (ADR-0060, Latin script targeted); fonts are embedded only at export.
- `#run` F17 planned and built overnight: SP-019, US-042 to US-051, EN-044, CHK-018, AUD-019 (#237–#250). VAL-018 prepared on `feat/val-018-text-corrections` as a checklist page (`?checklist=E15`, 53 lines), the new validation method.
- `#process` Methods tried: no per-story review in F17 (the worker breaks its own tests once per criterion) with one thorough feature audit; lighter models (Sonnet) for clerical agents.
- `#open` Apple Color Emoji (192 MB collection) exhausts memory: a decision on mutable byte copies is needed. Three AUD-019 questions are in VAL-018.
- `#stop` VAL-018 needs the Product Owner; then NAM-001, REV-003, RET-003 and the `develop` → `main` pull request.

## 2026-10-15 — Run of E15 up to the validations of F05, F14 and F16

- `#run` Started on 2026-10-14 in the evening; the main thread only dispatched, each story going to a worker subagent (ADR-0051, trial).
- `#run` F05 (text) into `feature/f05-text`: SP-015, EN-031, EN-032, US-031, CHK-014, US-032, AUD-015 (#198–#204). VAL-014 prepared on `feat/val-014-text`: 35 guided steps, 17 decisions to confirm.
- `#run` F14 (box of text): SP-016, EN-033–EN-036, CHK-015, US-033, US-034, AUD-016 (#205–#213). VAL-015 prepared on `feat/val-015-text-box`: 39 steps, 22 decisions.
- `#run` F16 (colors): SP-017, EN-037, EN-038, CHK-016, US-035, AUD-017 (#214–#219). VAL-016 prepared on `feat/val-016-colors`: 40 steps, 20 decisions.
- `#run` F15 (fonts, HTML layer): SP-018 merged (#220, ADR-0053); EN-039 in progress.
- `#process` One worker per story in its own worktree; an independent audit of every story before its merge, with mutations, often by brute force, its findings sent back to the worker. Every story had real findings, all fixed. The decisions the agent took alone are written as provisional and listed in each VAL story.
- `#stop` The Product Owner validates F05, F14 and F16 from 14:00; F15 continues meanwhile.
- `#metrics` The session report's counter resets each time the Product Owner writes (logged for RET-003). About 10 h 40 of work reported overnight, then about 9 h from 2026-10-15 01:00 to 13:30. The main thread stayed free throughout: the Product Owner wrote twice and was answered at once.

## 2026-10-14 — Run of F05 started; E14 released

- `#run` The Product Owner started a run: the next feature in order is F05 (text, E15), so the run begins with its refinement, in two rounds (first questions with a long free comment, then every remaining question at once). Its runs keep the main thread free (ADR-0051, trial).
- `#process` New routine: `npm run epic:check` at the start and at the end of each epic, GitHub's code scanning alerts included; at each retrospective, the list of tools worth writing and the metrics of those written; at each epic's end, a pull request `develop` → `main` the Product Owner validates.
- `#security` The two open CodeQL alerts (incomplete sanitization in `scripts/library-html.ts`) concern the code on `main`, already rewritten on `develop`: the release of E14 closes them.

## 2026-10-14 — E14 closed

- `#ret` The Product Owner answered the retrospective's questions. The main thread stays free during E15's runs: each story goes to a worker subagent, as a measured trial (ADR-0051). Point 1 alike in every shape is decided as recommended and built in E10: the rectangle starts at its bottom-right corner, the round shape at its right point. Every questionnaire now ends with room for remarks.
- `#tools` Every tool worth writing is listed, with the ones worth Rust: the hooks and the status line, measured at 65 ms and 149 ms per run, and the large readers (`docs/tooling/agent-tools.md`). A lighter context at session start is planned during E02, the epic after E15.
- `#merge` RET-002 and E14 done. The retrospective's pull request (#194) holds the review's (#193) and merges by itself once labeled (ADR-0050).

## 2026-10-14 — RET-002, the retrospective of E14

- `#ret` The Product Owner decided every logged item and asked for the work without further questions; their remaining questions wait in the record (`docs/process/retrospectives/E14-retrospective.md`).
- `#tooling` Status line: the cause of the stuck background task found in the transcript (a call refused before it started answers with an error, never a notification); four states, a line per background task. Each call's record holds the session's state; the report splits turns between tools and model. `check:all` in parallel lanes (90 s to 54 s), ESLint's cache for fixes (28 s to 8 s).
- `#tools` `npm run replace`, `backlog:status`, `story:finish`, `golden:write` and `golden:check`, `activity`, `epic:check`; specifications of the next ones and the reason Rust waits (`docs/tooling/agent-tools.md`).
- `#decisions` ADR-0050 (tools, parallel checks, validated epic stories merging by themselves, GitHub deleting merged branches); ADR-0051 proposed (the main thread kept free, stories worked by subagents) for a trial at the next run; source tiers in the research protocol; a JSDoc word no longer counts as a use; point 1 alike in every shape noted in E10 with SVG 2's convention.

## 2026-10-14 — REV-002, the review of E14

- `#rev` A free walk-through of the playground: round shape, conversion, handles in every mode, a loop, deleting a point, turning; everything as intended. Report `docs/guide/e14-draw-curves.md`. One idea for the end of V1: every shape's point 1 at its lowest right point (note in E10, before E11 fixes the file format).
- `#ret` For RET-002, the Product Owner raised: the status line still shows a background task after the end of the turn; it should say whether the agent works, waits only for agents (the chat is free), or both; richer data per command for the analysis; what agents do is hard to follow; a full check of what the epic produced, at each retrospective.

## 2026-10-14 — F10 and F13 validated

- `#val` VAL-012 and VAL-013: the Product Owner tried the round shape, its points and handles, and the shapes made from it, and accepted F10 and F13. One change: **Convert to rounded rectangle** removed (an ellipse becomes a capsule, so it keeps the drawing only for a circle); F10.AC5 reworded, the function and its helper removed, the guided test down to six steps. The agent's other F10 decisions accepted as built.
- `#merge` One feature pull request from `feat/val-013-validate-f13`, which holds F10 and F13; `feat/val-012-validate-f10` deleted, its content inside.

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
