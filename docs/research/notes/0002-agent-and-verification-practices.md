# Note 0002 — Working with a coding agent, and verifying its work

**Date**: 2026-10-08 — research done by Claude Code (WebSearch / WebFetch, trial installs in a scratch copy), at the Product Owner's request: what would help the agent's work later.
**Resulting decisions**: ADR-0021 (agent working environment), ADR-0022 (property-based testing, link checking, mutation testing deferred).

## 1. What the agent's own documentation recommends

- **Verification is the main lever**: an agent stops when the work "looks done"; a check it can run (tests, build, linter, script) closes the loop. Show evidence (command and output), not assertions (`REF-CC-BEST-PRACTICES`).
- **Separate exploration and planning from implementation**; for larger work, interview the Product Owner first and write a self-contained spec that ends with a verification step (`REF-CC-BEST-PRACTICES`). Our research spike and feature plan play that role.
- **Instruction file short**: loaded in every session, so only what applies broadly; target under 200 lines; a multi-step procedure or something relevant to one part of the code belongs in a **skill** or a **path-scoped rule** (`.claude/rules/` with `paths:`), loaded only when needed. Contradictions make the agent pick one arbitrarily: review periodically (`REF-CC-MEMORY`, `REF-CC-BEST-PRACTICES`).
- **Instructions are advisory, hooks are deterministic**: what must happen every time, or must never happen, goes into a hook (`PreToolUse` can deny a call, exit code 2 feeds the reason back) (`REF-CC-HOOKS`, `REF-CC-MEMORY`).
- **Skills** (`.claude/skills/<name>/SKILL.md`): a description used to decide relevance, the body loaded only on use, invocable as `/name`; `disable-model-invocation: true` for workflows with side effects (`REF-CC-SKILLS`).
- **Subagents** (`.claude/agents/*.md`): fresh context, restricted tools. A reviewer that sees only the result and the criteria — not the reasoning that produced it — judges it on its own terms; "the agent doing the work isn't the one grading it". Tell the reviewer to report only gaps that affect correctness or the requirements, or it will invent findings (`REF-CC-BEST-PRACTICES`, `REF-CC-SUBAGENTS`).

## 2. Long-running work and context

- Context is a finite attention budget: the smallest set of high-signal tokens. Persistent notes outside the context window, reloaded later, keep long work coherent; references (paths, links) are loaded just in time (`REF-ANTHROPIC-CONTEXT`).
- For this project: the repository already is the note-taking system (backlog statuses, domain, ADRs, derivations); what was missing is a **cheap reload** after a compaction or a new session → a `SessionStart` hook that prints the state derived from the files (branch, uncommitted files, stories `ready` or `in-progress`).

## 3. How AI-assisted projects structure specifications

- Spec-driven development: a structured, behavior-oriented spec written before the agent codes; levels _spec-first_, _spec-anchored_ (kept up to date with the feature), _spec-as-source_. Tools (Kiro: requirements → design → tasks with Given / When / Then; spec-kit: constitution → specify → plan → tasks) (`REF-BOCKELER-SDD`).
- Critiques from the same author: one workflow for every size is overkill for small changes; verbose generated Markdown is tedious to review; agents still ignore or over-apply spec instructions — a spec gives a **false sense of control** without checks.
- Position of this project: already spec-anchored — feature and story files (Given / When / Then), domain files kept in the same change as the code, ADRs as the "constitution". Nothing to adopt; the lesson kept is that **checks, not documents, give control** → hooks, tests, audit.
- `AGENTS.md`: an open format read by many agents, stewarded by the Linux Foundation (`REF-AGENTS-MD`). Claude Code can read it in place of `CLAUDE.md` (`REF-CC-MEMORY`). Not adopted on 2026-10-08: one agent was used. Adopted at RET-001 (2026-10-13): `AGENTS.md` for any agent, imported by `CLAUDE.md` (ADR-0040).

## 4. Verifying mathematical code

- **Property-based testing**: a property is a relation between input and output that must hold for all inputs; random inputs from generators ("arbitraries"), failing inputs shrunk to a minimal counterexample, seeded and reproducible (`REF-FASTCHECK-PBT`). Origin: QuickCheck — properties tested on random input; "especially suitable for functional programs because properties can be stated at a fine grain"; when a function is built from separately tested components, random testing gives good coverage (`REF-QUICKCHECK`). This matches the library: small pure functions composed like formulas.
- Candidate properties: symmetry of `dot`, antisymmetry of `perpDot`, turning angle in ]−π, π], total turning ±2π on a simple closed contour, setback ≥ 0 and even in τ, effective radius ≤ requested radius, `formatSvgNumber` within 10⁻⁵ of its input.
- Tool: `fast-check` (MIT, one dependency `pure-rand`) and `@fast-check/vitest` (MIT, `test.prop`, seed replay) — trial install: `npm audit` clean.
- **Mutation testing** measures whether tests detect injected faults; at Google it is part of code review, limited to covered, non-"arid" lines and to a few surfaced mutants (`REF-GOOGLE-MUTATION`).
- Tool: StrykerJS with its Vitest runner. Trial on this repository: score 4 % — every covered mutant survived, including `radius / tan(0)`. Cause: open issue `REF-STRYKER-6210` — with Vitest 5 the runner's test-name filter matches nothing, so no test runs against a mutant. Also pulled a vulnerable `qs` (fixable by lockfile update). **Deferred** until the issue is fixed; the audit's manual mutation spot-checks stay.

## 5. Checking links

- lychee: link checker for Markdown and HTML, single binary, GitHub Action `lycheeverse/lychee-action` (`fail`, `jobSummary`, `.lycheeignore` with one regex per line) (`REF-LYCHEE-ACTION`).
- Trial (lychee 0.24.2, checksum verified): `docs/references.md` 54 URLs all OK; all Markdown 253 links, 2 false positives (the local dev server URL, a template link valid once copied into `stories/`). Local file links are checked too, including files VitePress does not publish (`CLAUDE.md`, `.claude/`).
- Scheduled weekly in CI: source rot is caught between audits.

## 6. Retained for this project

| Practice | Where |
| --- | --- |
| Procedures out of `CLAUDE.md`, as skills: story, spike, audit, derivation | `.claude/skills/` (ADR-0021) |
| Pitfalls scoped to the files they concern | `.claude/rules/` (ADR-0021) |
| Independent auditor in a fresh context, read-only, reports only real gaps | `.claude/agents/auditor.md` (ADR-0021) |
| Guards that never depend on memory: no merge of a pull request, no `--no-verify`, no force push without lease | `.claude/settings.json` hook (ADR-0021) |
| State reloaded at session start and after compaction | `SessionStart` hook (ADR-0021) |
| Interview the Product Owner when refining a feature | backlog method |
| Property-based tests for `math` and `geometry` | `fast-check` (ADR-0022) |
| Weekly link check of all Markdown | `links.yml` workflow (ADR-0022) |
| Mutation testing tool | deferred, `REF-STRYKER-6210` (ADR-0022) |
