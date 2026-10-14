# ADR-0050 — The agent's tools, parallel checks, and validated epic stories that merge by themselves

**Status**: Accepted (Product Owner, 2026-10-14, at RET-002). Extends ADR-0045 to an epic's closing stories.

## Context

E14's metrics showed where the time went:

- the model writing commands it had written before (1 147 Python edit scripts);
- the checks run one after the other at every push (49 pushes, about 44 minutes);
- `npm run fix` at 28 s each time;
- a status line that showed a refused background command as running for three hours.

The Product Owner asked for tools instead of repeated scripts, for faster checks, for more data on each call, for a status line that says when the chat is free, and for the closing stories of an epic to merge without a separate step once they validate them.

## Decision

1. **The agent's tools are repository scripts.** They are written in TypeScript like the others, tested and linted, and each is listed with what it replaces in `docs/tooling/agent-tools.md`. Tools to write later are specified there with the signal that calls for them. A project of tools in Rust waits until a tool's own execution, not its writing, is measured as the cost.
2. **`check:all` runs in four parallel lanes.** On this machine it takes 54 s instead of 90 s, and the playground is still built after the site. `npm run fix` and lint-staged keep ESLint's cache. The pre-push lint does not use it, so that a type changed in one file still reaches the files that use it.
3. **A validated epic story merges by itself.** Once the Product Owner validates an epic's `NAM`, `REV` or `RET`, its pull request into `develop` gets the `autonomous` label and GitHub merges it with a merge commit, as a validated feature does (ADR-0045).
4. **GitHub deletes a merged branch** (`delete_branch_on_merge`). `npm run branches:clean` then only prunes the local copies.
5. **Each tool call's record holds the session's state.** That state is the status line's snapshot, written at each refresh: the turn's tokens, the conversation memory, the background tasks and the step. The record also holds the call's start, the size of its input and result, and the subagent's id. The metrics report splits each turn between the tools and the model.

## Consequences

- The run skill uses `npm run story:finish` and `npm run backlog:status`; the retro skill runs `npm run epic:check`; a refactor uses `npm run golden:write` and `golden:check`.
- The status line counts a monitor as a background task, ends a task answered with an error, and shows each background task on its own line.
- The two `unicorn` rules whose autofix writes `Iterator.concat` and `Iterator.zip`, missing from Node.js 24, are off until the Node.js LTS ships them.

## References

ADR-0023, ADR-0038, ADR-0045, ADR-0049, `docs/tooling/metrics.md`, `docs/tooling/agent-tools.md`
