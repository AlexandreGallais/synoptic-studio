# E14 — Retrospective

**Dates**: 2026-10-13 to 2026-10-14. **Story**: RET-002. **Participants**: Product Owner, Claude Code.

E14 delivered curves with handles, the round shape and a code base where every calculation is named. Four features were built in two days. The retrospective made the agent faster and easier to follow:

- tools instead of scripts written again and again;
- checks in parallel;
- a status line that says when the chat is free;
- a record of every call with the session's state;
- a full check of the epic at each retrospective.

## 1. Facts

| Measure                 | E14                                                                                                                                                                  |
| ----------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Features delivered      | 4 (F11, F12, F10, F13)                                                                                                                                               |
| Stories                 | 40, among them 4 spikes, 4 checkpoints, 4 audits, 4 validations, the review and this retrospective                                                                   |
| Commits on `develop`    | 173 since 2026-10-13                                                                                                                                                 |
| ADRs                    | 51 files (ADR-0043 to ADR-0051 in this epic)                                                                                                                         |
| Library                 | 228 modules: `geometry` 105, `model` 69, `math` 44, `io` 6, `render` 4                                                                                               |
| Tests                   | 1 211; 100 % coverage of `src/`                                                                                                                                      |
| `check:all`             | 90 s one check after the other, 54 s in four lanes (RET-002)                                                                                                         |
| Active time, by feature | F12 7 h 25, F11 2 h 36, F13 1 h 41, F10 56 min                                                                                                                       |
| Tokens, by feature      | F12 422 M, F11 178 M, F13 84 M, F10 40 M                                                                                                                             |
| Most costly commands    | `python3` edits 389 min (1 147 calls, mostly the model writing them), `cat` 198 min, `git add` with its hooks 73 min, `git push` with `check:all` 44 min (49 pushes) |
| Waiting loops           | about 1 h on the whole project, most in E01 (CI); none on CI in E14                                                                                                  |

Durations before RET-002 come partly from transcript times, which include the time spent writing each command. Calls since RET-002 are timed by the hooks, and the report splits each turn between the tools and the model.

### Full check of the epic

`npm run epic:check -- E14 2026-10-12`, run during the retrospective:

| Check                                            | Result | Detail                                                                   |
| ------------------------------------------------ | ------ | ------------------------------------------------------------------------ |
| check:all (code, site, playground, dependencies) | ok     | passed in 57 s                                                           |
| branches left                                    | ok     | the review's branch, waiting for its merge with this one                 |
| pull requests open                               | ok     | #193, the review of E14                                                  |
| issues open                                      | ok     | 0                                                                        |
| calls failed during the epic (2 % at most)       | ok     | 40 of 3 905; 37 shell commands ending in error (checks, refused commits) |
| calls left running over an hour                  | ok     | none                                                                     |
| backlog of E14                                   | ok     | every feature and story done                                             |
| private records                                  | ok     | saved                                                                    |
| golden file of refactors                         | ok     | present                                                                  |

## 2. Logged items, decided

