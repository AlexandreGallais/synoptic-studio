---
id: E17
title: Fast tooling with Oxc and Rust
status: done
---

# E17 — Fast tooling with Oxc and Rust

Decided by the Product Owner on 2026-10-16, at E15's retrospective (RET-003): the agent's checks and tools are made fast now, before E02, rather than later. The epic starts with research; nothing is decided before it. Its refinement with the Product Owner comes next, in two rounds.

| Field | Content |
| --- | --- |
| For | the Product Owner and the agent |
| who | wait for checks and pushes, and pay in tokens for every input an agent reads |
| the solution | the agent's tooling on Oxc and Rust, and a lighter context |
| is a | linter, formatter and agent tools that run in seconds, in this repository |
| that | brings `check:all` and the pre-push wait far down, and gives each agent its context in fewer tokens |
| unlike | `check:all` at 170 to 217 s (57 s at E14), ESLint and Prettier on Node.js, agents reading whole files |
| our solution | Oxc's tools where they carry the project's rules and settings; any rule they cannot carry rewritten in Rust on Oxc, here; Rust tools for the agent |
| Business outcomes | faster pushes; fewer tokens per story; the computer usable while agents work |
| Leading indicators | `check:all`'s duration; tokens of each agent's input, before and after; the lint rules carried, counted |
| Non-functional requirements | the library unchanged and still dependency-free (ADR-0025); it keeps targeting the latest ECMAScript, so no syntax lowering is needed; the private records stay apart, a SQLite store the Product Owner may browse with a viewer |
| In scope | the linter, the formatter, the agent's tools and hooks, the checks' speed, the agent's context |
| Out of scope | the library's code and behaviour; the editor's features |
| Closure criteria | ESLint and its plugins removed; `check:all` under a duration set at the refinement; tokens per agent input measured lower; the release that closes the epic made with the flow of EN-046 |

## Goal

Go faster and lose less time on the agent's checks and tools, not on the library:

- **Oxc instead of ESLint and Prettier.** VoidZero's toolchain: Vite and Vitest (already used), Rolldown, and Oxc — oxlint (a linter compatible with ESLint), oxfmt (a formatter compatible with Prettier), the transformer, the resolver (Node.js module resolution) and the minifier. The aim is to drop ESLint and its plugins entirely, not to keep a mixed setup.
- **Rules in Rust when needed.** A lint rule or setting oxlint and oxfmt cannot carry as it is is rewritten by hand in Rust, on Oxc, in this repository: the plugins are open source, so each rule is read, then reimplemented. Rust's own tools (rustfmt, clippy), strict by default, are enough for the Rust code's own style.
- **Agent tools in Rust** for what Oxc cannot replace, living in this repository: for example a tool that gives a new agent its context in fewer tokens and faster.
- **Every new script in Rust** (decided 2026-10-16): from now on a new script, hook or agent tool is written in Rust, not TypeScript, so that no new TypeScript tool has to be rewritten later; the existing TypeScript tools may still be edited until F19 rewrites them.
- **Everything around the library in Rust** (Product Owner, 2026-10-17): the library stays JavaScript; every tool around it — checks, hooks, agent tools, the release and its changelog, the secret scan, the CI's steps — becomes our own Rust or the fastest sound tool (Oxc, TypeScript 7), as fast as it can be for the agent (ADR-0069), depending on as few outside packages as possible, so that the tools connect easily and later project-management tools can build on them.
- **`check:all` optimised hard**, and the pre-push wait with it.
- **More agents without lag**: with the heavy-command lock in the tools (EN-047), ADR-0063's trial of three subagents stands, Vitest limited to half the machine's cores.

## Open questions for the spike

The spike opens the epic. VoidZero's site (voidzero.dev) and Oxc's documentation are read first; a separate research on making AI agents cheaper feeds it too.

- Which of the project's ESLint rules does oxlint carry as they are — the custom rules of `eslint/`, the functional rules, `jsdoc`, `unicorn`, `import-x`, Markdown linting — and which must be rewritten in Rust on Oxc? How much work is each?
- Can the current configurations be kept while the tools are swapped, or must they be translated?
- Does oxfmt format TypeScript and Markdown as Prettier does here (tables, front matter, `proseWrap`, ADR-0065)?
- Which agent tools to rewrite in Rust first (hooks, status line, session report, `activity`, a context tool), and what each saves in time and tokens, measured.
- How fast `check:all` can get, lane by lane, and which target to set.
- How the Rust code is built, tested and checked in this repository without a runtime dependency of the library.

## Features

Ordered by the Product Owner at the refinement of 2026-10-16.

