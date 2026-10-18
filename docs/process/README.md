# Process

How the project inspects itself and adapts (ADR-0023). The method: [agile.md](./agile.md); the backlog's rules: [backlog](../backlog/).

## Cadences

Each moment has one owner and one output, so that nothing waits on an unnamed person.

| When | What | Who | Output |
| --- | --- | --- | --- |
| any time | an idea, an irritant or a lesson, written in neutral words | both | a row in the [improvement log](./improvements.md) |
| any time | a critique of how the agent worked: what happened, what it should have done, what now prevents it | agent | a row in the [feedback log](./feedback.md) |
| start of each story and review | triage of open GitHub issues (`/triage`), each with a recommendation (ADR-0026) | agent, then Product Owner | decisions, backlog items, replies |
| start of a feature | refinement interview, research spike `SP` | both, then agent | stories, sources |
| during a run (ADR-0028) | stories chain on the feature branch up to the scope the Product Owner set; a test card per user story, a checkpoint `CHK` midway | agent | stories merged into the feature branch, cards to run |
| end of a feature | audit `AUD` (with a light evolvability check), validation `VAL` (demo page, checklist page) | agent, then Product Owner | findings, [guide](../guide/) page, feedback |
| after `VAL` | local previews: what the Product Owner saw against what the demo meant to show | both | gaps recorded |
| release | feature pull request into `develop`, then release pull request into `main`; next feature prepared | Product Owner, agent prepares | a release, next feature refined |
| end of an epic | review `REV` (report and user test), then retrospective `RET` (metrics, way of working, evolvability, playbook) | agent, then Product Owner | guide report, [retrospective record](./retrospectives/), ADRs, stories |
| end of an epic, after `RET` | fresh research on practices for agent-built projects | agent | [playbook](../playbook/) and starter kit updated |
| every 4 to 5 epics, end of project | review of every rule ([agile.md](./agile.md#cadences-beyond-the-epic)) | both | decisions, ADRs, a stricter configuration |

### A run stops only on one of four things (EN-086)

A run keeps working until the first of: the end of the scope the Product Owner set (a time or an end); a blocker no other story can get around; the Product Owner's word; 90 % of the week's usage. Anything else is not a stop: a question parks its own story, with the question written in it, and the run goes on with the independent stories. Reason: a question blocks one story, not the work. A question alone never ends a run; the Product Owner answers the parked questions on return.

## Evolvability review

Asked in every `RET` and, lightly, in every `AUD`. The test for each question: does the answer change anything? It is "nothing yet", a story, or an ADR.

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
| [improvements.md](./improvements.md) | improvement log, emptied by each retrospective |
| [tool-incidents.md](./tool-incidents.md) | one card per tool that failed, read and fixed at the epic's end (ADR-0085) |
| [feedback.md](./feedback.md) | critiques of the agent's work, turned into the playbook's sheet at each retrospective |
| [retrospectives](./retrospectives/) | one record per retrospective |
| [history.md](./history.md) | the commits before the restart at 0.1.0 |
| [journal.md](./journal.md) | what each session dealt with, newest first (ADR-0024) |

## Coming from the V-model

The V-model is not this project's lifecycle (requirements grow feature by feature); its documents' functions are generated or tested here rather than written by hand (ADR-0027):

| V-model document | Here |
| --- | --- |
| system specification | [domain](../domain/), epics |
| software requirements | feature acceptance criteria `F01.ACn` |
| interface description | public API reference (`tools/api-pages`), ADRs |
| design description | ADRs, [conventions](../conventions/), derivations |
| test plan and test cases | [test strategy](../conventions/testing.md#test-strategy); tests tagged `[F01.ACn]` |
| test report | JUnit report on each GitHub release |
| traceability matrix | feature plans and test tags; generated page planned (E12) |
| version description | `CHANGELOG.md`, GitHub releases |
