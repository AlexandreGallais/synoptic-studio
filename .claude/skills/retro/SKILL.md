---
name: retro
description: Run the method's part of a review (REV story) — tool incident cards, improvement and feedback rows, what helped and hurt, evolvability review, playbook update. Use inside a REV, after its feature review.
argument-hint: "[REV id]"
---

# The method's part of a review

Review: $ARGUMENTS. The method is reviewed in the `REV` that also holds the feature review (`/review`). Facts come before opinions, and every change proposed carries its evidence. The span is everything since the last merge into `main`: keep each point to what that span shows.

1. **Check what the span produced**: `target/release/epic-check <epic> <day of the last merge into main>` for each epic its features belong to (check:all, branches, pull requests, issues, failed calls, calls left running, backlog, records); each "to look at" line explained or fixed.
2. **Gather the facts**: `target/release/metrics --since <day of the last merge into main>` (commands by count and time, slowest calls, time and tokens per story and task, the sources read most), the span's `git log`, the audits' findings, the improvement rows of the records, the duration of `check:all`. Read them as an engineer: a command run often or slowly asks for a tool, a cache or fewer runs; a story far above the others asks why. Then write the agent's own account of the span: where time and tokens went, what to change to go faster, spend less or do better.
3. **Open with one table** of every tool incident card and every improvement and feedback row, a recommendation per row, for the Product Owner to decide row by row, each saying in plain words what would change for them, with an example. Then ask what helped, what hurt, what they want changed.
4. **Method and tools**: every card is fixed in this review or decided; then what the agent wants to change in its own work — its method, new Rust tools — each with its evidence; the **Tools** section of `target/release/metrics`: unused, failing or slow tools.
5. **The agent's own review**: recurring mistakes, guards hit, rules ignored, context lost; the feedback rows grouped into habits.
6. **Evolvability review**, when the span gives it figures: size, tools, fitness functions, patterns, agent configuration, process, each "nothing yet", a story or an ADR; every "considered for later" trigger and every override of `package.json` checked again.
7. **Decide** with the Product Owner: each change becomes an ADR, a story, a rule, a check, or "not now" with its reason.
8. **Write** the review's record in the records (`knowledge add review -`), the treated rows marked done.
9. **The agent's configuration** (`AGENTS.md`, `CLAUDE.md`, rules, skills, agents): new lessons with the check that prevents each; a repeated mistake gets a check, not a sentence.
