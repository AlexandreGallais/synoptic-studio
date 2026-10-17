---
id: EN-096
epic: E17
feature: F20
title: The agent's rules and skills written as principles
status: ready
points: 3
---

# E17 · F20 · EN-096 — The agent's rules and skills written as principles

Asked by the Product Owner on 2026-10-17: many rules an agent reads were written close to how a decision was first explained in conversation — its examples, its order, its wording — so an agent has to infer the principle and may doubt where it applies. Each rule is rewritten to state the idea in general terms: the principle, its reason, how to tell a case, what to do on conflict, with an example only where it removes a doubt. The agent is critical of how a decision was explained, gives a wider range of examples when they help, and leaves out what does not matter: shorter docs that serve the reader beat exhaustive ones. Split at F20's refinement into one story per family of pages, each measured in tokens before and after (2026-10-17). A later idea, not for these stories: keep superseded ADRs out of the agent's context.

This story covers `AGENTS.md`, `CLAUDE.md`, `.claude/rules/`, `.claude/skills/`, `.claude/agents/` and the worker brief.

## Acceptance criteria

- Given each page of `AGENTS.md`, `CLAUDE.md`, `.claude/rules/`, `.claude/skills/`, `.claude/agents/` and the worker brief, when the pass ends, then every rule reads as a principle with its reason and its limits; dated decision notes stay as the record of when and by whom, never as the rule's body.
- Given an accepted ADR whose wording follows a conversation too closely, when a page applies it, then the ADR is left as written and the page states the principle.
- Given a page or a section that guards nothing an agent needs, when the pass finds it, then it is cut, not rewritten.
- Given the pages before and after, when they are compared, then their length in tokens is written in the story, and no rule was lost: each old rule is traced to its new sentence.

## Tasks

One task = one commit, referenced as `EN-096.Tn`.

- [x] T1 — Measure the pages in tokens and list their rules (1 h)
- [ ] T2 — Rewrite the pages as principles, each old rule traced (4 h)
- [ ] T3 — Measure again and record the figures (1 h)

## Measures

