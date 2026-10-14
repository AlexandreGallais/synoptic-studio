# E01 — Retrospective

**Dates**: 2026-10-12 to 2026-10-13. **Story**: RET-001. **Participants**: Product Owner, Claude Code.

E01 delivered shapes drawn from numbers in eight features, in six days. The retrospective changed how the project works more than any feature did: no more waiting for servers, measures of every command, a local library of sources, expert reviewing agents, a cleaner repository and a playground that is easier to use.

## 1. Facts

| Measure                 | E01                                                                                                                |
| ----------------------- | ------------------------------------------------------------------------------------------------------------------ |
| Features delivered      | 8 (F01, F02, F07, F03, F09, F08, F04, F06); F10 moved to E14, F05 to E15                                           |
| Stories                 | 70, among them 9 spikes, 9 checkpoints, 9 audits, 9 validations, a review and this retrospective                   |
| Commits                 | 544 before the history restart                                                                                     |
| ADRs                    | 39 (two written at this retrospective)                                                                             |
| Library                 | 121 modules — `model` 50, `geometry` 41, `math` 18, `io` 7, `render` 5; 88 exported functions                      |
| Tests                   | 744, in 13.6 s; 100 % coverage of `src/`; `check:all` in about 50 s locally                                        |
| CI                      | about 2 min per run; at least 40 min spent waiting for it in explicit loops                                        |
| Active time, by feature | F01 4 h 28, F08 3 h 29, F04 3 h 25, F06 2 h 24, F03 2 h 22, F02 2 h 20, F07 1 h 59, F09 1 h 17 (`npm run metrics`) |
| Most frequent commands  | `cat`, `python3`, `grep`, `sed` (reading and editing files); `git push` 46 s median, the pre-push check            |

The durations before the metrics hooks are estimates from the transcripts: they include the time spent writing each command.

## 2. Logged items, decided

The improvement log held 88 rows and the feedback log 17; the retrospective opened with one table of them all, decided row by row by the Product Owner.

