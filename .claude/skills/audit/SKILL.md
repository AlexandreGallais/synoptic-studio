---
name: audit
description: Run the feature audit (AUD story) — mathematics, sources, provenance, duplicates, consistency — with an independent auditor subagent. Use when a feature's implementation stories are merged, at the end of a method-and-tools epic, or for a story of type AUD.
argument-hint: "[audit id, e.g. AUD-001]"
---

# Feature audit

Audit: $ARGUMENTS. Checklist: `docs/conventions/audit.md` (A–G); findings go in the audit's row of the records (`text`), with evidence.

Stories have no independent review, so this audit is the feature's thorough one; an epic tagged method and tools has one audit, at its end, with a full check. At each review, the naming pass goes over the code changed since the last merge into `main`.

1. **Scope**: the feature's functions (`@see` targets, tests, derivations, rules) in depth; the whole project in breadth.
2. **Independent review**, in the background: the `auditor` alone first (feature id, files, derivations, checklist sections A to E); then, only for a feature holding mathematics, the `math-expert` on its derivations and expected values. They return findings with evidence; they do not edit.
3. **Mutation spot-checks (A4)** yourself, one per function at least: `npm run mutate -- <files>` tries the first mutation of each kind per file in a copy of the repository (`--all` for every one) and lists the survivors; for a mutation it cannot write, change one operator, sign or constant by hand, run its test file, record the mutation and the failing test, restore the file and check `git status` is clean.
4. **Properties**: add `test.prop` properties to the feature's functions that lack them.
5. **Verify each finding** of the auditor before recording it: reproduce it; a gap that does not affect correctness or the requirements is recorded as "no action" with the reason.
6. **Fix** small findings in the audit's commits; larger ones become stories (the Product Owner decides their priority).
7. **Breadth checks**: `npm run check:all`, the full one; the weekly link check is green; `AGENTS.md`, `CLAUDE.md`, `.claude/` and `docs/conventions/` still describe reality.
8. **G**: the sources and questions the next feature needs, for its refinement.
9. **H, light evolvability check**: size, tools, fitness functions, patterns, agent configuration, process, one line each; anything worth acting on becomes an improvement row in the records for the next review.
10. Branch, commits per task, pull request as for a story (`/story` steps 3 to 7).
