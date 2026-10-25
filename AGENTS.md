# AGENTS.md — synoptic-studio

The entry point for any coding agent. Everything you need to work lives in **the records**, not in the docs: read this page, then ask the records. `CLAUDE.md` adds only what is specific to Claude Code; the shared method (skills, agent briefs, area rules) is in `.agents/`.

## The project

A TypeScript library, `editor`, that draws **SVG symbols** for **synoptic views** — strict, numeric, schematic — and the editors to be built on it (Symbol Editor, Configurator, View Editor). The playground (`playground/`) shows each stage. Built by coding agents for one Product Owner, who sets the direction and the priorities and is also the agent's manager: their "no" stands, and the agent may always ask for its reason.

## The records first

The records are one SQLite file outside the repository, `$AGENT_RECORDS_ROOT/synoptic-studio.sqlite`, shared by every branch and worktree. They hold everything the agent works from: the backlog (epics, features, stories, tasks, criteria), the business rules, the decisions (ADRs), the conventions, the research and its sources, the logs, the incidents, the memory and the metrics. A row written on a branch is a draft until its branch is merged.

Talk to them through `target/release/knowledge`, called from the checkout's root:

| Need | Command |
| --- | --- |
| an item by its id: a story `EN-151`, a rule `RULE-shapes-box`, a decision `ADR-0086`, a source `REF-SQLITE-WAL`, a page `PAGE-research` | `knowledge show <id> [<heading>]` |
| what the records know on a subject | `knowledge search <words> [--kind …]` |
| the items of a kind, an epic, a feature, a status | `knowledge list --kind … --epic … --status …` |
| any question, any shape of your own (tables and indexes may be added, each registered in `tables_registry`) | `knowledge sql "<statement>" [--purpose …]` |
| a record written: a decision, a note | `knowledge add <kind> <file or ->` |
| a row written: a journal row, an incident, an improvement | `knowledge sql "INSERT INTO <table> …"` |
| a story's status | `target/release/backlog-status <ID> <status>` |
| the ids a change cites, checked against the records | `knowledge check-ids <paths>` |

Start any task with the records: the story (`knowledge show <ID>`), its feature and epic, the rules and decisions it touches (`knowledge search`), the conventions of its area. The records are kept true as you go: a rule, a decision, a source, an incident or a status changes in the records with the work that changes it.

**The docs are for the Product Owner only** (`docs/`: what the software is and how the project is made). They are not changed until the Product Owner asks, not even at a review. Code cites short ids, never a path into the records.

## Golden rules

- **English everywhere** in the repository and the records; messages to the Product Owner in French.
- **Business rules belong to the Product Owner**: each is a row of the records' `rules` (the rule and one example), written with the code that applies it, cited by its id. A rule missing or ambiguous is asked, never invented.
- **Nothing private in the repository**: no quote of the Product Owner (the private records may keep their words), no personal information — nor in any request an agent sends or a tool codes (header, URL, payload).
- **Never invent a reference**: cite only what was read, through the `library` sources; otherwise a derivation or a research request.
- **A rule earns its place**: kept only if it makes the code more correct or more uniform.
- **Tooling as fast as it can be**: every tool of ours is a Rust crate of `tools/`, called by its binary `target/release/<name>`; a lint rule oxlint lacks is written in our Rust linter, never with ESLint.
- **Use our tools, record their failures**: a task goes through the project's tools; a tool that fails or does not fit is worked around and recorded as an incident in the records, fixed at the next review.
- **Figma is read, never edited**: a remark on a design is a comment in Figma.

## Code

The details and their reasons are rows of the records — `conventions` by area, `lint_rules`, `lint_exceptions`, `dependencies`, `tools`, `method_rules` (`knowledge sql "SELECT * FROM tables_registry"`); the essentials:

- typed integers → model → evaluated geometry → SVG, never reading the DOM back; a pure core (`math` to `io`), effects only in `render`, `interaction` and `playground`; a function never modifies its arguments;
- in the library (`src/`): one export per file named after it, in the folder of its business domain, no default nor optional parameter; the playground follows its framework's usage;
- one function holds one business context; every calculation a field names is a call to a function of that name;
- every function has one `@kind` (`math`, `geometry`, `domain`, `format`, `procedure`) and austere TSDoc; a `math`, `geometry` or `domain` function cites its source or rule by `@see <id>`;
- every input an integer, derived geometry never stored; `EPSILON = 1e-9`, `SVG_DECIMALS = 5`; everything a `<path>` but text;
- plain data, no `enum` in the library, a class only where it is a real object and costs no speed;
- tests before or with the code: hand-computed examples, degenerate cases, properties; 95 % of `src/` covered; a test proving a criterion is titled `[F01.AC3] …`.

## Never

Hooks refuse most of these; none is worked around.

- a runtime dependency; a dev package or crate younger than 7 days or without its row in the records;
- ESLint, or a new script, hook or tool in TypeScript;
- a lint rule disabled to make code pass;
- a push to `main` or `develop`; `--no-verify`; a force push without lease; a pull request merged other than by its auto-merge label — the one into `main` only once the Product Owner validated the review in the chat.

## Work

| Level | Flow |
| --- | --- |
| Epic | one subject, taken in the Product Owner's order; its research reads the sources and the code, then one questionnaire asks every question at once, meant to be right the first time; the whole cut — features, stories, tasks — is made before any code; unrelated features go to another epic, a scope that grows is cut again at once; an epic tagged **method and tools** has no audit per feature and one thorough audit at its end |
| Feature | branched from `develop`; its stories one at a time, each by a fresh worker; at its end a light audit of that feature (none in a method-and-tools epic), then the full check — mechanical, as the CI runs it — and its auto-merge into `develop` |
| Story | from the records (criteria, tasks); code and tests written together; a commit per task (`Refs: <ID>.Tn`); a push that checks only what it touches; a pull request into its feature, auto-merged; its status and log rows in the records, no story file; every friction or need written at once as an `improvements` row |
| Branches | `main` the reviewed work; `develop` integration; `feature/f<nn>-<topic>` from `develop`; `<type>/<id>-<topic>` per story; every change on a branch of its own merged by a merge commit, even a one-commit change, so the history shows each story and feature as a bubble; always a work branch into its base, never `develop` into a work branch — a conflict is solved by rebuilding the branch from `develop` |
| Review | only when the Product Owner asks, the feature under way finished first: the naming pass (names of functions, arguments, properties, constants), the tool incidents, the dependency updates, the issues triaged, the method and tools — every friction and need, unfiltered; once the Product Owner validates it in the chat, the `po-validated` label merges `develop` into `main`; a release only on request |
| Agents | one worker per story, in the background, never one agent chaining stories; specialists (`auditor`, `math-expert`, `functional-analyst`) only where their independence pays |

## Done

1. `npm run fix`, then `npm run check:all` green, seen and not assumed.
2. The records up to date: the rule, the decision, the source, the status, the incidents.
3. A summary in French: result, changes, evidence, decisions needed, next step.

## Stop and ask

Stop when going on would mean deciding what belongs to the Product Owner, or guessing: report what blocks, what was read, the question and the options, without choosing one — a rule missing, code and decisions contradicting each other, a formula without a readable source, the same check failing three times, a runtime dependency needed.

## Git

Conventional Commits, a scope of `SCOPES` in `tools/git-hooks/src/message.rs`, a subject starting with a verb; commits made by `target/release/git-sequences commit -F <file> <paths>`, a story finished by `target/release/heavy-lock --lock merge target/release/story-finish <base> "<title>" <body file>`; `main` is never merged back into `develop`.

`package.json` keeps only what a person or npm runs (`dev`, `docs:dev`, `test`, `fix`, `check:all`, `prepare`); the checks' commands live in `tools/repo-tools/src/check_all.rs` (`target/release/check-all --list`, one lane by `--lane <name>`), and the agent's heavy tools run by their binary under the heavy lock: `target/release/heavy-lock target/release/mutate <files>`, the golden check `target/release/heavy-lock npx vitest run tests/golden-results.test.ts --mode golden-check`.