| Theme         | Item                                                                                         | Decision                                                                                        | Where it went                                  |
| ------------- | -------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------- | ---------------------------------------------- |
| Status line   | it showed a background task after the turn; four states asked                                | done: the cause found (a refused call never notifies), four states, one line per task           | `status-tasks.ts`, `docs/tooling/metrics.md`   |
| Metrics       | every figure of the status line with each call, as complete as is cheap                      | done: start, sizes, step, subagent id, turn tokens, memory, background tasks                    | `log-tool-call.ts`, ADR-0050                   |
| Metrics       | find the slow tools; Rust if simple                                                          | measured: the time is the model writing and the checks; tools in TypeScript, Rust later         | `docs/tooling/agent-tools.md`                  |
| Agents        | see what subagents and background commands do                                                | done: status line lines, `npm run activity`                                                     | `scripts/activity.ts`                          |
| Agents        | keep the main thread free, subagents doing the work, with rules                              | tried during E15's runs, measured at its retrospective                                          | ADR-0051, run skill                            |
| Tools         | list every tool that would spare the agent's writing, and those worth Rust                   | listed: 23 tools in six groups; Rust for the hooks, the status line and the large readers       | `docs/tooling/agent-tools.md`                  |
| Context       | fewer tokens at the start of each session                                                    | planned during E02, the epic after E15                                                          | E02 epic, improvement log                      |
| Questions     | every questionnaire ends with room for remarks                                               | done                                                                                            | `CLAUDE.md`                                    |
| Shapes        | point 1 alike in every shape                                                                 | decided: rectangle from its bottom-right corner, round shape from its right point; built in E10 | E10 epic                                       |
| Retrospective | a full check of what the epic produced, at each retrospective                                | done                                                                                            | `npm run epic:check`, retro skill step 1       |
| Refactors     | the golden file as a repository script                                                       | done                                                                                            | `npm run golden:write`, `golden:check`         |
| Speed         | `check:all` at each push, `npm run fix`, commit hooks                                        | done: parallel lanes, ESLint's cache for fixes                                                  | `scripts/check-all.ts`                         |
| Speed         | Python scripts written again and again                                                       | done: `replace`, `backlog:status`, `story:finish`; specifications for the next tools            | `docs/tooling/agent-tools.md`                  |
| Lint          | `prefer-iterator-concat` writes a method Node.js 24 lacks                                    | off, with `prefer-iterator-zip`, until the Node.js LTS ships them                               | `eslint/rules/unicorn/iteration.ts`            |
| Lint          | an import named only in a comment was kept                                                   | cause found: `jsdoc/no-undefined-types` counted comment words as uses; option off, rule kept    | `eslint/rules/jsdoc/types.ts`                  |
| GitHub        | delete merged branches automatically                                                         | done                                                                                            | repository setting, ADR-0050                   |
| GitHub        | validated review and retrospective merge by themselves                                       | done, after the Product Owner's validation                                                      | `story-automerge.yml`, ADR-0050                |
| Sources       | how many serious sources must agree                                                          | tiers: one primary source, two independent ones, three for a surprising claim                   | `docs/research/README.md` §2                   |
| Watchdog      | relaunch a run stalled while the Product Owner is away                                       | kept: wanted, needs a design that wakes the agent; the status line shows a stall meanwhile      | improvement log                                |
| Later         | parallel sessions; what stays public with the paid edition; playground asking test questions | kept, with the features that need them; the playground idea to be explained better next time    | improvement log                                |
| Later         | code debt list, stretched polygon proof, backlog MCP server, fillets computed three times    | kept, with the features that need them; the MCP server once metrics show backlog reads cost     | improvement log, `docs/tooling/agent-tools.md` |
| Done earlier  | short demo for a feature without visible change; reviews grep `docs/domain/`                 | closed: already applied                                                                         | —                                              |
| Feedback      | 11 critiques of E14                                                                          | written as habits                                                                               | `docs/playbook/agent.md`                       |

## 3. What helped, what hurt

| Side          | Helped                                                                                      | Hurt                                                                                                                     |
| ------------- | ------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------ |
| Product Owner | runs that go on alone; handles typed as numbers that behave as in Figma; a simple interface | long turns with the chat blocked; a status line that was wrong again; not seeing what subagents do; slow checks          |
| Agent         | spikes before code; the golden file for F13; reviews by the expert agents                   | writing the same edit scripts again; checks one after the other; a fix to the status line made without finding the cause |

## 4. Evolvability review

| Question                          | Answer                                                                                                                         |
| --------------------------------- | ------------------------------------------------------------------------------------------------------------------------------ |
| Is the code still easy to change? | yes: folders by business domain (ADR-0043), every calculation named (ADR-0049), a golden file for refactors                    |
| Are the checks fast enough?       | better: 54 s; the slowest lanes are lint and tests, about 25 s each alone                                                      |
| Do the tools fit the size?        | the agent's tools are new; the next ones wait for their signal (`docs/tooling/agent-tools.md`)                                 |
| Fitness functions?                | one more: no named calculation written inline (`tests/named-calculations.test.ts`); the `NAM` pass checked by the backlog test |
| "Considered for later" items?     | Rust tools, a backlog MCP server, a watchdog: each with its trigger                                                            |
| Agent configuration?              | skills use the tools; experts follow the source tiers; the main thread kept free proposed (ADR-0051)                           |

## 5. Changes adopted

- ADR-0049: every calculation named, and a `NAM` naming pass closes each epic.
- ADR-0050: the agent's tools, parallel checks, validated epic stories merging by themselves, GitHub deleting merged branches, each call's record.
- ADR-0051, accepted for a trial during E15: stories worked by subagents, the main thread free.
- The status line's four states and its background lines; `npm run activity`; `npm run epic:check` at each retrospective.
- The tiers of sources in the research protocol.

## 6. Playbook updates

- Six habits added to [Working with the agent](../../playbook/agent.md#lessons-from-the-product-owner).

## 7. Best practices read for agent-built projects

| Source                                              | What it says that we use                                                                    |
| --------------------------------------------------- | ------------------------------------------------------------------------------------------- |
| W3C, SVG 2 — Basic Shapes (`REF-SVG2-SHAPES`)       | an ellipse starts at its 3 o'clock point and runs clockwise: the recommendation for point 1 |
| Wikipedia, Verifiability (`REF-WIKI-VERIFIABILITY`) | independent sources, several for an exceptional claim, no fixed number: the source tiers    |
