# ADR-0049 — Every calculation named, and a naming pass closes each epic

**Status**: Accepted (Product Owner, 2026-10-14). Extends ADR-0023: a third story closes each epic.

## Context

F13 went over the whole code base twice and turned 33 inline calculations into named functions — a rotation, a signed angle, a regular partition — each with its source; a golden file and the audit proved that no result changed. The Product Owner wants this to be a standing rule of the project, not a one-off feature: every function and every step must mean something that a field names, and the code base must be checked for it again regularly, since one pass never catches everything (F13's second pass found what the first one missed).

"A field" is wide: mathematics, geometry, physics, technical drafting, industrial practice, the laboratory — whatever names the operation.

## Decision

1. **While writing**: an operation, or a run of steps that a field names, is a call to a function of that name, with its `@see` source (a `REF-*` or a `DERIV-*` step); a process is named the same way, as a `procedure`. Bare arithmetic stays only where no field names it, or where naming it would change a floating-point result — then a comment says why.
2. **At each story's review**: the `auditor` reports an unnamed calculation in the diff, with the name and source it proposes; the feature's `AUD` lists those left inline, with their reason (`docs/conventions/audit.md` §D).
3. **At the end of each epic**: a `NAM` story, attached to the epic like `REV` and `RET`, runs before the review: two passes over the whole code base (the second on the result of the first), the `math-expert` checking each name against its source, and every result proved unchanged bit for bit (a golden file written before, compared after each pass; no expected test value edited). No new feature, no business rule changed.
4. `tests/backlog.test.ts` requires the `NAM` story in every started epic's closing, except E01 (closed before this decision) and E14, whose naming pass is F13.

## Consequences

- `AGENTS.md`, `docs/conventions/naming.md`, `docs/conventions/audit.md`, `docs/process/agile.md`, `docs/backlog/README.md`, the `run` skill and the `auditor` agent say so; a template `docs/backlog/_naming-pass.md`.
- An epic's closing is `NAM` → `REV` → `RET`; the review then shows code that already reads as its theory.
- The improvement log's row on making the golden file a repository script becomes the tool of every `NAM` story; until it exists, the story writes its own golden file, as F13 did.

## References

ADR-0011, ADR-0023, F13, research note 0016
