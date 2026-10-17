# ADR-0069 — Tooling as fast as it can be, for the agent

**Status**: Accepted (2026-10-17, decided by the Product Owner during F19's run).

## Context

- The agent runs the repository's tools hundreds of times a session: checks, formatter, linter, tests, the backlog and Git tools.
- A tool that takes 5 s instead of 1 s costs a person little, but costs the agent far more than the 4 s shown: a command not finished at once is sent to the background, then checked again and again until it ends, and each check adds a round of thinking. These rounds do not appear in the tools' times, yet they can double the time of a task (Product Owner, 2026-10-17).
- E17 already moves the tooling to Rust and Oxc (every new tool in Rust, 2026-10-16) and measures `check:all` by lane.

## Decision

1. **Speed is a requirement of every tool** of the repository, not only of the tests: each tool, script, hook and check is made as fast as it can be, aiming at a result before the agent's next step — under a second where the work allows it.
2. **Our own tools are written in Rust**; third-party tools are the fastest sound ones (Oxc, TypeScript 7), in line with E17's goal.
3. **Work is done once**: what is slow to prepare (a file read, a build, a parse) is prepared once and reused, and its preparation tested once (EN-089, EN-090).
4. **Each story that adds or changes a tool states its time** before and after, measured on this Mac.
5. **Each epic ends with a pass over the whole repository's tooling and tests** — duplicates, slow steps, waits — beside the naming pass (ADR-0049), however small its share of the time looks.

## Reasons

| Option | For | Against |
| --- | --- | --- |
| Speed as a requirement of every tool (chosen) | fewer background waits and check-again rounds for the agent; the gain multiplies over a session | rewriting tools costs work now |
| Speed only where a person would notice it | less work | the agent's waits stay hidden in its thinking time |

## Consequences

- A tool slower than it needs to be is a defect to log in `docs/process/improvements.md` and to fix in the next tooling story.
- Stories of tooling carry their timings; the retrospective reads them with the metrics.

## References

ADR-0049, ADR-0063, ADR-0064, [EN-089](../backlog/stories/E17-F19-EN-089-font-files-read-once-per-test-run.md), [EN-090](../backlog/stories/E17-F19-EN-090-tests-without-over-testing.md), [E17](../backlog/epics/E17-fast-tooling-with-oxc-and-rust.md)
