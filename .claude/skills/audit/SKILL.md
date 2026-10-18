---
name: audit
description: Run the feature audit (AUD story) — mathematics, sources, provenance, duplicates, consistency — with an independent auditor subagent. Use when a feature's implementation stories are merged or for a story of type AUD.
argument-hint: "[audit id, e.g. AUD-001]"
---

# Feature audit

Audit: $ARGUMENTS. Checklist: `docs/conventions/audit.md` (A–G); findings go in the story's **Findings** table, with evidence.

Checks by level (ADR-0064): stories have no independent review, so this audit is the feature's thorough one. At each review, the naming pass goes over the code changed since the last merge into `main` (ADR-0086): duplicates, shareable code, named calculations.

1. **Scope**: the feature's functions (`@see` targets, tests, derivations) in depth; the whole project in breadth.
2. **Independent review**, one agent at a time, in the background (ADR-0086, decision 1): the `auditor` alone first (feature id, files, derivations, checklist sections A to E); then, only where its independence pays, the `verifier` when the tests need a second look, the `math-expert` only for a feature holding mathematics (its derivations and expected values), the `doc-editor` only when the feature changed many pages, never beside another agent. They return findings with evidence; they do not edit.
3. **Mutation spot-checks (A4)** yourself, one per function at least: `npm run mutate -- <files>` tries the first mutation of each kind per file in a copy of the repository (`--all` for every one) and lists the survivors; for a mutation it cannot write, change one operator, sign or constant by hand, run its test file, record the mutation and the failing test, restore with `git checkout -- <file>` and check `git status` is clean.
4. **Properties**: add `test.prop` properties to functions of the feature that lack them (ADR-0022).
5. **Verify each finding** of the auditor before recording it: reproduce it; a gap that does not affect correctness or the requirements is recorded as "no action" with the reason.
6. **Fix** small findings in the audit's commits; larger ones become stories (Product Owner decides their priority).
7. **Breadth checks**: `npm run check:all`; the weekly link check is green (or run lychee); `AGENTS.md`, `CLAUDE.md`, `.claude/rules/`, `docs/conventions/`, `docs/tooling/` still describe reality.
8. **G**: list the sources and questions the next feature needs, for its spike.
9. **H, light evolvability check**: the six questions of `docs/process/README.md` in one line each; anything worth acting on goes to the improvement log for the next review.
10. Branch, commits per task, pull request as for a story (`/story` steps 3 to 9).
