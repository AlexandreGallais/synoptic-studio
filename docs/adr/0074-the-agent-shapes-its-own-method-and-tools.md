# ADR-0074 — The agent shapes its own method and tools, the Product Owner decides

**Status**: Accepted (Product Owner, 2026-10-17, review of the ADRs, EN-094). Amended by ADR-0085 (2026-10-18, RET-004): a tool that fails is a card, read first at the "method and tools" point; tools change only at an epic's end.

## Context

- The method of this project — epics, features, stories, reviews and retrospectives — was written for a team of people, then fitted to one agent and its subagents run by run: the main thread kept free (ADR-0051, removed: ADR-0101), the limit on agents (ADR-0072), the shared files written at merge (ADR-0065), the tooling made fast for the agent (ADR-0069).
- Each of these changes came from a friction the agent met: a freeze, a conflict on a shared file, a slow check, a wait. They were often seen late, in the retrospective's metrics, rather than written when they happened.
- On 2026-10-17 the Product Owner decided to give the agent growing freedom over its own method and tools, so that it proposes what fits an agent best, with evidence, rather than following a method made for people.

## Decision

1. **Log as it happens.** While working, the agent writes in the improvement log (`docs/process/improvements.md`), at once, each friction, error or slow step it meets, and each reading that suggests a faster way to work, one cheaper in tokens, or a healthier one for the project. A worker writes its rows in its story file, under `## For the main thread` (ADR-0065).
2. **A "method and tools" point at each epic end.** At the retrospective, beside its tooling pass, the agent presents what it wants to change in its own work, with its evidence — metrics, incidents, sources read:
   - its method, the backlog method itself included when it does not fit an agent, for instance stories that compete for the same shared files;
   - new methods, and new Rust tools (ADR-0073), to adopt.
3. **The Product Owner decides** each proposal: an ADR, a story, a rule, a tool, or "not now" with its reason (the retrospective's decision step).

## Reasons

| Option | For | Against |
| --- | --- | --- |
| The agent proposes changes to its own method, the Product Owner decides (chosen) | the method fits the one who works by it; frictions written when fresh; each change backed by evidence | a point more at each retrospective |
| The method set by the Product Owner alone | one voice | frictions the Product Owner cannot see stay unseen until they cost |
| The agent changes its method by itself | faster | changes nobody decided; the rules of `AGENTS.md` would drift |

## Consequences

- `docs/process/agile.md` and the retro skill hold the "method and tools" point; the improvement log is fed during the work, not only at the retrospective.
- Rules hardened or relaxed at the review of every rule (every four to five epics) take these proposals as input.

## References

ADR-0023, ADR-0065, ADR-0069, ADR-0072, ADR-0073, ADR-0101 (ADR-0051, removed), EN-094
