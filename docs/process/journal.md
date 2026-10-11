# Journal

What each working session dealt with, in the agent's words: requests of the Product Owner (paraphrased), decisions, where they were recorded. Newest first. Written by the agent at the end of each session or before a long pause (ADR-0024); searched with `grep` (tags written `#tag`). The full conversations stay in the local transcripts (see [agent configuration](../tooling/agent.md#searching-past-conversations)).

## 2026-10-12 — F04 run stopped at VAL-007

- `#run` Merged into `feature/f04-free-rotation`: SP-007 (#122), EN-014 (#123), EN-015 (#124), US-018 (#125), CHK-007 (#126), US-019 (#127), AUD-007 (#128; #129, a duplicate opened by an interrupted command, closed). Stop: the next story is VAL-007, with the Product Owner.
- `#model` Every shape has an angle (0 to 359, clockwise, about the center of its box) and a position (ADR-0035); the straight box counts the arcs (`DERIV-arc-bounding-box`); edits of a turned shape keep it in place (`DERIV-turned-shape` step 5).
- `#playground` Angle field, −90° and +90° buttons, X and Y of the straight box, the turned box as a frame and the straight box dashed; width, points and conversion of a turned shape.
- `#review` Reviews caught a moved free shape losing its typed width, a frame covering a 1 × 1 shape, a width over 100 000 making a turned shape jump, a test card describing the boxes the wrong way round; the audit a width retyped, ADR-0034 changed silently, "as drawn" for straight corners.
- `#process` The Product Owner relaunched the run twice, seeing no token move during long scripts; asked for a live view of what runs, and for a count of the commands run most, to make runs faster (improvement log).

## 2026-10-11 — F04 run authorized

- `#run` Scope: F04 from SP-007 up to VAL-007, started with `/run` right after the proposed stories (« si je te fais des /run, ça veut dire oui », 2026-10-09). F04 set `in-progress`, its stories `ready`, on `feature/f04-free-rotation`.

## 2026-10-11 — F08 merged, F04 refinement

- `#review` F08 merged by the Product Owner (#118), released in 0.12.0; the validation branch deleted, only `main` left.
- `#run` `/run` without a scope: no feature in progress, the next in E01's order is F04 (free rotation), so its refinement interview first, on `feature/f04-free-rotation`.
- `#domain` Decided: a shape turns about the center of its box by an integer angle, clockwise, kept from 0 to 359; width and height stay those before turning; points keep their numbers; every shape gets a position, the x and y of its straight box (« la vraie get bbox »), the turned box drawn as the selection.
- `#process` Critique: point numbering asked again although settled at F08 (« on en a déjà parlé, c'est dingue »); the rule is now in `shapes.md` §1.

## 2026-10-11 — F08 validated

- `#review` The Product Owner tested F08 in the playground: « Putain, c'est génial, ça marche de fou. » VAL-005 and F08 set `done` on `feat/val-005-validate-node-editing`, with the guided test (steps 39 to 46) and the demo page.
- `#domain` Q28 settled (ADR-0034): only the coordinate typed is an integer, edits keep the fractions, deleting joins the neighbours; Q29 opened (degenerate shapes, a cap on typed coordinates).
- `#playground` The Radius field sits with the point fields in Points mode (the Product Owner's remark).
- `#domain` For the editor applications: moving several points by `+10`, arithmetic in number fields.

## 2026-10-11 — F08 run stopped at VAL-005

- `#run` Merged into `feature/f08-node-editing`: EN-013 (#109), EN-011 (#110), US-014 (#111), CHK-005 (#112), US-015 (#113), US-016 (#114), AUD-005 (#115). Stop: the next story is VAL-005, with the Product Owner.
- `#playground` Points mode: a polygon converted to a free shape without moving; x and y of one point typed (on Enter); a point added in the middle of two neighbours; points deleted by a button, the Delete key or Backspace (at least 3 left).
- `#review` Story reviews caught two crashes on a refused shape (US-014), a negative x that could not be typed, the Mac delete key (Backspace); the audit caught an edit leaving a flat shape and two surviving mutations — all fixed.
- `#domain` Q28 opened: five decisions for VAL-005 (integer coordinate or whole point after a move, fractions kept after edits against ADR-0003, degenerate and crossing contours, neighbours joined on deletion, a cap on typed coordinates).

## 2026-10-11 — F08 resumed, autonomous run authorized

- `#review` F09 merged by the Product Owner (#106). Q27 settled: fractions until edited, the x and y typed are the SVG's, each point kept with its radius (a corner); Q18 amended for the regular polygon, numbered from the right end of its base (the Product Owner's idea).
- `#run` Scope: F08 from EN-013 up to VAL-005, started with `/run` after the proposed stories. Its refinement and stories are on `feature/f08-node-editing` (with SP-005), merged into `main` with the feature.

## 2026-10-11 — F09 validated

- `#review` The Product Owner tested F09 from the card: « Pour le rectangle, là, c'est vraiment parfait. » VAL-006 and F09 set `done` on `feat/val-006-validate-stretchable-free-shape`, with the guided test (steps 34 to 38) and the demo page.
- `#domain` Remark routed to Q27 and F08: keep each point with its radius, as a corner, rather than two lists.
- `#process` A domain-modelling expert agent, for RET-001 (improvement log).

## 2026-10-11 — F09 run stopped at VAL-006

- `#run` Merged into `feature/f09-stretchable-free-shape`: SP-006 (#99), EN-012 (#100), CHK-006 (#101), US-017 (#102), AUD-006 (#103). Stop: the next story is VAL-006, with the Product Owner. F08's spike SP-005 (#97) is on its paused branch.
- `#model` The rectangle is a free shape (ADR-0032): its points stored in their own box, stretched by width and height from the top-left, without loss (`DERIV-free-shape-stretch`); every F01 and F03 rectangle test now runs through it.
- `#review` Reviews caught a far edge off by 4e-15 (`(29 / 7) · 7`) before code, a test that could not fail on loss, `boundingBox` throwing past 100 000 points; 10 of 10 audit mutations caught.
- `#domain` Q27 opened: integer points and size of F09 against the fractional vertices F08 decided — to settle before F08 resumes.
- `#tooling` Personal settings for the Product Owner: focus view and a status line showing the current step; files written through the shell.

## 2026-10-11 — F09 inserted before F08, F08 paused

- `#review` The review of SP-005 raised the rectangle's width and height once its points move. The Product Owner decided that the rectangle is a free shape, stretchable by its width and height without loss, in a feature of its own: « tu peux faire une feature ». F09 comes before F08 in E01; F08 is paused after SP-005 (#97), its other stories back to `draft`.
- `#run` F09 runs from SP-006 up to VAL-006 (« go »). `feature/f08-node-editing` stays on `origin` while F09 runs: a paused feature branch, deliberate exception to "one feature branch at a time".

## 2026-10-11 — F08 refined, autonomous run authorized

- `#run` Scope: F08 (node editing), from SP-005 up to VAL-005. The Product Owner started it with `/run` after the proposed stories, F08 kept in E01.
- `#backlog` F08 `in-progress`, its eight stories `ready`. Later ideas went to E02 (dragging, stretching by the box, the conversion warning) and E03 (operations on edges).
- `#tooling` The CI of release 0.10.0 failed on a plugin version published seconds before; updated to 65.3.0; a `--cooldown` proposal in the improvement log.

## 2026-10-11 — F03 validated

- `#review` The Product Owner tested F03 from the cards: « C'est bien, j'ai bien mon premier point et tout. Ça, c'est trop cool. Comme ça, c'est le sens. » VAL-004 and F03 set `done` on `feat/val-004-validate-radius-per-vertex`, with the guided test (steps 29 to 33) and the demo page.
- `#domain` Q26 opened: a memory of the radii of removed corners (12 → 3 → 12), against F03.AC3 as decided.
- `#backlog` The free contour, with x and y typed in the list of points, confirmed for F08 (next).

## 2026-10-11 — F03 run stopped at VAL-004

- `#run` Merged into `feature/f03-radius-per-vertex`: SP-004 (#79), EN-010 (#80), US-011 (#81), CHK-004 (#82), US-012 (#84), US-013 (#87), AUD-004 (#90). Stop: the next story is VAL-004, with the Product Owner.
- `#model` One requested radius per vertex (`radii`, ADR-0029), radii kept by vertex number, the common radius or Mixed; the selection of vertices (click, Ctrl / Cmd, Shift range in the list) as pure model functions.
- `#playground` Shape and Points modes, handles (point 1 red, point 2 green), a list of points, the radius field for the selected points.
- `#review` Every review changed its story; three real defects caught: a removed vertex's radius coming back, radii lost while a polygon was briefly refused, a radius typed in Points mode not capped (Q20). 14 of 14 audit mutations caught.
- `#process` The run picked up `main` before AUD-004 (parallel session, PR #83): no change to F03.
- `#domain` Q25 opened: one radius or one per corner for a layout box (E13).

## 2026-10-11 — Agenda for the E01 retrospective

- `#process` Raised by the Product Owner for RET-001 (improvement log): local library of read sources searched by content, expert subagents, a cleaner log without diffs, a setup portable to other AI tools and offline skills, a complete experience report; parallel work only after several epics.
- `#process` More for RET-001: rework the agile method in one clear page (US, EN, AUD, VAL), parallel documentation threads, a start-of-project interview with the agent as functional analyst, human-ready reports, Product Owner tests without help, a summary of all topics first.
- `#domain` Pro ideas added: a Rust engine without the free edition's business limits, and a full drawing interface next to the synoptic editors (`editions.md` §2).
- `#domain` Q24 open: a `foreignObject` tool, free or pro, against ADR-0002.

## 2026-10-11 — Where the project stops: free V1, then pro edition

- `#domain` The Product Owner sets a stop: a complete free V1, then bug fixes only; every new feature after V1 goes to a paid pro edition, private, as a service or self-hosted (ADR-0030, [editions](../domain/editions.md)). « Ouais, super, faire du libre, c'est cool, mais il faut vraiment mettre un stop. »
- `#domain` Pro ideas kept out of V1: symbol change report, open API on every element, project management and servers, Rust and WebAssembly engine, maybe an online service. Storage in V1 is **local only** (Q5 replaced): a working folder, files saved one by one, no server at all; E11 renamed. Q22 settled: every epic makes V1 (E11 as files, E07 local only); a design system epic may come later. Open: Q23 (application shell: web view in Electron or a Rust-based app). Pro ideas stay in this public repository, by the Product Owner's choice.
- `#process` Parallel sessions: written in a second chat during the F03 run, in its own worktree; the run now picks up `main` before each story ([agent configuration](../tooling/agent.md#parallel-sessions)).

## 2026-10-11 — F03 refined, autonomous run authorized

- `#run` Scope: F03 (per-vertex corner radius), from SP-004 up to VAL-004. The Product Owner started it with `/run` after the proposed stories: « Fais ton run jusqu'à la fin et puis voilà, merci. »
- `#backlog` F03 split: node editing is F08. F03 `in-progress`, its eight stories `ready`.
- `#process` To review at the end of each epic, at the Product Owner's request: versions, branches, and the small releases made by documentation pushes to `main` (improvement log).

## 2026-10-11 — F07 released, feedback log, F03 next

- `#release` F07 merged into `main` by the Product Owner (#73), released as v0.9.0; branches deleted, only `main` left.
- `#process` New [feedback log](./feedback.md): every critique of the agent's work, quoted, with what should have been done and what now prevents it; grouped at each retrospective into the playbook's sheet. Asked by the Product Owner: « tu vas pouvoir, à la fin, te faire une petite fiche de "Quand je fais les choses de telle manière, il faut que je fasse ça" ».
- `#run` `/run` received with no scope: the next feature in order is F03, which starts with its refinement interview.

## 2026-10-11 — F07 validated

- `#review` The Product Owner finished the guided test (steps 20 to 28): « OK, super, voilà, c'est parfait. » VAL-003 and F07 set `done` on `feat/val-003-validate-anchor`.
- `#domain` Rejected, with its reason: an option pushing a rounded polygon onto its box; measures are taken from the face that touches the box.
- `#playground` Guided test: large Previous / Next at the top of the panel; `?step=N` opens a step and follows the step shown.
- `#process` Before a feature pull request: clean-up, then the Product Owner's yes (run skill, VAL step 6).

## 2026-10-11 — F07 validation prepared, Product Owner testing

- `#review` The Product Owner tested from the US-010 card and stopped part-way: only five of the nine anchor places looked useful, and they left the choice to the agent. The nine are kept: a corner place holds both alignments, and the one that counts follows the box (`docs/domain/shapes.md`).
- `#run` On `feat/val-003-validate-anchor`: tests per criterion counted, F07 guided test (steps 20 to 28, step 26 for the corner place), demo page `docs/guide/f07-polygon-anchor.md`. Waiting for the Product Owner to finish the guided test before the feature pull request.

## 2026-10-11 — F07 run stopped at VAL-003

- `#run` Merged into `feature/f07-polygon-anchor`: SP-003 (#64), EN-009 (#65), US-009 (#66), CHK-003 (#67), US-010 (#68), AUD-003 (#69). Stop: the next story is VAL-003, with the Product Owner.
- `#geometry` The placement follows SVG's `preserveAspectRatio` (meet, nine alignments); anchored vertices land on the box edge within rounding and are written as the integer edge.
- `#review` Every review changed its story: a false "exact in floating point" claim caught in the spike, a vacuous playground test, a stale "center" in the derivation, a missing property for AC4; 8 of 8 audit mutations caught.
- `#tooling` happy-dom's `:checked` follows the attribute, not the state: radio buttons read by property.
- `#backlog` For F03: node editing of an anchored regular polygon is a business question.

## 2026-10-11 — F07 refined, autonomous run authorized

- `#run` The Product Owner typed `/run` with no scope; the order guard gave F07 (next in E01). Refinement: five criteria, option « toucher dans tous les cas » left out (« le but, c'est que ça touche toute la box en radius 0, et puis, si radius 10, ça touche plus, pas grave »). Scope of the run: SP-003 to AUD-003, stop at VAL-003.
- `#backlog` F07 `in-progress`, seven stories `ready`; rotation-after rule recorded in F04.

## 2026-10-10 — Polygon anchor (F07), shape tree and layout box (E13)

- `#backlog` Product Owner's idea after F02: choose where a polygon sits in its box. Draft feature F07, right after F02 in E01: anchor on a 3 × 3 grid, default center (F02 unchanged), stored as an intention, placing the sharp-cornered polygon, rotation after; the "always touch" option is left to the spike.
- `#domain` Q21 settled: a rectangular layout box placing its children by anchor or in a stack, a child overriding the anchor leaves the stack, stacks fixed once the symbol is built, no hug; its name is still to choose.
- `#backlog` New epic E13 (shape tree and layout box), third in the order before E03, whose booleans are tree nodes (confirmed by the Product Owner); the shape tree leaves E05.
- `#agent` Order guard in `/run`: a run only takes the next feature in the Product Owner's order (« si je fais /run machin chouette alors que ça n'était pas dans l'ordre prédéfini, t'as pas le droit »).
- `#deps` `eslint-plugin-jsdoc` 65.2.4 and `prettier` 3.9.10.
- `#process` Irritant logged: the epic order is hard to find. Nothing started: F07 waits for the Product Owner's `/run`.

## 2026-10-09 — F02 validated, merged and released

- `#validation` The Product Owner ran the playground of the feature branch and validated F02 (« ça fonctionne comme je le voulais »); VAL-002 added the tests per criterion, the guided test of F02 (steps 10–20) and the demo page.
- `#release` Feature pull request #59 merged by the Product Owner; release v0.8.0.
- `#cleanup` Feature and validation branches deleted; only `main` remains. F03 not started: the Product Owner asked to wait.

## 2026-10-09 — Idea for SVG import

- `#backlog` Product Owner's idea for E11: oversized shapes from other sources are imported without loss but flagged, simplified and scaled down when possible, refused when too complex. Recorded in the E11 epic and the improvement log.

## 2026-10-09 — Q20 settled; idea of a stretched polygon

- `#domain` Q20 settled on the feature branch (US-008, #55): no upper limit in the calculations, `EPSILON` kept absolute, the interface caps sizes at 100 000.
- `#backlog` Product Owner's idea: a polygon stretched to fill its whole box (a triangle no longer equilateral). Recorded as draft feature F06, separate from F02 (stretching breaks regularity; per-vertex effective radii link it to F03).

## 2026-10-09 — Q20, the Product Owner's view

- `#domain` On Q20 (largest size of a shape), the Product Owner said a user unit should render one screen pixel; even 8K screens side by side stay near 32 000 pixels, 10 000 being already extreme; a far screen is served by scaling the SVG, not by drawing huge views; people should be guided by good practice, e.g. the View Editor asking for the target screen. They leaned towards a tolerance relative to the size, then towards "as is, documented"; decision pending (improvement log, E08 idea).

## 2026-10-09 — Autonomous run of F02: stopped at VAL-002

- `#run` Merged into `feature/f02-regular-polygon` after the auditor and the checks: SP-002 (#43), EN-008 (#46), US-005 (#48), CHK-002 (#49), US-006 (#50), US-007 (#51), AUD-002 (#52). Stop: the next story is VAL-002, with the Product Owner.
- `#domain` Q19 settled at 12 corners after research 0004, run by the Product Owner in claude.ai; polygon orientation through rotation recorded in F04; Q20 opened (largest size of a shape: rounding grows with the size).
- `#audit` Every review changed its story; one real bug found only by review: the Corners field never hid in a browser (`label { display: block }` over `hidden`), invisible to the tests until they loaded the page's style. Feature audit: 26 of 27 mutants caught, 1 equivalent and now proved so.
- `#tooling` Feature branch rebased on `main` to get `deps:tools`; `.lycheeignore` excludes IEC, ANSI and ISO, which refuse robots.

## 2026-10-09 — Autonomous run of F02: refinement first

- `#run` The Product Owner started the run with `/run F02`. F02 is still `draft` without stories: the run opens with its refinement interview, then the research spike; no code before the Product Owner sets the stories `ready`.
- `#backlog` Refinement: Q18 settled, Q19 open (SP-002), eight stories. The Product Owner relaunched `/run F02` right after being asked to agree: F02 set `in-progress`, its stories `ready`.
- `#run` Feature branch `feature/f02-regular-polygon`. SP-002 done up to its last criterion (#43, not labeled): derivation completed, eight sources read, auditor found no correctness error and eight accuracy points, fixed. Stop: Q19 (largest number of corners) is the Product Owner's decision; the symbol standards are paid, request 0004 is optional.
- `#tooling` The Product Owner asked that every tool be on its latest version, not only npm packages: `npm run deps:tools` (`scripts/tool-versions.ts`) in `check:all` fails when a GitHub Action has a newer major or `.nvmrc` is behind the latest Node.js LTS; made to fail on purpose once (Node 22, checkout v6).
- `#tooling` Before the run: `eslint-plugin-jsdoc` 65.2.2 (a patch had turned `check:all` red); the feature branch pattern is written `feature/f<nn>-<topic>` in `CLAUDE.md`, the skills and the Git workflow (ADR-0028 keeps its F01 example).

## 2026-10-09 — F01 validated, merged and released

- `#validation` The Product Owner ran the nine-step guided test in the local playground over two sessions: every step understood as intended (« tout a l'air parfait pour moi »). Q16 settled (a spike is consumed by its fillet), Q17 settled (the radius stays where nothing is rounded).
- `#release` Feature pull request #38 (from the `VAL` branch, holding the whole feature) merged by the Product Owner; release v0.7.0 with its test report; docs site and playground published.
- `#cleanup` All story and feature branches deleted (auto-merges had left the story branches on GitHub); only `main` remains.
- `#process` Learned for the next features: guided test inside the playground, a playground test for the criteria only it can show, one feature pull request from the `VAL` branch, branches deleted through the API (no hook run), `/run` without argument takes the feature in progress.
- `#remarks` For later: radius per vertex by clicking a vertex (F03), browser SVG measurements (E12), specialized expert agents (E01 retrospective).

## 2026-10-09 — Autonomous run of F01: stopped at VAL-001

- `#run` Seven stories merged into `feature/f01-rectangle-with-corner-radius` by GitHub after the auditor and the checks: SP-001 (#30), EN-005 (#31), EN-006 (#32), CHK-001 (#33), EN-007 (#34), US-003 (#35), AUD-001 (#36). Stop: the next story is VAL-001, with the Product Owner.
- `#audit` Every story review found real gaps before merge: a signed-zero half turn in `turningAngle`, a radius never read by the playground, a wrong step in the test card, an untested part of F01.AC3, a test 1000 times too loose. Feature audit: 22 of 24 mutants caught, 2 equivalent.
- `#domain` Open questions raised for the Product Owner: Q16 (spike consumed by its fillet), Q17 (effective radius shown where nothing can be rounded).
- `#process` The Product Owner asked to wait for CI by reading its statuses, failing fast, with a time limit: `/run` and `/story` updated. The AUD-001 pull request had to be recreated (its creation was interrupted) and failed on a happy-dom patch published overnight.

## 2026-10-08 — Autonomous run of F01

- `#run` The Product Owner started the run with `/run FO1` (read as F01, the only feature in progress). Scope: SP-001, EN-005, EN-006, CHK-001, EN-007, US-003, AUD-001; stop at VAL-001. Feature branch `feature/f01-rectangle-with-corner-radius`. No open issue.

## 2026-10-08 — Feature branches and autonomous runs

- `#process` Autonomous runs authorized by the Product Owner: stories chain overnight on the feature branch `feature/f01-…`, each reviewed by the auditor and merged by GitHub when green (label `autonomous`); the feature pull request into `main` is merged by the Product Owner at `VAL`, after testing on local previews; one release per feature (ADR-0028). Direct auto-merge into `main` was refused by the agent's safety classifier and dropped.
- `#github` Ruleset `feature branches` created (same five checks, rebase only); branch names `feature/f01-…` allowed.
- `#backlog` Checkpoint story `CHK` in the middle of each feature (CHK-001 in F01); a Product Owner test card in every user story.
- `#process` After `RET`: fresh research on agent-built projects, playbook and starter kit.

## 2026-10-08 — Requirements and traceability

- `#github` Topics added (svg, typescript, geometry, synoptic, hmi, scada, vector-graphics, library), wiki disabled; pull requests restricted to collaborators by the owner.
- `#process` V-model functions mapped onto the repository: criteria as requirements `F01.ACn`, tests tagged with them, JUnit report attached to each release, test strategy (ADR-0027). E12 validated in principle and order by the Product Owner.
- `#process` Work-centered interview planned before the business epics.

## 2026-10-08 — Contributions, privacy, portable core, product goal

- `#github` External pull requests refused: owner setting "Collaborators only" plus the `external-prs.yml` workflow; issues triaged by the agent with `/triage`, presented to the Product Owner, untrusted data (ADR-0026).
- `#privacy` No personal information about the Product Owner in the repository; personal details removed from the docs.
- `#architecture` Core runnable in any JavaScript engine and transcribable to another language: host globals banned by lint, mathematical intent stated in TSDoc (ADR-0025).
- `#product` Goal: a product teams can trust, with summary documentation for humans; draft epic E12 "Release a product people can trust".

## 2026-10-08 — Cadences, playbook, writing rules, GitHub presentation

- `#process` The Product Owner never edits the repository: their feedback is written by the agent, quoted.
- `#process` Inspection cadence adopted: demo page and guided test at each feature, epic review and retrospective at each epic, improvement log (ADR-0023). Sections `guide/`, `process/`, `playbook/` created.
- `#backlog` F01 and E01 set `in-progress` (Product Owner's agreement); rule: in-progress at the first story.
- `#agent` Session journal and transcript search instead of a database (ADR-0024).
- `#docs` Writing rules extended: voice, word list, page skeletons, messages to the Product Owner.
- `#github` Contributing guide, code of conduct, issue forms, README; repository topics and wiki proposed to the Product Owner.
- `#term` The Product Owner calls this mode "vibe coding"; here every change is still reviewed, tested and sourced.

## 2026-10-08 — Agent environment and verification tools

- `#agent` Skills, path-scoped rules, auditor subagent, guard and session-state hooks (ADR-0021).
- `#test` fast-check properties adopted; StrykerJS tried and deferred (stryker-js issue 6210); weekly lychee link check (ADR-0022).
- `#backlog` Parent-table status drift found (four F01 stories) and now tested.

## 2026-10-08 — Feature research and audit

- `#process` Every feature opens with a research spike and closes with an audit (ADR-0020); SP-001 and AUD-001 added to F01.
- `#docs` `CLAUDE.md` reorganized; stale statements fixed.

## 2026-10-07 / 2026-10-08 — Foundations and first stories

- `#tooling` Toolchain: TypeScript 6.0, ESLint 10 with every rule decided, custom rules, Prettier, Vitest 100 % coverage, VitePress + TypeDoc, commitlint, husky, release-please with auto-merge, GitHub Pages (ADR-0009 to ADR-0019).
- `#domain` Q10–Q15 answered by the Product Owner (`SVG_DECIMALS = 5`, `EPSILON = 1e-9`, clockwise contours, Béziers in drawings only, animation remapping, configuration inheritance, sizes ≥ 0).
- `#backlog` Epics rewritten for the Symbol Editor, Configurator and View Editor; F01 split into stories.
- `#code` EN-001, US-001, EN-002, US-002, US-004, EN-003, EN-004 merged (fixed precision, rectangle contour, path data, playground, fixed scale, turning angle, fillet setback with `DERIV-fillet-setback`).
- `#github` Repository renamed synoptic-studio, Apache-2.0, `RELEASE_PLEASE_TOKEN` (expires 2026-12-31).
