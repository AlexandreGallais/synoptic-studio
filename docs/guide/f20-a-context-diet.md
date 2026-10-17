# F20 — A context diet for every agent

## What changed for you

A run is the main thread handing each story to a subagent, a worker of its own, which works it in a worktree and hands it back. Every request a worker makes sends again everything it carries since its start, read from Claude's cache: at F20's start, cache reads were 78 % of what the subagents cost (research note 0024). F20 cut what each agent carries and how many calls it makes: each kind of task has its own agent type, started from a short digest of the rules instead of `CLAUDE.md`, `AGENTS.md`, the memory index and every tool and skill; the rules are written as principles, shorter; the Git sequences take one call; a page is read from the library instead of fetched again. For you, a run now works to the scope you set, parks a question instead of stopping, pushes a notification only for a question or a blocker, stops at 90 % of the week's usage, and shows its duration and tokens on the status line. Nothing changes in the editor.

## The words

| Word | In plain words |
| --- | --- |
| subagent, worker | an agent the main thread starts for one task, with its own context; a story worker codes one story |
| agent type | a file of `.claude/agents/` that sets a subagent's tools, model and what it is given at its start |
| first request | the first call a subagent makes to the model: what it carries before it does anything, sent again with every later request |
| token | the unit the model reads and the usage is counted in; a page of these docs is about 2.3 characters a token |
| cache read | the part of a request the model has already seen, read again at a lower price; most of a subagent's tokens |
| hand-back | the report a worker gives the main thread when it ends |
| the library | the project's store of the pages read, served by its own tool, which keeps each page's text to cite it |

## Each agent starts lighter

Measured by `npm run context:size` on the run's own transcripts (EN-065, then VAL-021 on 2026-10-18). Before, every worker was `general-purpose`: it received every tool's definition, the listing of every skill (26,805 characters), `CLAUDE.md`, `AGENTS.md` and the memory index. Now each task kind has its type, given only its tools and the digest `project-rules`, plus the worker brief for those that code.

| Task kind | Before: agent type, first request | After: agent type, first request |
| --- | --- | --- |
| a story | `general-purpose`, 38,897 tokens (median of 74, F18 and F19) | `story-worker`, 10,562 (median of 12, F20) |
| a research spike | `general-purpose`, 38,339 (SP-021) | `researcher`, 11,971 (SP-023) |
| an audit | `auditor` 15,477 and 15,655, `verifier` 13,994 and 14,192 (AUD-020, AUD-021) | `auditor` 8,505, `verifier` 7,099 (AUD-022) |
| a review of pages | `doc-editor`, 13,865 | `doc-editor`, 6,146 (measured by EN-065) |
| logs, backlog records | `general-purpose`, 38,839 to 39,419 | `clerk`, 6,901 (measured by EN-065) |

A story worker carries about 28,300 tokens less at each of its requests: on a story of 70 requests, about 2 million tokens less read from the cache. The memory index came with `CLAUDE.md`, not on its own: the few rules a worker needs from it are in `project-rules`.

## A model per agent type

Each agent file names its model and why (EN-066). The `clerk` copies given facts into set formats, nothing to derive or judge, so it runs on Sonnet, at half of Opus's price; every other type runs on Opus, since it codes, computes expected values or judges sources. The session report gives the tokens per model.

The trial (CHK-021): EN-098 was worked on Sonnet beside EN-097, EN-096 and EN-099 on Opus, four stories rewriting pages as principles. EN-098 cost $0.17 against $2.41 to $3.09, but its three pages weighed 9 to 15 times less, its output was under-recorded, it edited pages by Python scripts, which `project-rules` forbids, and it counted tokens another way. **Outcome: the story worker stays on Opus**; a fair trial works one story twice from the same base, once per model, and audits both. A decision for you to confirm.

## The rules, shorter

Four stories rewrote the pages an agent reads as principles: each rule its idea, its reason and its limits, an example only where it removes a doubt, what guarded nothing cut, every old rule traced to its new sentence (EN-096 to EN-099). Tokens are characters divided by 2.30, the ratio EN-097 measured on these pages.