| Feature | Status |
| --- | --- |
| [F18](../features/E17-F18-a-safe-and-cheap-parallel-run.md) — A safe and cheap parallel run, and E15's leftovers | done |
| [F19](../features/E17-F19-oxc-and-rust-tools.md) — Checks on Oxc, TypeScript 7 and Rust | done |
| [F20](../features/E17-F20-a-context-diet.md) — A context diet for every agent | done |
| [F21](../features/E17-F21-agent-tools-in-rust.md) — Agent tools and the library server in Rust | done |
| [F22](../features/E17-F22-the-release-from-the-merge-into-main.md) — One story at a time, the release by a button, its CI green (RET-004 and ADR-0086, 2026-10-18) | done |
| [F24](../features/E17-F24-the-ci-in-parallel-jobs.md) — The CI in parallel jobs, its tests shown as they run (REV-006, 2026-10-18) | done |
| [F25](../features/E17-F25-the-test-lanes-rust-tools.md) — The test lane's Rust tools (#370, 2026-10-18) | done |
| [F26](../features/E17-F26-mutate-stops-on-linux.md) — The mutation runner stops a mutation on Linux (#370, 2026-10-18) | done |
| [F27](../features/E17-F27-retired-scopes-in-history.md) — The history check reads the scopes a commit was written with (#370, 2026-10-18) | done |
| [F29](../features/E17-F29-local-checks-stable-and-fast.md) — The local checks stable and fast (SP-024, 2026-10-18), first: a timing test fails about one push in three | done |
| [F28](../features/E17-F28-one-workflow-per-purpose.md) — One workflow per purpose (2026-10-18, ADR-0088) | done |
| [F30](../features/E17-F30-a-ci-on-develop-that-blocks-nothing.md) — A CI on develop that blocks nothing (SP-024, ADR-0088) | done |
| [F23](../features/E17-F23-the-path-of-a-story.md) — The path of a story through the tools (RET-004 and REV-006, moved from E02 by SP-024) | done |
| [F31](../features/E17-F31-the-guard-and-the-agents-commands.md) — The guard and the agent's commands (SP-024) | done |
| [F32](../features/E17-F32-reports-that-say-what-happened.md) — Reports that say what happened (SP-024) | done |
| [F33](../features/E17-F33-the-checks-ret-004-listed.md) — The checks RET-004 listed (SP-024) | done |
| [F34](../features/E17-F34-the-use-of-each-workflow.md) — The use of each workflow (2026-10-18, its questions answered the same day) | done |

F18 holds the stories decided at RET-003 (ADR-0063, ADR-0064, ADR-0065, ADR-0066), written then as F11 was at RET-001.

The tooling left after 0.3.0's merge into `main` was cut by the Product Owner on 2026-10-18 into small features, each one theme (SP-024, [research note 0026](../../research/notes/0026-the-tooling-features-of-2026-10-18.md)): F29 first, then F28, F30, F23, F31, F32 and F33, the order of the table above. F23, drafted at RET-004 as E02's first feature, moved into E17, its stories EN-118 to EN-121 spread over F23, F31, F32 and F33 with their ids. The review and the method's work are held at the session's end, before the merge into `main` (ADR-0086).

## Refinement (2026-10-16)

Decisions of the Product Owner at the start of the run, recorded as understood.

- **Order**: F18, then F19, then F20. F19's spike (SP-021) runs in parallel with F18. F21 was split from F19 on 2026-10-17 and comes after it; its place against F20 is set at the refinement.
- **`check:all` target**: set after F19's spike, from the lane-by-lane measure, not before.
- **Rust**: already installed on the Product Owner's machine (stable 1.99.1 with cargo, clippy and rustfmt; the `wasm32` target present). Nothing to install.
- **Claude Code settings**: the research of 2026-10-16 recommends four settings — a limit on simultaneous subagents, a one-hour prompt cache on long agents, workers started from a digest instead of all of `CLAUDE.md`, a model per agent type. Each becomes a story, and each change is shown to the Product Owner before it is applied: EN-063 (subagent limit, F18, beside EN-047), EN-064 (prompt cache), EN-065 (digest), EN-066 (model per agent type), all three in F20.
- **E15's leftovers** (US-061, EN-057, EN-062) stay in F18.
- **Model per task**: measured from E17 — Sonnet for logs, backlog, merges and step texts; Opus for mathematics, audits and research. The figures come from the session report.

## Not adopted

- **`git rerere`**: it only replays a resolution already recorded by hand and leaves the file for review unless `rerere.autoUpdate` is set; the gain is low here and the Product Owner is unsure of automatic resolution (ADR-0065).

## Closing

After its last feature is validated (ADR-0023, ADR-0049, ADR-0064). F21's clean-up story, which removes every script, hook, configuration and dev package left unused, comes last among the features' stories, before NAM-002 (decided on 2026-10-17).

| Story | Status |
| --- | --- |
| [NAM-002](../stories/E17-NAM-002-naming-pass-e17.md) — Naming pass over the code of E17 | done |
| [REV-004](../stories/E17-REV-004-review-e17.md) — Review E17 with the Product Owner | done |
| [RET-004](../stories/E17-RET-004-retrospective-e17.md) — Retrospective of E17 | done |
| [REV-006](../stories/E17-REV-006-review-2026-10-18.md) — Review of 2026-10-18, F22 | done |
| [REV-007](../stories/E17-REV-007-review-2026-10-19.md) — Review of 2026-10-19, F23 to F34 | done |
