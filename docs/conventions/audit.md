# Feature audit

The `AUD` story that closes every feature checks the feature in depth and the whole project in breadth (ADR-0020). Each check leaves its evidence in the story's **Findings** table. A problem found is fixed in the audit, or becomes a story.

The procedure is the `/audit` skill. The independent `auditor` subagent reviews sections A to E first, with a fresh, read-only context (ADR-0021). Each of its findings is reproduced before it is recorded.

## A. Mathematics (feature, in depth)

1. Re-derive every formula of the feature from its `@see` (derivation or reference), without looking at the code.
2. Recompute every expected test value by hand; compare with the test.
3. Look for missing cases: degenerate input (0, empty, aligned, coincident), sign, bounds, very large values.
4. **Mutation spot-check**: break each function once (flip a sign, swap an operator, change a constant) and check that at least one test fails; restore. Record each mutation and the failing test.
5. **Properties**: every function with a stateable invariant has `test.prop` properties (ADR-0022); add the missing ones.

## B. Sources

1. Feature: re-open every `@see` target online — still reachable, still saying what is cited; update the `[verified] YYYY-MM-DD` date in `docs/references.md`.
2. Project: the weekly link check (`links.yml`) is green — run it on demand if needed; `[unverified]` rows: try again, or keep them uncited.
3. No `@see` points to an `[unverified]` row (lint guarantees it; check no disable directive bypasses it).

## C. Provenance (no copied code)

1. Every function is written from its derivation or specification, not from third-party code; its TSDoc formula matches its body.
2. No comment, identifier pattern or structure taken from a known library; when in doubt, search the distinctive line on GitHub code search.
3. Licenses: GPL / LGPL code was only read (`docs/research/` §4); no dependency added without its row in `docs/tooling/dependencies.md`.

## D. Duplicates

1. Exported API (`npm run docs:api`, or `src/**`): no two functions for one concept, no near-identical helpers.
2. Derivations and references: no two documents for the same result; no URL under two IDs (`tests/references.test.ts`).
3. Tests: no copy-pasted test hiding a missing case.
4. Named calculations (ADR-0049): no operation or run of steps that a field names left as bare arithmetic; each one left inline is listed with its reason.

## E. Consistency

1. Code ↔ TSDoc ↔ derivation ↔ `docs/domain/` ↔ ADRs say the same thing.
2. Open questions of `docs/domain/README.md`: still open, or answered by the feature.
3. `AGENTS.md`, `CLAUDE.md`, `docs/conventions/`, `docs/tooling/` describe what the project really does.
4. Backlog: statuses, feature plan, epic table up to date.

## F. Quality gates

1. `npm run check:all` green; coverage 100 %; `npm run build` green.
2. No `.skip` / `.only` in tests; every disable directive (`oxlint-disable`, `lint-disable`, Rust `allow`) has a justification still valid and its row in the [register of exceptions](./exceptions.md).
3. The feature is demonstrable (playground or tests) for its `VAL` story.
4. Agent configuration (`AGENTS.md`, `CLAUDE.md`, `.claude/rules/`, skills, agents) still matches the project; contradictions removed.

## G. Next feature

List the sources and questions the next feature will need: they feed its research spike.

## H. Light evolvability check

The six questions of the [evolvability review](../process/) in one line each (size, tools, fitness functions, patterns, agent configuration, process). What deserves action goes to the [improvement log](../process/improvements.md), treated at the epic retrospective (ADR-0023).

## Findings table (in the `AUD` story)

| Check | Result | Evidence | Action |
| --- | --- | --- | --- |
| A4 — mutation of `filletSetback` (sign of τ) | caught | `fillet-setback.test.ts` "direction" fails | — |
