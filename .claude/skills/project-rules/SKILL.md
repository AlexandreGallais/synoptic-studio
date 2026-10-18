---
name: project-rules
description: The rules of AGENTS.md and CLAUDE.md that every subagent of this repository applies, preloaded by the project's agent types that start without CLAUDE.md (EN-065).
user-invocable: false
---

# Project rules, for a subagent

The digest of `AGENTS.md` for an agent type started without it (`omitClaudeMd`, EN-065), kept in step with it. `AGENTS.md` is the source: read its section, from the worktree, when a task touches what this page does not settle; never guess.

## The repository

synoptic-studio: a dependency-free TypeScript library `editor` that draws SVG symbols for synoptic views; its playground (`playground/`), docs (`docs/`, the ADRs in `docs/adr/`, the backlog in `docs/backlog/`) and tooling (`tools/` in Rust, `scripts/` in TypeScript until F21).

## Always

- **English** in every file and commit; only the glossary keeps a French column. A report to the main thread may stay in English; the main thread speaks French to the Product Owner.
- **Never quote the Product Owner**: a decision is written in neutral English, with its reason and date (`docs/conventions/writing.md`). **No personal information** about them: the repository is public.
- **Every business rule lives in `docs/domain/`**, changed in the same commit as the code; a rule missing or ambiguous is a stop, never an invention.
- A structural decision is a new ADR; an accepted ADR is superseded, never rewritten.
- **Never invent a reference**: cite only what was read (a `REF-*` row of `docs/references.md`, `[verified]`, read alone with `target/release/page REF-…`); otherwise a derivation (`docs/derivations/`) or a research request. Sources come through the `library` server (`library_search`, `library_get`, `library_read`), which keeps each page read; the web only for a page it cannot read; how many sources a claim needs: `docs/research/README.md` §2.
- **A rule earns its place**: it stays only if it makes the code or the docs more correct or more uniform (`docs/conventions/writing.md`, Writing a rule).
- Use the repository's tools rather than ad-hoc scripts (`docs/tooling/agent-tools.md`), each called by its binary from the checkout's root, `target/release/<binary>`, as its table maps the scripts (ADR-0078): `target/release/replace` or the Edit tool for edits across files, never a Python edit script; `target/release/page [<file>] <id or heading>` rather than `Read` for one row or one section of a long page; `backlog-status`, `context-size`.

## Code

- Pipeline (ADR-0005): typed integers → model → evaluated geometry → SVG; never read the DOM back. Functional core, imperative shell (ADR-0014): a function never modifies its arguments.
- One export per file, named after it in kebab-case (ADR-0019), in its business domain's folder (ADR-0043); no default nor optional parameter in `src/` (ADR-0016); no class, no `enum`; `any`, `!` and `as` only with a disable directive and its reason, registered in `docs/conventions/exceptions.md` (ADR-0066). Reuse an existing function, never copy it.
- Every function: one `@kind` (`math`, `geometry`, `domain`, `format`, `procedure`), one `@see` to a verified source, austere TSDoc; every calculation a field names is a call to a function of that name (ADR-0049).
- Numbers (ADR-0003): every input an integer; derived geometry float and never stored; `EPSILON = 1e-9`, `SVG_DECIMALS = 5`. SVG primitives: `svg`, `g`, `path`, `text`, `tspan`, `defs`, `style` in `defs` only; no native stroke.
- **Never**: a runtime dependency; a dev package or crate without its row in `docs/tooling/dependencies.md`, its release notes read and 7 days old; a lint rule disabled to make code pass; a new script, hook or agent tool other than a Rust crate of `tools/` (ADR-0073).
- Tests first (ADR-0022): hand-computed examples with their computation in a comment, degenerate cases, then a property test for each `math` and `geometry` function; 100 % of `src/` covered; a test proving a criterion is titled `[F20.AC1] …` (Rust: `f20_ac1_…`); a new test that reads files is made to fail once on purpose.

## Git

- Never push to `main` or `develop`, never merge a pull request, never `--no-verify` (nor `commit -n`, `HUSKY=0`), never force push without lease, nor at all on a feature branch; the `guard-bash` hook refuses most of them: never look for a way around it.
- Commits: Conventional Commits with a scope of `SCOPES` in `tools/git-hooks/src/message.rs` (`docs/process/` takes `docs(docs)`, a derivation `docs(geometry)`), a subject starting with a verb (an id first is refused as sentence case), no body line starting with `word:` (read as a footer), footers within 100 characters. The message comes from a file, committed by `target/release/git-sequences commit -F <file> [--push] <paths>`; a story is brought up to its feature branch by `target/release/git-sequences sync` (EN-100). A commit refused by a hook leaves its files staged: `git status` before retrying.

## Stop and report

Stop when going on would mean deciding what belongs to the Product Owner, or guessing; report the question, what was read, and the options without choosing one.

| Signal | Report |
| --- | --- |
| a business rule missing from `docs/domain/`, or code, domain and ADRs contradicting each other | the question and the options |
| a formula without a readable source; expected values not computable by hand | a derivation to validate, or a research request |
| a function over its kind's limits that no split into named parts fits | a new limit to propose, the function as evidence |
| the same check failing after 3 attempts | the analysis |
| a runtime dependency or a forbidden primitive needed | an ADR to propose |
