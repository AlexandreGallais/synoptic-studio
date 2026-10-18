# ADR-0064 — Checks by level: the story, the feature, the epic

**Status**: Accepted (Product Owner, 2026-10-16, at RET-003), applied from E02. Supersedes the reviews per story of ADR-0051, decision 3.

## Context

- During F05 to F15, each story had three independent reviews (`auditor`, `verifier`, `math-expert`). They found real defects in almost every story, and were the largest token line after the workers.
- F17 tried no review per story: the worker broke each of its tests once on purpose, and one thorough audit closed the feature. AUD-019 still found 8 test gaps and one fatal case (a 192 MB font collection), all before the Product Owner's validation.
- The Product Owner asked on 2026-10-15 for checks by level, to apply after E15.

## Decision

| Level | Check |
| --- | --- |
| Story | no independent review; the worker writes the story's edge cases into its tests and mutation-checks each new test once |
| Feature | a thorough audit (`AUD`): mathematics, sources, mutations, brute force, duplicates, docs |
| Epic | a full audit beside the naming pass (`NAM`): duplicates, shareable code, named calculations, and the merge audit of the parallel branches |

## Reasons

| Option | For | Against |
| --- | --- | --- |
| Checks by level (chosen) | fewer tokens and less time per story; every defect still found before the validation | a defect found later in the feature, its fix a little costlier |
| Three reviewers per story | defects found at once | about a third more tokens per story |

## Consequences

- The run skill and the audit skill describe the levels; the epic's `NAM` story carries the full audit.
- E02's retrospective compares tokens per story and defects found at the validation with E15's.

## References

ADR-0020, ADR-0049, ADR-0051, ADR-0063, `docs/conventions/audit.md`
