# Journal

What each working session dealt with, in the agent's words: requests of the Product Owner (paraphrased), decisions, where they were recorded. Newest first. Written by the agent at the end of each session or before a long pause (ADR-0024); searched with `grep` (tags written `#tag`). The full conversations stay in the local transcripts (see [agent configuration](../tooling/agent.md#searching-past-conversations)).

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