| Pages | Before | After the story | Today (2026-10-18) |
| --- | --- | --- | --- |
| rules and skills: `AGENTS.md`, `CLAUDE.md`, `.claude/` (EN-096) | 44,900 | 39,500 | 39,300 |
| the conventions, `docs/conventions/` (EN-097) | 78,300 | 66,000 | 65,900 |
| the process, `docs/process/` without its logs (EN-098) | 5,200 | 4,900 | 4,900 |
| the tooling docs, `docs/tooling/` (EN-099) | 53,711 | 39,330 | 34,900 |
| in all | 182,100 | 149,700 | 144,900 (−20 %) |

The tooling docs shrank again when the progress page was removed. A worker also stops reading the two records of the conventions, `rules.md` and `rule-catalogue.md` (33,400 tokens), which their index now sets apart. `AGENTS.md` and `CLAUDE.md`, which the main thread reads at every start, went from 7,000 to 6,300 tokens.

## Fewer calls

- **The Git sequences in one call** (EN-100): `npm run git:sync` fetches the feature branch and rebases the story on it; `npm run git:commit -- -F <message file> [--push] <files>` stages, commits and pushes. Each stops at the first failing step with Git's own message, and never pushes `develop`. Each call saved is about 3 s of the model and, in a subagent, about 166,000 tokens read from the cache (note 0024). In F19 a story worker made 1 to 32 Git calls, median 12; in F20, 1 to 7, median 4 (the stories differ in size: an order of magnitude, not a measure of the tool alone).
- **Every page read through the library** (EN-101): a `WebFetch` or a `curl` of a page is refused with the `library_get` call to make, unless the library holds that page empty or its site refused it. Reading a stored page went from 6 s for a `WebFetch`, which returned a summary that cannot be cited, to 0.1–0.2 s for `library_get`, which returns the page's own text by sections. The library records which project asked, so that it can serve several.

## A worker's hand-back checked

A worker that ends with its branch not pushed, or whose report leaves a commit unnamed, is refused once and told what is missing (EN-088). Once it is complete, the main thread gets one line above the report: `check-handback: <story> on <branch> pushed at <hash>; next: npm run story:finish -- <feature branch>`. During F20's run, nine hand-backs went through with that line; two were refused, both by a bug the audit fixed (it counted the commits from the whole history).

## A run's totals and its stops

- **Its totals** (EN-092): `/run` opens a marker with its start; the status line shows `run <duration> · <tokens> tokens · week <n> % used`, counted from the run's start, subagents included, across compactions and sessions, and `npm run metrics:session -- --run` gives the same with each kind of token. At this validation: `run 2 h 35 · 191,734,902 tokens · week 58 % used`. The message at a run's end holds its duration and tokens, nothing else.
- **Its scope and its stops** (EN-086): you set the scope at the start, a time or an end; the agent sets no bound of its own. A question parks its story, written in its file, is pushed once, and the run goes on with independent stories. A run stops at its scope's end, on a blocker it cannot get around, when you say so, or when the week's usage reaches 90 %: `npm run run:should-stop` prints `stop` from 90 % and `go` below, so that the margin stays yours. A merge or a plain stop is never pushed.

## Tried, then dropped

- **A progress page** (EN-087): a claude.ai page showing the run's tree, statuses, durations, agents, tokens and commits, updated after each story. Built and seen on a live run, then dropped on 2026-10-17 as not useful: the run's duration and tokens stay on the status line and in the stop report. Its tool and its steps were removed; criterion 5 of F20 is struck.
- **A one-hour cache for subagents** (EN-064): the main thread already has it, and on subagents it would cost more than it saves. Dropped at the refinement.

## What the audit found

AUD-022 had two independent reviews (auditor, verifier); each finding was reproduced before it was fixed, each new test made to fail once.

| Finding | Outcome |
| --- | --- |
| the hand-back check took a worktree's folder for its story and counted commits from the whole history | fixed: the story read from its branch, the commits counted from the feature branch |
| `git:commit --push` on `develop` pushed it | fixed: refused |
| the page hook missed or wrongly refused some commands (a quoted address cut at `&`, an installer piped to `sh`, a link check); agents without the library were told to use it | fixed: each case tested; such agents name the page to their caller |
| the page hook ran on every shell call, through Cargo, with a 600 s default timeout | fixed: run only on commands naming `curl` or `wget`, timeout 30 s |
| no Rust test tagged with an F20 criterion; EN-098 counted tokens another way | fixed: tagged tests; EN-098's figures restated at 2.30 characters a token |
| the run's totals with a cache, a stale tally or a second session, and a here-string that hid a later `curl` from the page hook, untested or wrong | fixed, each a test |
| two mutations survived the spot-checks | each now caught by a test |

