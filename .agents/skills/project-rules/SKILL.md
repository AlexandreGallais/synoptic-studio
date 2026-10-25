---
name: project-rules
description: The rules of AGENTS.md and CLAUDE.md that every subagent of this repository applies, preloaded by the project's agent types that start without CLAUDE.md (EN-065).
user-invocable: false
---

# Project rules, for a subagent

The digest of `AGENTS.md` for an agent type started without it, kept in step with it. `AGENTS.md` is the source: read its section, from the worktree, when a task touches what this page does not settle; never guess.

## The repository

synoptic-studio: a dependency-free TypeScript library `editor` that draws SVG symbols for synoptic views; its playground (`playground/`), the person's docs (`docs/`: the editor in plain words, how it works, where it stands, the order of the epics, the commands) and tooling (`tools/` in Rust; `scripts/default-font-metrics.ts` alone in TypeScript, since it runs the library itself).

## The records first

The backlog, the ADRs, the research, the derivations, the bibliography, the history, the logs and the agent's memory live in the records, a database outside the repository, read and written by `target/release/knowledge`: `show <id> [<heading>]` for an item (`EN-151`, `ADR-0081`, `REF-SQLITE-WAL`, `DERIV-fillet-setback`), `search <words>`, `list`, `sql "<statement>"`, `add <kind> <file or ->` (`docs/tooling/knowledge.md`). Read them before the docs; cite short ids, never a Markdown path into the records.

## Always

- **English** in every file and commit. A report to the main thread may stay in English; the main thread speaks French to the Product Owner.
- **Never quote the Product Owner in the repository**: a decision is written in neutral English, with its reason and date; the records may keep their words. **No personal information** about them: the repository is public. **Nor in a request** an agent sends or a tool codes: no name, e-mail address nor account of theirs in a header (`User-Agent`), a URL or a payload; a registry asking for a contact gets the repository's URL.
- **Every business rule is written in the records' `rules` table with its code**, as the rule and one example, the code citing its id; a rule missing or ambiguous is a stop, never an invention.
- **ADRs live in the records**: a structural decision is an ADR, changed in place when it changes; one that no longer holds is removed into the record of ADR-0101.
- **Never invent a reference**: cite only what was read, a `REF-*` of the records or a page read through the `library` server (`library_search`, `library_get`, `library_read`), the web only for a page it cannot read; otherwise a derivation or a research request.
- **A rule earns its place**: it stays only if it makes the code or the docs more correct or more uniform.
- Use the repository's tools rather than ad-hoc scripts, each called by its binary from the checkout's root, `target/release/<binary>`: `replace` or the Edit tool for edits across files, never a Python edit script; `page [<file>] <id or heading>` rather than `Read` for one section of a long page; `knowledge`, `backlog-status`, `context-size`.
- **Figma is read, never edited**: a remark is a comment.

## Code

- Pipeline (ADR-0005): typed integers → model → evaluated geometry → SVG; never read the DOM back. Functional core, imperative shell (ADR-0014): a function never modifies its arguments.
- One export per file in the library (`src/`), named after it in kebab-case, in its business domain's folder; no default nor optional parameter in `src/`; no `enum`; a class only where it makes the code clearer; `any`, `!` and `as` only with a disable directive and its reason. Reuse an existing function, never copy it.
- Every function: one `@kind` (`math`, `geometry`, `domain`, `format`, `procedure`), austere TSDoc; a `math`, `geometry` or `domain` function cites its source or rule by `@see`, the others none; every calculation a field names is a call to a function of that name (ADR-0049).
- Numbers (ADR-0003): every input an integer; derived geometry float and never stored; `EPSILON = 1e-9`, `SVG_DECIMALS = 5`. SVG primitives: `svg`, `g`, `path`, `text`, `tspan`, `defs`, `style` in `defs` only; no native stroke, but for the helps of the playground's layer above the drawing.
- **Never**: a runtime dependency; a dev package or crate without its row in `docs/tooling/dependencies.md`, its release notes read and 7 days old; a lint rule disabled to make code pass; ESLint; a new script, hook or agent tool other than a Rust crate of `tools/` (ADR-0073).
- Tests first: the tests written before or with the code, each new test seen failing once; hand-computed examples with their computation in a comment, degenerate cases, then a property test for each `math` and `geometry` function; 95 % of `src/` covered; a test proving a criterion is titled `[F20.AC1] …` (Rust: `f20_ac1_…`); a new test that reads files is made to fail once on purpose.

## Git

- Never push to `main` or `develop`, never merge a pull request, never `--no-verify` (nor `commit -n`, nor a change of `core.hooksPath`), never force push without lease, nor at all on a feature branch, `develop` or `main`; never fix a tool during a feature's work: work around it and report it for an incident card, fixed at the next review; the `guard-bash` hook refuses most of them: never look for a way around it.
- Commits: Conventional Commits with a scope of `SCOPES` in `tools/git-hooks/src/message.rs`, a subject starting with a verb (an id first is refused as sentence case), no body line starting with `word:` (read as a footer), footers within 100 characters. The message comes from a file, committed by `target/release/git-sequences commit -F <file> [--push] <paths>`; a story is brought up to its feature branch by `target/release/git-sequences sync`. A commit refused by a hook leaves its files staged: `git status` before retrying.

## Stop and report

Stop when going on would mean deciding what belongs to the Product Owner, or guessing; report the question, what was read, and the options without choosing one.

| Signal | Report |
| --- | --- |
| a business rule missing from the records' rules, or code, domain and ADRs contradicting each other | the question and the options |
| a formula without a readable source; expected values not computable by hand | a derivation to validate, or a research request |
| a function over its kind's limits that no split into named parts fits | a new limit to propose, the function as evidence |
| the same check failing after 3 attempts | the analysis |
| a runtime dependency or a forbidden primitive needed | an ADR to propose |
| a story of more than 3 tasks | its split, with an example, for the Product Owner |