| Theme           | Item                                                                                                                                    | Decision                                                                       | Where it went                                                 |
| --------------- | --------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------ | ------------------------------------------------------------- |
| Branches and CI | a `develop` branch; no CI on work branches; checks run twice; releases for every doc push                                               | done                                                                           | ADR-0038                                                      |
| Branches and CI | faster CI                                                                                                                               | done through ADR-0038: one run per release; jobs merged                        | `ci.yml`                                                      |
| Dependencies    | read release notes before upgrading; a cooldown on fresh versions                                                                       | done: 7 days                                                                   | `docs/tooling/versions-and-security.md`, `.ncurc.cjs`         |
| Dependencies    | Dependabot version updates                                                                                                              | dropped; security updates kept                                                 | ADR-0038                                                      |
| Metrics         | log every command with its time; time and tokens per story and task; reports                                                            | done                                                                           | `docs/tooling/metrics.md`                                     |
| Metrics         | an exact token counter; a session timer; the call running                                                                               | done: the status line                                                          | `docs/tooling/metrics.md`                                     |
| Metrics         | a report of time and tokens at the end of each turn                                                                                     | done                                                                           | `npm run metrics:session`                                     |
| Metrics         | a watchdog relaunching a stalled run                                                                                                    | later: the status line shows a silent session; no reliable relaunch exists     | improvement log                                               |
| Sources         | a local library of the pages read, searched before going online                                                                         | done: SQLite full-text search behind an MCP server                             | ADR-0039                                                      |
| Agents          | expert agents with their own documents, in the background                                                                               | done: math, verification, functional analysis, documentation                   | `docs/tooling/agent.md`                                       |
| Agents          | a domain-modelling expert                                                                                                               | part of the `functional-analyst`                                               | `.claude/agents/functional-analyst.md`                        |
| Agents          | a setup portable to other tools                                                                                                         | done: `AGENTS.md`; documentation bundled offline comes with the library        | `AGENTS.md`                                                   |
| Agents          | several passes on the configuration                                                                                                     | done in this retrospective; again every four to five epics                     | `docs/process/agile.md`                                       |
| Agents          | a quiet terminal during runs                                                                                                            | done: commits and status line only                                             | `/run`                                                        |
| Agents          | the bash guard denying quoted commands                                                                                                  | fixed: here-document bodies are skipped                                        | `.claude/hooks/guard-bash.ts`                                 |
| Agents          | a `Stop` hook to keep runs going                                                                                                        | not now: runs did not stop early in E01                                        | —                                                             |
| Repository      | the Product Owner's words removed from every page                                                                                       | done                                                                           | `docs/conventions/writing.md`                                 |
| Repository      | the old history replaced by one commit                                                                                                  | done, with a summary kept                                                      | `docs/process/history.md`                                     |
| Repository      | stricter commit messages                                                                                                                | not needed: no subject starts with a capital; ids inside are allowed           | —                                                             |
| Repository      | `package.json` and configuration files tidied                                                                                           | done; overrides checked, still needed                                          | `docs/tooling/dependencies.md`                                |
| Documentation   | modern writing rules; diagrams as code; every page reworked                                                                             | done: scanning rules, Mermaid, every folder reread                             | `docs/conventions/writing.md`                                 |
| Process         | the agile method on one clear page                                                                                                      | done                                                                           | `docs/process/agile.md`                                       |
| Process         | a start-of-project interview, the agent as functional analyst                                                                           | done                                                                           | `docs/playbook/analysis.md`                                   |
| Process         | a complete experience report and a starter kit                                                                                          | done                                                                           | this record, `docs/playbook/setup.md`                         |
| Process         | ask early which things are kinds of one thing; read rules in their natural sense                                                        | done                                                                           | `docs/playbook/analysis.md`                                   |
| Process         | a rules review every four to five epics and at the end, harden by default                                                               | adopted                                                                        | `docs/process/agile.md`                                       |
| Process         | the epics' order on the backlog's front page                                                                                            | done                                                                           | `docs/backlog/README.md`                                      |
| Process         | parallel threads; several projects at once                                                                                              | later, after several epics                                                     | improvement log                                               |
| Process         | thinking sessions on what stays public with a paid edition                                                                              | later, before the pro edition                                                  | improvement log                                               |
| Playground      | a simpler interface: a floating toolbar, grouped panels, a pipeline column, a reset                                                     | done                                                                           | `playground/index.html`                                       |
| Playground      | a short guided test per feature, the full recheck at the epic review                                                                    | done: `?guide=epic`                                                            | `/review`, `/run`                                             |
| Playground      | the playground asking its own questions, answered by voice                                                                              | later: E12 or the editor applications                                          | improvement log                                               |
| Playground      | step 72: the shape seemed to leave the canvas                                                                                           | fixed: a margin of 40                                                          | `playground/show-shape.ts`                                    |
| Playground      | "press Enter" in the steps while the drawing follows the typing                                                                         | kept: a point's X and Y apply on Enter, so that a minus sign moves nothing yet | `docs/guide/e01-draw-shapes.md`                               |
| Playground      | fields red after choosing a rectangle or a polygon from a free shape                                                                    | fixed                                                                          | `playground/write-whole-shape-fields.ts`                      |
| Tools           | knip                                                                                                                                    | done                                                                           | `knip.json`                                                   |
| Tools           | a property test for every math and geometry function                                                                                    | done as a fitness function; nine functions listed until EN-020                 | `tests/fitness.test.ts`                                       |
| Tools           | a coverage floor for the playground                                                                                                     | done                                                                           | `vite.config.ts`                                              |
| Tools           | StrykerJS                                                                                                                               | later: stryker-js issue 6210 still open on 2026-10-12                          | `docs/tooling/dependencies.md`                                |
| Tools           | a dev container                                                                                                                         | not now: one machine, Node.js pinned                                           | —                                                             |
| Tools           | published previews of feature branches                                                                                                  | not now: local previews suffice                                                | —                                                             |
| Tools           | generated pages: board, traceability, commits per story; tests in a real browser                                                        | later: E12                                                                     | E12                                                           |
| Code            | shared test helpers, one path writer, one pass per corner, folders of `model/`                                                          | planned: F11, first feature of E14                                             | `docs/backlog/features/E14-F11-code-base-ready-for-curves.md` |
| Code            | separate tests from code, folders that make sense                                                                                       | researched by SP-010 before any move                                           | F11                                                           |
| Code            | a state module for the playground, radii per shape kind, `boundingBox` on large lists, concave free shapes, old files without an anchor | later, with the features that need them                                        | improvement log                                               |
| Code            | references verified without a date                                                                                                      | dated when their features come                                                 | `docs/references.md`                                          |
| Product         | ideas for later epics: fill the parent box, a shape menu, imports of large SVG, a design system, memory of removed radii                | kept in the domain and the epics                                               | `docs/domain/`, `docs/backlog/epics/`                         |
| Product         | an agile workspace application for working with agents                                                                                  | a separate private project, after this one                                     | —                                                             |

