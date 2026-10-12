# ADR-0022 — Property-based tests, weekly link check, mutation tool deferred

**Status**: Accepted

## Context

Example-based tests check a few hand-computed values; a formula can pass them and still be wrong elsewhere (sign, quadrant, degenerate input). The library is made of small pure functions composed like formulas — the case where random testing of properties works best (`REF-QUICKCHECK`). The feature audit (ADR-0020) mutates functions by hand and re-verifies sources by hand; sources can disappear between audits.

## Decision

1. **Property-based tests** with `fast-check` and `@fast-check/vitest` (`test.prop`), in addition to the hand-computed examples, for `math` and `geometry` functions with a stateable invariant (symmetry, bounds, periodicity, composition). Examples stay mandatory: they carry the sourced values; properties widen the input space. Runs are seeded by fast-check and a failure prints its seed and shrunk counterexample (`REF-FASTCHECK-PBT`). New stories use them from EN-005; AUD-001 adds them to the functions of F01 already merged.
2. **Weekly link check** of all Markdown with lychee (`lycheeverse/lychee-action`, `links.yml`, also run on demand): external URLs and local file links, exclusions in `.lycheeignore` (`REF-LYCHEE-ACTION`).
3. **Mutation testing tool deferred**: StrykerJS with Vitest 5 runs no test against mutants (`REF-STRYKER-6210`; trial: 4 % score, every covered mutant survived). The audit's manual mutation spot-checks remain. Review when the issue is closed and released.

## Consequences

- Two dev dependencies (`docs/tooling/dependencies.md`), no runtime dependency.
- A property failure is a finding: either the function is wrong or the property is; both are recorded.
- A broken source link fails the weekly workflow and notifies the owner; the reference is fixed or marked `[unverified]`.

## References

`REF-QUICKCHECK`, `REF-FASTCHECK-PBT`, `REF-GOOGLE-MUTATION`, `REF-STRYKER-6210`, `REF-LYCHEE-ACTION`, ADR-0020, [research note 0002](../research/notes/0002-agent-and-verification-practices.md)
