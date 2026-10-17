---
name: project-rules
description: The rules of AGENTS.md and CLAUDE.md that every subagent of this repository applies, preloaded by the project's agent types that start without CLAUDE.md (EN-065).
user-invocable: false
---

# Project rules, for a subagent

The digest of `AGENTS.md` and `CLAUDE.md` for an agent type started with `omitClaudeMd` (EN-065): the rules any task applies. `AGENTS.md` stays the source; read a section of it only when a task touches what this page does not cover.

## The repository

synoptic-studio: a dependency-free TypeScript library `editor` that draws SVG symbols for synoptic views, its playground (`playground/`), its docs (`docs/`), and its tooling (`tools/`, Rust crates; `scripts/`, TypeScript until F21). The backlog (`docs/backlog/`) holds epics, features and stories; ADRs (`docs/adr/`) say why.

## Always

- **English** in every file and commit; only the glossary keeps a French column. The main thread speaks French to the Product Owner: a report to it may stay in English.
- **Never quote the Product Owner**: a decision is written in neutral English, with its reason and date (`docs/conventions/writing.md`). **No personal information** about them: the repository is public.
- **Every business rule lives in `docs/domain/`**, changed in the same commit as the code; a rule missing or ambiguous is a stop, never an invention.
- A structural decision is a new ADR; an accepted ADR is superseded, never rewritten.
- **Never invent a reference**: cite only what was read (`docs/references.md`, a `REF-*` row `[verified]`); otherwise a derivation (`docs/derivations/`) or a research request. Sources through the `library` server first (`library_search`, `library_get`, `library_read`); the web only for a page it cannot read.
- Use the repository's tools rather than ad-hoc scripts (`docs/tooling/agent-tools.md`): `npm run replace` for edits across files, `backlog:status`, `context:size`; never a Python edit script.

## Code

- Pipeline (ADR-0005): typed integers → model → evaluated geometry → SVG; never read the DOM back. Functional core, imperative shell (ADR-0014); a function never modifies its arguments.
- One export per file, named after it in kebab-case (ADR-0019); folders by business domain (ADR-0043); no default nor optional parameter in `src/` (ADR-0016); no class, no `enum`; `any`, `!`, `as` only with a disable directive and its reason, registered in `docs/conventions/exceptions.md` (ADR-0066).
- Every function: one `@kind` (`math`, `geometry`, `domain`, `format`, `procedure`), one `@see` to a verified source, austere TSDoc; every calculation a field names is a call to a function of that name (ADR-0049).
- Numbers (ADR-0003): every input an integer; derived geometry float and never stored; `EPSILON = 1e-9`, `SVG_DECIMALS = 5`. SVG primitives: `svg`, `g`, `path`, `text`, `tspan`, `defs`, `style` in `defs` only.
- **Never**: a runtime dependency; a dev package or crate without its row in `docs/tooling/dependencies.md`, its release notes read and 7 days old; a lint rule disabled to make code pass.
- Every new script, hook or agent tool in Rust, a crate of `tools/` (ADR-0073).
- Tests first: hand-computed examples with their computation in a comment, degenerate cases, then properties (ADR-0022); a test proving a criterion is titled `[F20.AC1] …` (Rust: `f20_ac1_…`); a new test that reads files is made to fail once on purpose.

## Git

- Never push to `main` or `develop`, never merge a pull request, never `--no-verify` (nor `commit -n`, `HUSKY=0`), never force push without lease; the `guard-bash` hook refuses them: never look for a way around.
- Commits: Conventional Commits with a scope of `commitlint.config.ts`, a subject starting with a verb (an id first is refused as sentence case), footers within 100 characters; the message from a file, committed by `npm run git:commit -- -F <file> [--push] <paths>`, and a story brought up to its feature branch by `npm run git:sync` (EN-100). A commit refused by a hook leaves its files staged: `git status` before retrying.

## Stop and report

| Signal | Report |
| --- | --- |
| a business rule missing from `docs/domain/`, or code, domain and ADRs contradicting each other | the question, what was read, the options without choosing one |
| a formula without a readable source; expected values not computable by hand | a derivation to validate, or a research request |
| the same check failing after 3 attempts | the analysis |
| a runtime dependency or a forbidden primitive needed | an ADR to propose |