The feedback log's 17 critiques became the habits sheet of [Working with the agent](../../playbook/agent.md#lessons-from-the-product-owner).

## 3. What helped, what hurt

| Side          | Helped                                                                                                                                                                                                        | Hurt                                                                                                                                                           |
| ------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Product Owner | the playground: testing each feature there, instead of reading the code or the pull requests, made validation faster and let the code be trusted to its tests and audits                                      | the waits for pull requests and their CI, which stretched every working session; the same documentation pages fetched again and again when one read was enough |
| Agent         | spikes before code: no story blocked on a missing source; the auditor found real bugs (signed zeros, a spike consumed by its fillet); the guided test in the playground; questionnaires with numeric examples | waiting for the CI; questions asked twice; reading the same pages again; a long guided test that grew with every feature; no measure of time                   |

## 4. Evolvability review

| Question               | Answer                                                                                                                      |
| ---------------------- | --------------------------------------------------------------------------------------------------------------------------- |
| 1. Size                | `model/` holds 50 modules, `playground/` 80: folders planned in F11 (SP-010, EN-022). Tests 13.6 s, `check:all` about 50 s. |
| 2. Tools               | knip's trigger was reached: adopted. StrykerJS still blocked (issue 6210). Mermaid adopted for diagrams.                    |
| 3. Fitness functions   | a property test for every math and geometry function, now tested; the layer imports were already a lint rule.               |
| 4. Patterns            | repeated test helpers and two path writers: F11 (EN-021).                                                                   |
| 5. Agent configuration | `AGENTS.md` and `CLAUDE.md` rewritten; five expert agents; skills updated; the guard's false positive fixed.                |
| 6. Process             | cadences kept; the guided test shortened per feature; a rules review every four to five epics; Markdown backlog still fit.  |

## 5. Changes adopted

- ADR-0038: `develop`, local checks, one CI into `main`, merge commits for features and releases, a 7-day cooldown.
- ADR-0039: the local source library behind an MCP server.
- Metrics hooks, the status line, the session report, `npm run metrics` read at each retrospective.
- `AGENTS.md`; expert agents `math-expert`, `verifier`, `functional-analyst`, `doc-editor`.
- Writing rules for scanning readers and for the Product Owner's decisions; Mermaid diagrams.
- The playground's toolbar, panels, pipeline column, reset and short guided tests.
- knip, the property fitness function, the playground's coverage floor; `package.json` tidied; repository-wide tests in `tests/`.
- F11 in E14 for the code debt; the history restarted at 0.1.0.

## 6. Playbook updates

- [Principles](../../playbook/principles.md): measure, read once, never wait for a server, write decisions not words.
- [Starter kit](../../playbook/setup.md): fourteen steps, the rhythm, what to read first, the cases met in E01.
- [Start-of-project interview](../../playbook/analysis.md): new.
- [Working with the agent](../../playbook/agent.md): the configuration table, the Product Owner's lessons habit by habit.
- [Lessons](../../playbook/lessons.md): seven new rows.

## 7. Best practices read for agent-built projects

| Source              | What it says that we use                                                       |
| ------------------- | ------------------------------------------------------------------------------ |
| `REF-CC-MEMORY`     | short instruction files; `AGENTS.md` shared with other tools through an import |
| `REF-CC-HOOKS`      | hooks for what must always happen; `duration_ms` on every tool call            |
| `REF-CC-SUBAGENTS`  | reviewers in a fresh context, with their own tools and documents               |
| `REF-CC-STATUSLINE` | the session's facts on a status line refreshed every second                    |
| `REF-MCP-TOOLS`     | a small tool server over stdio                                                 |
| `REF-NNG-SCANNING`  | readers scan: concise, scannable, objective text                               |
