# ADR-0085 — A tool that fails is a card; tools change only at an epic's end

**Status**: Accepted (Product Owner, 2026-10-18, the card rule during REV-004, its home at RET-004's P9). Amends ADR-0074: its "method and tools" point at each epic end reads every card first; ADR-0074's other decisions stand. Supersedes, in ADR-0072 (The run, decision 2), "a comment about tools … goes to another agent, on `develop`": such a comment becomes a card.

## Context

- During E17's run the agent fixed its tools as they failed: each fix took a worktree, a merge, a rebuild of the main checkout's binaries and a rebase of every running story, and changed the tools other stories were using.
- On 2026-10-18 nine tools failed or did not do what was needed in one day; the main thread wrote each as a card — how the tool was used, why it failed, what was done instead — in the private records' folder, outside the repository, where a worker can neither read nor write.
- The cards hold no personal data.

## Decision

1. **A tool that fails during an epic's work is worked around, never fixed then.** The worker names it in its report; the main thread writes its card. A comment of the Product Owner about a tool becomes a card too.
2. **The cards live in `docs/process/tool-incidents.md`**, a dated card each (the use, the failure, what was done instead), written by the main thread at merge like the other logs, merged by union; a worker's card goes into its story file under `### Incidents`, for `move-rows` to move.
3. **No tool writes the cards**: a few lines each; a test checks their form.
4. **The retrospective reads every card first**, at its "method and tools" point, each with a recommendation in its opening table; the tools change then, in the stories it decides.

## Consequences

- `tools/shared-files` lists `tool-incidents.md` among the logs, `move-rows` takes `### Incidents`, and `tests/logs.test.ts` checks the cards' form: a story of [F23](../backlog/features/E02-F23-e17s-tool-incidents-fixed.md).
- The nine cards of 2026-10-18 moved into the repository at RET-004.

## References

ADR-0065, ADR-0072, ADR-0074, `docs/process/retrospectives/E17-retrospective.md`
