# E17 — Fast tooling with Oxc and Rust: what was built

## In one paragraph

Before E17, every check the agent ran before a push took 170 to 217 s under load, four helpers working at once froze the computer, and each helper started by reading about 38,900 tokens of rules and tools before doing anything. Now the tools themselves enforce the limits of a parallel run, the checks run on native tools (Oxc, TypeScript 7 and the project's own Rust), each helper starts from a short digest of about 10,600 tokens, and every hook, script and Git hook is a Rust program that answers in about 3 ms instead of about 59. `check:all` now ends in about 75 to 84 s on a quiet machine (EN-113, NAM-002), and in 106 to 113 s in this review's own runs, the computer busier. Nothing changed in the editor nor in the library, which still has no dependency.

## Feature by feature

### F18 — A safe and cheap parallel run, and E15's leftovers

At most three helpers at once and one heavy command at a time, each enforced by a tool: a fourth helper is refused by a hook, a second `check:all` waits on a lock and says who holds it. The files every branch would write (the logs, the backlog's indexes, the checklists) are written once, at merge, and the logs merge by union. E15's leftovers came with it: the font list held while the default font is read again, every font test run on every system, a checklist of at most 12 lines per feature. [Demo page](./f18-safe-and-cheap-parallel-run.md)

### F19 — Checks on Oxc, TypeScript 7 and Rust

ESLint, Prettier, TypeScript 6 and TypeDoc left; oxlint and oxfmt from the Oxc project, TypeScript 7 and a linter of the project's own in Rust took their place. Every one of the 854 rules ESLint had is accounted for: 374 run in oxlint, 205 in the Rust linter, 181 were dropped with their reason, 94 were already off. The lint went from 45 to 62 s to about 2 s; the documentation site is built only when its inputs change. [Demo page](./f19-checks-on-oxc-and-rust.md)

### F20 — A context diet for every agent

Each kind of task has its own agent type, started from the digest `project-rules` instead of `CLAUDE.md`, `AGENTS.md` and every skill: a story worker's first request went from 38,897 to 10,562 tokens. The rules were rewritten as principles, 20 % shorter; the Git sequences take one call; a page is read from the library instead of fetched again; a run works to the scope you set and stops at 90 % of the week's usage. [Demo page](./f20-a-context-diet.md)

### F21 — Agent tools and the library server in Rust

Every tool the project wrote for itself — the Claude Code hooks, the status line, the scripts, the Git hooks, the library's server, the dependency checks, the secret scan and the release — is now a Rust binary of the project's workspace. `guard-bash`, run before every shell command, went from 58.8 to 3.0 ms; the dev packages from 38 at E17's start to 13; a release is prepared on a branch of its own and published by the project's tool. [Demo page](./f21-agent-tools-in-rust.md)

### After the features: NAM-002, EN-114, EN-115

- **NAM-002, the naming pass and the epic's full audit**: the dates written in nine crates moved into one crate, `tools/calendar`; the Git calls written fifteen times into `tools/git-repository`; the cooldown and the hash functions named after their source. `check:all` went from 106.0 to 83.9 s across the story, most of it from the load, not claimed. Six findings were logged for later, each with its reason ([NAM-002](../backlog/stories/E17-NAM-002-naming-pass-e17.md)).
- **EN-114, the records kept on the computer**: GitHub refused the push of the private records, one transcript being 238 MB. They now stay in a local folder you chose, shared by every project, one folder per repository, never sent nor backed up (ADR-0081).
- **EN-115, fewer npm scripts**: `package.json` went from 52 scripts to 26, those a person runs (`dev`, `build`, `fix`, `check:all`, `test`…) and those something calls by name; the agent calls every other tool by its binary, `target/release/<binary>` (ADR-0082).

## The figures, before and after

| What | At E17's start | Now | Where it was measured |
| --- | --- | --- | --- |
| `check:all` | 170 to 217 s under load (57 s at E14) | 74.6 s (EN-113), 74.9 s (VAL-022), 83.9 s with the site built (NAM-002); median 73 s over its 64 runs under the heavy lock; 112.6 s and 105.9 s at this review, the site built, load average 9 to 49 | the stories, `target/release/metrics`, this review |
| a story worker's first request | 38,897 tokens (`general-purpose`) | 10,562 tokens (`story-worker`) | EN-065, VAL-021 |
| `guard-bash`, before every shell command | 58.8 ms | 3.0 ms | SP-023, VAL-022 |
| the lint | 45.6 to 62.0 s (ESLint) | 1.75 s (oxlint and the Rust linter) | SP-021, VAL-020 |
| dev packages | 38 | 13 | VAL-022, this review |
| npm scripts | 52 | 26 | EN-115, this review |
| the pages an agent reads | 182,100 tokens | 144,900 tokens (−20 %) | VAL-021 |
| helpers at once | up to 4, the computer frozen | 3 at most, the fourth refused | F18 |

The tokens per story, against E15's 24 million, are compared at E17's retrospective (RET-004).

## Words you will meet

| Term | In plain words | Where it shows |
| --- | --- | --- |
| subagent, worker | a helper the agent starts for one task, with its own context; a story worker codes one story | the chat, during a run |
| worktree | a second working copy of the repository on its own branch, so that several stories are worked at once | `.claude/worktrees/<story>` |
| heavy command | a command that keeps every core busy for minutes: `check:all`, `npm run test`, a push | the waiting line of the lock |
| `check:all` | every check of the repository, run before each push: format, lint, types, tests, Rust tools, site | `npm run check:all` |
| lint | a program that reads the code and reports what breaks a rule | `npm run lint` |
| formatter | a program that rewrites the code and the pages in one layout | `npm run fix` |
| Oxc | a set of JavaScript and TypeScript tools written in Rust: oxlint, oxfmt, a parser | `npm run lint`, `npm run fix` |
| Rust | the language every tool of the project's own is now written in; its programs are compiled once and start in milliseconds | `tools/`, `target/release/` |
| binary | a compiled program ready to run, called by its path | `target/release/<binary>` |
| hook | a small program run before or after each of the agent's calls, or at each commit and push | `.claude/settings.json`, `.githooks/` |
| token | the unit the model reads and the usage is counted in | the status line during a run |
| first request | what a helper carries before it does anything, sent again with each later request | `target/release/context-size` |
| dev package | a package of npm installed for the tooling, never shipped with the library | `package.json`, `devDependencies` |
| records | the agent's transcripts, metrics and library, kept private | a folder of your computer (ADR-0081) |

## User test of the epic

A story's life crosses all four features. This review's own story, REV-004, walked it on 2026-10-18; you can walk it with any story a run works.

| Step | Do | You should see | OK? |
| --- | --- | --- | --- |
| 1 | the main thread makes the story's worktree (F18) | `.claude/worktrees/<story>`, on the story's branch | yes |
| 2 | the worker starts (F20) | a `story-worker` of about 10,500 tokens, the digest and the brief preloaded | yes |
| 3 | `target/release/git-sequences sync` (F20, F21) | the story rebased on its branch, the binaries rebuilt, in one call | yes |
| 4 | `npm run fix` (F19) | the pages and the code formatted, the lint and the type errors listed, in a few seconds | yes |
| 5 | `target/release/git-sequences commit -F <message> <files>` (F20, F21) | the commit made through the Git hooks of `.githooks/`: format, lint, message, secret scan | yes |
| 6 | the push (F18, F19, F21) | `check:all` under the heavy lock, every lane green, the time of each lane printed | yes |
| 7 | the worker hands back (F20) | `check-handback: <story> on <branch> pushed at <hash>; …` above its report | to see at the hand-back |
| 8 | `target/release/metrics`, section Tools (F21) | each tool's calls and times | yes |

## Decisions taken on the way

- **A run** (ADR-0072, after ADR-0063, removed: ADR-0101): three helpers and one heavy command at a time, enforced by a hook and a lock; a tooling feature is validated by the agent.
- **Checks by level** (ADR-0064): no review per story, one thorough audit per feature, a full audit beside the naming pass.
- **Shared files** (ADR-0065, ADR-0068): the files every branch writes are written at merge; the logs merge by union; the tables are written without padding.
- **A register of exceptions** (ADR-0066): every place the code departs from a rule is listed with its reason, and checked.
- **The release** (ADR-0067, then ADR-0079, both removed: ADR-0101): now prepared on a branch `release/x.y.z` from `develop` by the project's tool, merged into `main` by you, tagged and published by the tool; release-please removed.
- **Fast tools** (ADR-0069): a tool is as fast as it can be, since every second the agent waits costs it rounds of checking again.
- **Tests without over-testing** (ADR-0070): one claim proven in one place; slow preparation done once.
- **Type errors with the lint** (ADR-0071): the type errors come listed with the lint; the declarations are written by Oxc.
- **Every new tool in Rust** (ADR-0073); **the agent shapes its own method, you decide** (ADR-0074).
- **The toolchain on Oxc** (ADR-0075): oxlint, the project's Rust linter, oxfmt and TypeScript 7 replace ESLint, Prettier and TypeScript 6.
- **Ported code under its licence** (ADR-0076): code ported into the tools keeps its licence, named in `NOTICE`; never in the library.
- **The library's server in Rust** (ADR-0077): on `rmcp`, its file named by `AGENT_LIBRARY_FILE`.
- **The tools called by their binary** (ADR-0078, ADR-0080): 3 ms a call instead of 47 to 150 ms through `cargo run` or `npm run`; the Git hooks live in `.githooks/`.
- **The records kept on the computer** (ADR-0081): one local folder shared by every project, never sent.
- **The npm scripts kept for a person or a caller** (ADR-0082): 26 scripts; the agent calls the rest by their binary.

## Limits and next epic

- E17's checklist (`?checklist=E17`) is kept as validated: 14 of its 48 lines name an npm script and one names release-please's pull request, as they were when tried; EN-115 later moved those tools to their binaries, `target/release/<binary>`, listed in [the agent's tools](../tooling/agent-tools.md) (ADR-0082).
- The release tool has not published yet: 0.3.0 will be its first run on `main`, after the retrospective (Review, below).
- `check:rust` is the longest lane of `check:all`: its coverage could run in a lane of its own (an improvement row of NAM-002).
- Left for later with their reason: the hooks' own JSON reader, the guard's text matchers, two readers of version numbers, five readers of a story id, the library's claims tested at two levels (NAM-002). They wait, with this review's findings, for the tools point at an epic's end.
- Vitest, Vite and VitePress stay JavaScript, by decision; `oxlint-tsgolint` is written in Go.
- Next: the retrospective of E17 (RET-004), then E02, editing shapes with precision.

## Review

Held with the Product Owner on 2026-10-18 (`REV-004`): the agent ran the checklist again first, then the Product Owner answered the questions it prepared.

**The checklist run again by the agent** on `develop`, line by line in [REV-004](../backlog/stories/E17-REV-004-review-e17.md), section E17's checklist run again: every tool works and every check is green; no tool was changed during the review.

| Result | Lines |
| --- | --- |
| ok | 26, among them the lint and its probes, the font tests, the heavy lock, the guard's refusals, the dev packages, the secret scan |
| ok by their tests, needing Chrome or a run | 6 |
| an npm script EN-115 removed: the binary works, the line names the old command | 14, and one expecting the removed `font:metrics` |
| obsolete | 1: release-please's pull request, replaced by ADR-0079, removed since (ADR-0101) |

Also found: `epic-check` takes a dropped story for one to look at; `session-report` writes `1617 min 60 s`; the main checkout's hooks ran binaries built before EN-115's merge; `check:all` took 112.6 s and 105.9 s in this review's runs, the site built and the machine loaded, and 82.3 s at its push.

**The answers.**

| Question | Answer (Product Owner, 2026-10-18) |
| --- | --- |
| `check:all` against the 70 to 90 s target | met: its median, about 73 s, is within the target; the runs of 106 to 113 s come from the load |
| the closing release | after RET-004, which weighs the Product Owner's idea of a tag that creates the version without a new commit; 0.3.0 is then published with the flow chosen, and E17 closes at that publication. ADR-0079 stands until then, removed since (ADR-0101) |
| the next epic | E02, editing shapes with precision, after RET-004 |
| NAM-002's logged tooling items and this review's findings | they wait for the tools point at an epic's end: tools are not fixed during an epic's features |
| anything built that was not meant | nothing; no remark on the report; the records' private GitHub repository is left for the Product Owner to delete |
| the decisions to confirm | confirmed: the tools called by their binary's path (ADR-0078), the story worker on Opus (EN-066), `AGENT_LIBRARY_FILE` (ADR-0077), one Git command let through when followed by read-only filters (EN-104), a tooling feature validated by the agent (ADR-0072); told: ADR-0080, ADR-0081, ADR-0082 |
| the permissions for the `story-finish` and `mutate` binaries, and the story auto-merge into `develop` | parked until the next review (ADR-0086: no epic has a retrospective) |
| E17's checklist lines naming removed scripts | kept as records of what was validated, with the note under Limits; the checklist and its test unchanged (the agent's decision) |

**Closure criteria.**

| Criterion | Met? | Evidence |
| --- | --- | --- |
| ESLint and its plugins removed | met | no ESLint package nor configuration left; its 854 rules accounted for (ADR-0075) |
| `check:all` under the duration set at the refinement, about 70 to 90 s most of the time | met | a median of about 73 s over the recorded runs; 74.6 s (EN-113), 74.9 s (VAL-022), 83.9 s with the site built (NAM-002), 82.3 s at this review's push |
| tokens per agent input measured lower | met | a story worker's first request 38,897 → 10,562 tokens; a spike 38,339 → 11,971; an audit 15,477 → 8,505 (VAL-021) |
| the release that closes the epic | not yet | 0.3.0, published after RET-004 with the flow it chooses; E17 closes then |

The retrospective of E17 (RET-004) follows.
