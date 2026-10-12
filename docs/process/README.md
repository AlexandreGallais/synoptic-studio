# Process

How the project inspects itself and adapts (ADR-0023). The method on one page: [agile.md](./agile.md); its full rules: [backlog](../backlog/).

## Cadences

| When                               | What                                                                                                                            | Who                           | Output                                                                             |
| ---------------------------------- | ------------------------------------------------------------------------------------------------------------------------------- | ----------------------------- | ---------------------------------------------------------------------------------- |
| any time                           | raise an idea, an irritant or a lesson (the agent writes it, in neutral words)                                                  | both                          | a row in the [improvement log](./improvements.md)                                  |
| any time                           | a critique of how the agent worked: what happened, what it should have done, what now prevents it                               | agent                         | a row in the [feedback log](./feedback.md)                                         |
| start of each story, each review   | triage of open GitHub issues (`/triage`): each presented with a recommendation                                                  | agent, then Product Owner     | decisions, backlog items, replies (ADR-0026)                                       |
| start of a feature                 | refinement interview, research spike `SP`                                                                                       | both, then agent              | stories, sources                                                                   |
| during a run (ADR-0028)            | stories chain on the feature branch; checkpoint `CHK` mid-feature; a test card per user story                                   | agent                         | stories merged into the feature branch, cards to run                               |
| end of a feature                   | audit `AUD` (with light evolvability check), validation `VAL` (demo page, guided test)                                          | agent, then Product Owner     | findings, [guide](../guide/) page, feedback                                        |
| after `VAL`                        | local previews; what the Product Owner saw compared with what the demo meant to show                                            | both                          | gaps recorded                                                                      |
| release                            | feature pull request merged into `develop`, then the release pull request into `main`; next feature prepared (interview, spike) | Product Owner, agent prepares | a release, next feature refined                                                    |
| end of an epic                     | review `REV`: epic report and user test                                                                                         | agent, then Product Owner     | guide report, backlog and domain changes                                           |
| end of an epic                     | retrospective `RET`: metrics, way of working, evolvability, playbook                                                            | both                          | [retrospective record](./retrospectives/), ADRs, stories, [playbook](../playbook/) |
| end of an epic, after `RET`        | fresh research on best practices for agent-built projects; playbook and starter kit updated                                     | agent                         | [playbook](../playbook/)                                                           |
| every 4 to 5 epics, end of project | review of every rule ([agile.md](./agile.md#cadences-beyond-the-epic))                                                          | both                          | decisions, ADRs, a stricter configuration                                          |

## Evolvability review (in every `RET`, light in every `AUD`)

Questions asked as the project grows — each answer is "nothing yet", a story, or an ADR:

1. **Size**: files and functions per layer, longest procedures, test duration, `check:all` duration, and the metrics of the epic (`npm run metrics`: commands, slowest calls, time and tokens per story). Is a layer becoming a package (monorepo ADR)?
2. **Tools**: each row of "considered for later" in [dependencies](../tooling/dependencies.md) — has its trigger happened (e.g. StrykerJS issue fixed, first npm publication, API stabilized)?
3. **Fitness functions** (`REF-FITNESS-FUNCTIONS`): which architecture rule is still only written and could be tested (layer imports, purity, size, performance budget)?
4. **Patterns**: duplicated shapes of code or docs that ask for a shared abstraction; abstractions no longer used.
5. **Agent configuration**: `AGENTS.md` and `CLAUDE.md` short (under 200 lines together), rules and skills still true, guards still useful, recurring mistakes not yet covered by a check.
6. **Process**: cadences kept or too heavy; backlog tool still fit (Markdown, board, GitHub Projects).

## Folders

| Page                                 | Content                                                                                                   |
| ------------------------------------ | --------------------------------------------------------------------------------------------------------- |
| [agile.md](./agile.md)               | the method on one page: levels, story types, who does what, cadences                                      |
| [improvements.md](./improvements.md) | improvement log, fed continuously, emptied by each retrospective                                          |
| [feedback.md](./feedback.md)         | the Product Owner's critiques of the agent's work, turned into the playbook's sheet at each retrospective |
| [retrospectives](./retrospectives/)  | one record per retrospective                                                                              |
| [history.md](./history.md)           | the record of the commits before the restart at 0.1.0                                                     |
| [journal.md](./journal.md)           | what each session dealt with, newest first (ADR-0024)                                                     |

## Coming from the V-model

The V-model is not this project's lifecycle (requirements grow feature by feature); but the functions of its documents have a place here, generated or tested rather than written by hand (ADR-0027):

| V-model document      | Function                         | Here                                                     |
| --------------------- | -------------------------------- | -------------------------------------------------------- |
| system specification  | the need                         | [domain](../domain/) (vision, purpose, rules), epics     |
| software requirements | verifiable behaviors             | feature acceptance criteria `F01.ACn`                    |
| interface description | contracts                        | public API reference (TypeDoc), ADRs                     |
| design description    | design and its reasons           | ADRs, [conventions](../conventions/), derivations        |
| test plan             | strategy                         | [test strategy](../conventions/testing.md#test-strategy) |
| test description      | test cases                       | tests, tagged `[F01.ACn]` when they prove a criterion    |
| test report           | results on a version             | JUnit report attached to each GitHub release             |
| traceability matrix   | requirement ↔ realization ↔ test | feature plans, test tags; generated page planned (E12)   |
| version description   | what a version contains          | `CHANGELOG.md`, GitHub releases                          |
