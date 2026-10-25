# Process

How the project inspects itself and adapts (ADR-0023). The method: [agile.md](./agile.md); the backlog's rules: [backlog](../backlog/).

## Cadences

Each moment has one owner and one output, so that nothing waits on an unnamed person.

| When | What | Who | Output |
| --- | --- | --- | --- |
| any time | an idea, an irritant or a lesson, written in neutral words | both | a row in the improvement log (LOG-improvements) |
| any time | a critique of how the agent worked: what happened, what it should have done, what now prevents it | agent | a row in the feedback log (LOG-feedback) |
| start of each story and review | triage of open GitHub issues (`/triage`), each with a recommendation (ADR-0026) | agent, then Product Owner | decisions, backlog items, replies |
| start of an epic | refinement: the Product Owner's account, the whole documentary research, the code read, every question asked at once, the features and their stories counted, in an `SP` from `develop` (ADR-0100) | both, then agent | features, stories, sources |
| start of a feature | its stories, known from the epic's refinement; a short spike `SP` only for a question nobody foresaw (ADR-0100) | agent | stories |
| during a run (ADR-0072) | stories follow one another on the feature branch up to the scope the Product Owner set, a fresh worker each; a test card per user story | agent | stories merged into the feature branch, cards to run |
| end of a feature | audit `AUD` (with a light evolvability check), then the feature into `develop` (ADR-0100) | agent | findings, the feature in `develop` |
| release | the Product Owner runs `release.yml` on `main` from the Actions tab, when they choose (ADR-0086) | Product Owner | a tag and a GitHub release |
| each return of the Product Owner | a feature under way finished first, then the review `REV` from `develop`: the features finished since the last merge into `main`, which the Product Owner tries on `develop`, the naming pass over the code changed since, the tool cards fixed, the method and the docs; then `develop` → `main`, an epic finished or not (ADR-0086, ADR-0100) | agent, then Product Owner | review record (PAGE-retrospectives), ADRs, stories, a merge into `main` |
| the review after an epic's last feature | fresh research on practices for agent-built projects | agent | [playbook](../playbook/) and starter kit updated |
| every 4 to 5 epics, end of project | review of every rule ([agile.md](./agile.md#cadences)) | both | decisions, ADRs, a stricter configuration |

### A run stops only on one of four things (EN-086)

A run keeps working until the first of: the end of the scope the Product Owner set (a time or an end); a blocker no other story can get around; the Product Owner's word. The week's usage stops no run since 2026-10-25, the Product Owner having dropped the stop at 90 % (`run-totals should-stop` says `go`). Anything else is not a stop: a question parks its own story, with the question written in it, and the run goes on with the independent stories. Reason: a question blocks one story, not the work. A question alone never ends a run; the Product Owner answers the parked questions on return.

## Evolvability review

Asked in every `REV`, when its span gives figures, and lightly in every `AUD`. The test for each question: does the answer change anything? It is "nothing yet", a story, or an ADR.

| Question | Look at |
| --- | --- |
| **Size** | files and functions per layer, longest procedures, duration of tests and `check:all`, the epic's `target/release/metrics`; is a layer becoming a package? |
| **Tools** | each "considered for later" row of [dependencies](../tooling/dependencies.md): has its trigger happened? |
| **Fitness functions** (`REF-FITNESS-FUNCTIONS`) | an architecture rule only written that a test could hold: layer imports, purity, size, performance |
| **Patterns** | duplicated shapes of code or docs that ask for an abstraction; abstractions no longer used |
| **Agent configuration** | `AGENTS.md` and `CLAUDE.md` under 200 lines together; rules and skills still true; guards still useful; recurring mistakes with no check yet |
| **Process** | cadences kept or too heavy; the backlog tool still fit |

## Pages

| Page | Content |
| --- | --- |
| [agile.md](./agile.md) | levels, story types, who does what |
| LOG-improvements | improvement log, emptied by each review |
| LOG-tool-incidents | one card per tool that failed, read and fixed at the next review (ADR-0085, ADR-0086) |
| LOG-feedback | critiques of the agent's work, turned into the playbook's sheet at each review |
| retrospectives (PAGE-retrospectives) | one record per epic retrospective up to E17, then per review |
| LOG-history | the commits before the restart at 0.1.0 |
| LOG-journal | what each session dealt with, newest first (ADR-0024) |

## Coming from the V-model

The V-model is not this project's lifecycle (requirements grow feature by feature); its documents' functions are generated or tested here rather than written by hand (ADR-0027):

| V-model document | Here |
| --- | --- |
| system specification | the domain's rules (`RULE-…`), epics |
| software requirements | feature acceptance criteria `F01.ACn` |
| interface description | public API reference (`tools/api-pages`), ADRs |
| design description | ADRs, [conventions](../conventions/), derivations |
| test plan and test cases | [test strategy](../conventions/testing.md#test-strategy); tests tagged `[F01.ACn]` |
| test report | JUnit report on each GitHub release |
| traceability matrix | feature plans and test tags; generated page planned (E47) |
| version description | GitHub releases, their notes the bodies of the pull requests into `main` |