Deferred to F21: duplicate test helpers across crates, one shell reader for both hooks, the library server's port. Left as designed: a page the library returned short goes through, the escape for a page built by JavaScript.

## Each criterion and its evidence

Proven again on 2026-10-18 by VAL-021: its story holds the command and what was seen for each.

| Criterion | What you get | Evidence |
| --- | --- | --- |
| AC1 an agent type per task kind, measured | a story worker starts at about 10,500 tokens instead of 38,900 | `tools/hooks/tests/context.rs`, `npm run context:size` |
| AC2 a model per type, the trial | `clerk` on Sonnet, the others on Opus; tokens per model in the session report; the story worker kept on Opus | `tests/agent-models.test.ts`, `scripts/session-report.test.ts`, CHK-021 |
| AC3 the hand-back checked | refused once if not pushed or a commit unnamed; a line to finish the story | `tools/hooks/tests/handback.rs` |
| AC4 the run's totals | on the status line and in `metrics:session -- --run`, across compactions and sessions | `tools/run-totals/tests/` |
| ~~AC5 a progress page~~ | dropped on 2026-10-17 | — |
| AC6 a run to its scope | a question parked, pushes for questions and blockers only, a stop at 90 % of the week | `tools/run-totals/tests/stop.rs`, `tests/backlog.test.ts` |
| AC7 one call per sequence, pages through the library | `git:sync`, `git:commit`; `curl` and `WebFetch` sent to `library_get` | `tools/git-sequences/tests/`, `tools/hooks/tests/pages.rs` |
| AC8 the pages as principles | −20 % tokens, every rule traced | `tests/backlog.test.ts` |

## How to try it

Run `npm run dev` and open `http://localhost:5173/?checklist=E17`: E17's checklist, F20's 12 lines after F19's, each a command to run in a terminal at the repository's root, or something to look at during a run.

| Try | You should see |
| --- | --- |
| `npm run context:size -- ~/.claude/projects/<this project's folder>/<a session's id>/subagents` | the first request per agent type: about 38,900 for `general-purpose`, about 10,500 for `story-worker` |
| `grep '^model:' .claude/agents/*.md` | `clerk` on `sonnet`, the seven others on `opus` |
| a story worker's report, in the chat during a run | `check-handback: <story> on <branch> pushed at <hash>; next: …` above it |
| the status line during a run | `run <duration> · <tokens> tokens · week <n> % used` |
| `npm run metrics:session -- --run` | the run's duration and tokens by kind |
| `npm run run:should-stop` | `go: week <n> % used, the limit is 90 %` |
| `.claude/skills/run/SKILL.md`, A question parks its story, Pushes, Stop when | what parks, what is pushed, when a run stops |
| `npm run git:sync` in a story's worktree | the story rebased on its feature branch, in about 1 s |
| `npm run git:commit -- -F <message file> <files>` in a story's worktree | `<id>: <hash> <subject> on <branch>` |
| ask the agent to `curl` a page the library holds | refused, with the `library_get` call to make |
| `npm run library -- stats` | the uses grouped by project |
| `cat docs/conventions/*.md \| wc -m` | about 151,600 characters, 65,900 tokens, against 78,300 before |

## Limits

- The library records the project once F20 is on `develop` and its server restarted: until then the running server is `develop`'s copy, and its uses read `(not recorded)`.
- The push on the phone waits for your setting (**Push when Claude decides** in `/config`, [agent configuration](../tooling/agent.md#pushes)); without it, a question is still asked in the chat and written in its parked story.
- The Sonnet trial did not settle the question; F21 or a later run can work one story twice to settle it.
- `npm run metrics:session` without `--run` fails in a worktree, its transcripts' folder taken from the worktree's path; the scripts and hooks still in TypeScript, the `library` server, a log of each tool's use and a reader of one row or section are F21's.

## Validation

Validated by the agent on 2026-10-18 (`VAL-021`), F20 being a tooling feature (ADR-0072): each of its seven criteria left proven again by its tests and its command, and F20's 12 checklist lines tried. The Product Owner may try the lines that concern them, and confirm keeping the story worker on Opus.

| Remark | What was done or where it went |
| --- | --- |
