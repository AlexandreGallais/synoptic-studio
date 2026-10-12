# ADR-0027 — Acceptance criteria as requirements, tests traced to them, a test report per release

**Status**: Accepted

## Context

V-model projects carry requirements, test descriptions, test reports and a traceability matrix as hand-written documents. Their functions — verifiable requirements, tests linked to them, results tied to an identified version, coverage of requirements by tests — matter for a product people must trust (domain purpose), even without the documents. The backlog already holds the requirements (feature acceptance criteria) and the plan (feature plan); stories close, criteria must survive. A requirement is good when it is appropriate, complete, conforming, correct, feasible, necessary, singular, unambiguous and verifiable (ISO/IEC/IEEE 29148, as listed in `REF-ISO29148-QUALITY`).

## Decision

The V-model is **not** adopted as a lifecycle: it needs the system and software specifications before development, whereas this project grows its requirements feature by feature (Product Owner, 2026-10-08). Only the light, automatic parts below are kept.

1. **Feature acceptance criteria are the requirements.** Numbered in the feature file, identified `F01.AC3`; written to meet the nine characteristics; checked at refinement and in the audit (section E).
2. **A done feature's criteria are never rewritten**: a change is a new story citing the criterion it changes; Git keeps the history; each release is the baseline.
3. **Tests name the criterion they verify**: a test proving a criterion carries `[F01.AC3]` in its title. A done feature with an untraced criterion fails `backlog.test.ts`.
4. **A test report per release**: the release workflow runs the tests and attaches a JUnit report (`test-report.xml`) to the GitHub release — the results tied to an identified version.
5. **Generated, not written**: the traceability matrix (criterion → stories → tests → result) becomes a generated page of the docs site (planned, E12); no hand-maintained matrix.
6. **The test strategy is a short stable section** of `docs/conventions/testing.md`: levels, environments, independence, success rules, regression.

## Consequences

- Who asks "which test proves this requirement, on which version?" gets an answer from the repository and the release page.
- Feature files become the requirements baseline; their criteria must be written with care at refinement.
- Readers coming from the V-model find the mapping in [process](../process/).

## References

`REF-ISO29148-QUALITY`, ADR-0020, ADR-0022, ADR-0023