The agent's rules and skills, measured on 2026-10-17 before T2 and after it. Tokens are each page's characters divided by **2.30 characters per token**, the ratio EN-097 measured the same day on Markdown of the same kind (a page printed whole into a worker, the growth of the next request's input read from its transcript), rounded to the hundred.

| Page | Words before | Tokens before |
| --- | --- | --- |
| `AGENTS.md` | 2,042 | 5,600 |
| `CLAUDE.md` | 532 | 1,400 |
| `.claude/rules/docs.md` | 161 | 400 |
| `.claude/rules/library.md` | 1,088 | 3,000 |
| `.claude/rules/tooling.md` | 993 | 2,800 |
| skill `audit` | 384 | 1,100 |
| skill `derivation` | 172 | 500 |
| skill `project-rules` | 707 | 2,000 |
| skill `retro` | 825 | 2,300 |
| skill `review` | 506 | 1,400 |
| skill `run` | 3,270 | 8,700 |
| skill `spike` | 285 | 800 |
| skill `story` | 730 | 2,100 |
| skill `triage` | 199 | 500 |
| skill `worker-brief` | 1,291 | 3,600 |
| agent `auditor` | 379 | 1,100 |
| agent `clerk` | 416 | 1,100 |
| agent `doc-editor` | 354 | 900 |
| agent `functional-analyst` | 477 | 1,300 |
| agent `math-expert` | 518 | 1,500 |
| agent `researcher` | 306 | 800 |
| agent `story-worker` | 288 | 800 |
| agent `verifier` | 395 | 1,000 |
| **all pages** | **16,318** | **44,900** |
| the main thread's instructions, `CLAUDE.md` and `AGENTS.md` | 2,574 | 7,000 |

First request of each agent type, measured by `npm run context:size` on real tasks before the pass:

| Agent type | Preloaded | First request, tokens | Measured on |
| --- | --- | --- | --- |
| `story-worker` | `project-rules`, `worker-brief` | 10,694 (median of 7) | EN-066, EN-086, EN-087, EN-096 to EN-099, 2026-10-17 |
| `researcher` | `project-rules`, `worker-brief`, `spike` | 11,201 | EN-065's measure (`docs/tooling/agent.md`) |
| `clerk` | `project-rules` | 6,901 | EN-065's measure |
| `auditor`, `verifier`, `doc-editor`, `math-expert`, `functional-analyst` | `project-rules` | 7,608 for the `auditor` | EN-065's measure |

## Rules before the pass

Each rule of the pages, by page, with an id the trace of T2 uses.

- `AGENTS.md`: AG1 the project and its tools; AG2 the state of each epic; AG3 the scope: a free V1, a request that looks pro is said and asked, pro ideas in `editions.md`; AG4 only `ready` stories, `/run` is the go-ahead and the agent sets its scope `ready`; AG5 where to read before any task; AG6 English everywhere, messages to the Product Owner in French; AG7 every business rule in `docs/domain/`, in the same change, ambiguity asked; AG8 never quote the Product Owner; AG9 a structural decision is a new ADR, an accepted one superseded; AG10 the pipeline; AG11 functional core, imperative shell, no argument modified; AG12 one export per file, no default nor optional parameter; AG13 every calculation named; AG14 functions follow the domain, a limit that cuts a context is wrong, a rule kept only if it makes code more correct or uniform; AG15 one `@kind`, one verified `@see`, austere TSDoc, limits per kind; AG16 never invent a reference; AG17 no personal information; AG18 tooling as fast as it can be; AG19 only the agent writes changes; AG20 numbers; AG21 geometry; AG22 portable core; AG23 contours clockwise, polygon start, radius reduced locally; AG24 no new tool in TypeScript; AG25 no runtime dependency, a dev package's conditions; AG26 no freehand, Bézier only from handles, the SVG primitives, no native stroke; AG27 no `any`, `!`, `as` without a directive, no class nor `enum`, no rule disabled to pass; AG28 never set `ready` outside a run, merge a pull request, push to `main`, `--no-verify`, force push without lease; AG29 a feature's flow; AG30 an epic's flow; AG31 branches; AG32 merges; AG33 three subagents, one heavy command, shared files at merge, checks by level; AG34 checks local, one CI; AG35 a story's commits and pull request; AG36 tests first, a criterion's test title; AG37 to AG40 the definition of done (fix, `check:all` seen green, docs, French summary); AG41 a file-reading test mutation-checked; AG42 the stop table; AG43 the stop format; AG44 the commands; AG45 a refused commit leaves its files staged, a subject starting with an id refused; AG46 `develop` brought up to `main` after a release; AG47 `RELEASE_PLEASE_TOKEN` expires 2026-12-31; AG48 Node.js scripts name `.ts` imports and need Temporal's flag.
- `CLAUDE.md`: CL1 the skills by need; CL2 the expert agents; CL3 sources through the library, the web only for a page it cannot read; CL4 the questionnaire (options with numeric examples, no drawing in characters, a last free question, a refinement in two rounds); CL5 `npm run activity` and the status line; CL6 the current step in `.git/metrics/step`; CL7 a command over a minute in the background with Monitor, no waiting loop, a red call looked at; CL8 during a run the main thread only dispatches, each comment acknowledged, a heartbeat line; CL9 three subagents and one heavy command, `limit-agents` and Claude Code's cap; CL10 a worktree named after its story; CL11 silence between tool calls during a run; CL12 the session report at a session's end; CL13 the chat markers; CL14 a behaviour explained by its situation; CL15 the journal and `records:save` before a pause; CL16 `guard-bash` and no workaround; CL17 the area rules.
- `.claude/rules/docs.md`: D1 style page, English; D2 `|` escaped in tables; D3 no double curly braces; D4 front-matter values with `: ` quoted; D5 folders linked as `./folder/`, templates never linked; D6 a backlog status in three places; D7 a reference's row before it is cited; D8 derivations' scope and body lines; D9 a local address in code font.
- `.claude/rules/library.md`, its bullets in order: Li1 import a folder; Li2 a module in its domain's folder; Li3 a new layer; Li4 the `export type *` barrel pitfall; Li5 `noUncheckedIndexedAccess` and a testable fallback; Li6 module constants and type members documented; Li7 JSDoc sentences and fragments, `@kind`; Li8 procedure verbs and limits in `tools/lint`; Li9 tests: examples, properties, `toBe` and `-0`; Li10 DOM tests' environment; Li11 `npm run fix` before each commit; Li12 zero vectors and signed zeros; Li13 more than three parameters grouped; Li14 the `[hidden]` rule; Li15 a cast in a test of a string union, `?.` and `??` in complexity; Li16 a mutation by hand restored by copy; Li17 happy-dom's `:checked`; Li18 the playground's limits and splits; Li19 `data-*` deleted by index; Li20 `expect.closeTo` in objects; Li21 scaling multiplies first; Li22 a decision on a paused branch; Li23 a control tested on a refused shape; Li24 a control writes only its value; Li25 frames and dashes as fills; Li26 a required field added to a model type; Li27 a bisection's last unit; Li28 a clamp before a bisection; Li29 immutability read from the written code; Li30 happy-dom's moved `<select>`; Li31 a shared math function's floats unchanged, the golden check; Li32 happy-dom's `<textarea>` line break; Li33 `golden:write` only by the main thread.
- `.claude/rules/tooling.md`, its bullets in order: To1 TypeScript 7 alone, VitePress 2.0 alpha; To2 oxlint and `tools/lint`, oxfmt; To3 a new dev package's conditions; To4 an oxlint upgrade; To5 `import.meta.dirname`; To6 a file-reading test mutation-checked; To7 explicit check targets; To8 a tooling change with its files' fix; To9 every new tool a Rust crate and how it is built and run; To10 Node.js hooks and scripts; To11 Stylelint not installed; To12 `guard-bash` and here-documents; To13 a remote branch deleted through the API; To14 Vite's `?raw` types; To15 commit scopes a closed list; To16 a body line read as a footer; To17 timing tests; To18 branch names; To19 no force push on a feature branch; To20 zsh word splitting and loops; To21 no polling of a subagent, background tasks stopped before a stop message; To22 `GIT_*` removed in Git tests; To23 a replacement text returned by a function; To24 ported code and its licence.
- skill `project-rules`: PR1 the digest of AGENTS.md and CLAUDE.md, each line traced to the AGENTS.md rule it repeats (the repository, Always, Code, Git, the stop table).
- skill `worker-brief`: WB1 who preloads it; WB2 the worktree from `worktree:new`, read the brief inside it; WB3 every Bash call starts with `cd`, absolute paths, no `git -C`; WB4 one Git command per call, `git:sync` and `git:commit`; WB5 a scratch folder; WB6 what to read first; WB7 code rules (one export, domains, `@kind`, `@see`, named calculations, reuse); WB8 new tools in Rust and how; WB9 tests first, property tests, 100 % of `src/`; WB10 edge cases and mutations; WB11 commits; WB12 the step file; WB13 `npm run fix` and the touched tests, no Python edits; WB14 docs in the same task; WB15 the shared files and the rows for the logs; WB16 no `golden:write`; WB17 heavy commands and the lock, never by hand; WB18 single test files between heavy commands; WB19 push, no force; WB20 stop before the pull request and the `done` commit; WB21 a rule change brings its docs, or `Rule-docs:`; WB22 a business question stops the story, not the run; WB23 the final report and `check-handback`.
- skill `run`: RU1 a silent run; RU2 never without the Product Owner's start, the scope set by them as a time or an end, no bound of the run's own, the default scope; RU3 the order guard; RU4 refinement in two rounds; RU5 `epic:check` for an epic's first feature; RU6 to RU11 before the first story (go-ahead and `run:start`, journal, feature branch, triage, the stories listed and a blocked one parked, the progress page); RU12 one worker per story, of its task's type, the prompt; RU13 the main checkout on `develop`, rules read in a worktree; RU14 a background Git command checks its branch; RU15 a rebase conflict resolved in the file; RU16 three subagents, one heavy command; RU17 parallel workers on separate files; RU18 heartbeat; RU19 the main agent's part; RU20 business questions asked by the main agent; RU21 only dispatch, each comment acknowledged, tool comments to another agent; RU22 to RU30 for each story (pick up `develop`, worktree, the worker stops before `done` and the main thread makes it, test card, checkpoint, checks by level, `story:finish` and the progress page, statuses, next); RU31 a question parks its story, its five steps; RU32 pushes: what, never what, the chat holds the question; RU33 the four stops; RU34 at the stop: `run:end`, journal, message; RU35 to RU43 validation (worktree, preparation and the checklist, how to launch, the Product Owner's test, last commit, clean-up then ask, the feature pull request and a tooling feature, the release, after the merge).
- skill `story`: ST1 may it start; ST2 read; ST3 worktree from the feature branch; ST4 the T0 commit; ST5 per task: tests first, implement, fix, commit; ST6 docs and a test card; ST7 the `done` commit and the rule's docs; ST8 push and pull request, in and outside a run; ST9 no CI; ST10 stop and summary; ST11 after the merge.
- skill `audit`: AU1 the checklist and Findings table; AU2 checks by level; AU3 to AU12 its ten steps.
- skill `derivation`: DE1 to DE6 its six steps.
- skill `retro`: RE1 to RE15 its fifteen steps.
- skill `review`: RV1 to RV6 the feature demo; RV7 to RV11 the epic review.
- skill `spike`: SP1 to SP7 its seven steps.
- skill `triage`: TR1 to TR6 its six steps.
- agents: AT1 each type's role, tools, model and reason (front matter, first paragraphs); AT2 `auditor`'s rules (read-only, re-derive, recompute, re-open, provenance, duplicates, unnamed calculations, consistency, findings only on correctness); AT3 `clerk`'s rules; AT4 `doc-editor`'s reading and return; AT5 `functional-analyst`'s reading, method and return; AT6 `math-expert`'s reading, method and return; AT7 `researcher`'s three steps; AT8 `story-worker`'s three steps; AT9 `verifier`'s reading, method and return; AT10 the Sources section, the same in `auditor`, `functional-analyst` and `math-expert`.
