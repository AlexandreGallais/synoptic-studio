---
name: project-rules
description: The digest of AGENTS.md that every subagent of this repository applies, preloaded by the project's agent types that start without it (EN-065).
user-invocable: false
---

# Project rules, for a subagent

The digest of `AGENTS.md`, kept in step with it. `AGENTS.md` is the source: read it from the worktree when a task touches what this page does not settle; never guess.

## The project

synoptic-studio: a dependency-free TypeScript library, `editor`, that draws SVG symbols for synoptic views; its playground (`playground/`); the Product Owner's docs (`docs/`); the tooling, Rust crates of `tools/`.

## The records first

Everything the agent works from lives in the records, one SQLite file outside the repository shared by every worktree; a row written on a branch is a draft until its branch is merged. From the checkout's root:

| Need | Command |
| --- | --- |
| an item, rendered from its table with its items: a story `EN-151`, a feature `F48`, a rule `RULE-shapes-box`, a decision `ADR-0086`, a source `REF-SQLITE-WAL`, a derivation `DERIV-fillet-setback`, a page `PAGE-research` | `target/release/knowledge show <id> [<heading>]` |
| what the records know on a subject, every table's text | `knowledge search <words> [--kind story, decision, note, rule, source…]` |
| the items of a kind, an epic, a status | `knowledge list --kind … --epic … --status …` |
| any other question | `knowledge sql "<statement>"` |
| any record written — a decision, a note, a source, a derivation, a journal row, an incident, an improvement: its row, then its text one item a row (`decision_items`, `note_findings`, `derivation_steps`…) | `knowledge sql "INSERT INTO <table> …"` |
| a story's status | `target/release/backlog-status <ID> <status>` |
| the ids a change cites, checked | `knowledge check-ids <paths>` |

The records are kept true with the work: a rule, a decision, a source, an incident or a status changes in the same task as the code. The docs are the Product Owner's: nothing in `docs/` is changed, not even at a review, until they say otherwise. Code cites short ids, never a path into the records.

## Always

- **English** in the repository, the records and the commits; the main thread speaks French to the Product Owner.
- **Business rules belong to the Product Owner**: each is a row of the records' `rules` (the rule and one example), written with the code that applies it, which cites its id. A rule missing or ambiguous is a stop, never an invention.
- **Nothing private in the repository**: no quote of the Product Owner, no personal information, nor in a request a tool sends (header, URL, payload); a registry asking for a contact gets the repository's URL.
- **Never invent a reference**: cite only what was read, a `REF-*` of the records or a page read through the `library` server; otherwise a derivation (a row of `derivations`, its steps in `derivation_steps`, each citing a read source) or a research request.
- **A rule earns its place**: kept only if it makes the code more correct or more uniform.
- **Our tools**: each a Rust crate of `tools/`, called by its binary `target/release/<name>`; `replace` or the Edit tool for edits across files, never a Python script; a tool that fails or does not fit is worked around and named in the report, for an incident fixed at the next review.
- **Figma is read, never edited**: a remark on a design is a comment in Figma.

## Code

- Typed integers → model → evaluated geometry → SVG, never reading the DOM back (ADR-0005); a pure core, effects only in `render`, `interaction` and `playground`; a function never modifies its arguments (ADR-0014).
- In the library (`src/`): one export per file named after it in kebab-case, in its business domain's folder; no default nor optional parameter; no `enum`; a class only where it is a real object, never at a cost in speed; `any`, `!` and `as` only with a disable directive and its reason. The playground follows its framework's usage.
- One function holds one business context; every calculation a field names is a call to a function of that name (ADR-0049); reuse a function, never copy it.
- Every function has one `@kind` (`math`, `geometry`, `domain`, `format`, `procedure`) and austere TSDoc; a `math`, `geometry` or `domain` function cites its source or rule by `@see <id>`, the others none.
- Every input an integer, derived geometry never stored; `EPSILON = 1e-9`, `SVG_DECIMALS = 5`; everything a `<path>` but text (ADR-0003).
- Tests before or with the code, in one commit: hand-computed examples with their computation in a comment, degenerate cases, a property for each `math` and `geometry` function; each new test seen failing once; 95 % of `src/` covered; a test proving a criterion is titled `[F20.AC1] …` (Rust: `f20_ac1_…`).

## Never

Hooks refuse most of these; none is worked around.

- a runtime dependency; a dev package or crate younger than 7 days or without its row in the records' `dependencies` (exported by `knowledge export-data`), its release notes read;
- ESLint, or a new script, hook or tool in TypeScript: a lint rule oxlint lacks goes into our Rust linter;
- a lint rule disabled to make code pass;
- a push to `main` or `develop`, a pull request merged by hand, `--no-verify` (nor `commit -n`, nor a change of `core.hooksPath`), a force push without lease.

## Git

- Every change on a branch of its own, merged by a merge commit, so that the history shows each story and feature as a bubble.
- Conventional Commits, a scope of `SCOPES` in `tools/git-hooks/src/message.rs`, a subject starting with a verb, no body line starting with `word:` (read as a footer), footers within 100 characters, `Refs: <ID>.Tn`.
- Commits by `target/release/git-sequences commit -F <file or -> [--push] <paths>`; a story brought up to its feature branch by `git-sequences sync`; `develop` never merged into a work branch. A commit refused by a hook leaves its files staged: `git status` before retrying.

## Stop and report

Stop when going on would mean deciding what belongs to the Product Owner, or guessing: report what blocks, what was read, the question and the options, without choosing one.

| Signal | Report |
| --- | --- |
| a business rule missing, or code, rules and decisions contradicting each other | the question and the options |
| a formula without a readable source; expected values not computable by hand | a derivation to check, or a research request |
| the same check failing three times | the analysis |
| a runtime dependency or a forbidden primitive needed | a decision to propose |
| a story of more than 3 tasks | its split, with an example |
