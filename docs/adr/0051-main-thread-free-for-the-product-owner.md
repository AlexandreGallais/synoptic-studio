# ADR-0051 — The main thread kept free: stories worked by subagents

**Status**: Superseded by ADR-0063 (2026-10-16, RET-003), which keeps and amends it. Accepted for a trial (Product Owner, 2026-10-14, at RET-002): the runs of E15 work this way; E15's retrospective measures it, then keeps or drops it.

## Context

During a run the main agent works on the main thread. A message from the Product Owner waits until its turn ends, sometimes for an hour. The Product Owner proposed another way: each task goes to a subagent, so that the main thread stays free and they can talk with the agent at any time. That needs rules, so that two agents never work on the same thing. They also want to see what each subagent does.

## Decision

1. **The main thread dispatches.** For each story of a run, the main agent starts a worker subagent in its own Git worktree (`isolation: "worktree"`), from the up-to-date feature branch. It gives the worker the story and the run's rules. It then answers the Product Owner while the worker works.
2. **One worker per feature branch at a time.** A feature's stories build on each other, so they stay in order. Only work that touches separate files may run at the same time: the research of the next feature, the documentation, the reviews.
3. **The worker owns its story.** It writes the code, the tests and the docs, commits each task, and pushes through the pre-push hook. It stops before the pull request. The main agent then runs the reviews (`auditor`, `verifier`, `math-expert`), sends the findings back, and finishes the story (`npm run story:finish`).
4. **Visible work.** The status line shows each worker and the call it makes now, and `npm run activity` shows its latest calls and outputs.
5. **Questions go through the main thread.** A worker that needs a business decision stops with its question. Only the main agent asks the Product Owner.

## To weigh in the trial

- **Cost.** A worker reads the context again: the metrics of the trial compare tokens per story with E14's.
- **Reviews.** Subagents cannot start subagents, so the reviews stay with the main agent.
- **Conflicts.** One worker per feature branch and one worktree each leave none on the code. The backlog files a story changes stay its own.

## Amendment (2026-10-14)

Added by the Product Owner during E15's run, so that the chat is always free:

1. **The main thread only dispatches.** Every piece of work goes to a subagent: research, stories, tool or rule changes.
2. **Each comment is acknowledged at once.** The main agent says it is taken into account and when: now, or once a named running agent has finished, if the comment concerns that agent's work. Comments may come one after another without waiting; each is acknowledged.
3. **Tool and rule comments run in parallel.** A comment about tools, rules, skills or configuration, unrelated to the running story, goes at once to another agent, on `develop`, in its own worktree.
4. **No two agents on the same files.** The main agent knows each running agent's topic and files.

## References

ADR-0028, ADR-0045, ADR-0050, `docs/tooling/agent.md` (parallel sessions)
