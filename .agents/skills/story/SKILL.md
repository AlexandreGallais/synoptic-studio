---
name: story
description: Implement one backlog story (US, EN) from branch to pull request. Use when the Product Owner says to do the next story or names a ready story.
argument-hint: "[story id, e.g. EN-005]"
---

# Implement a story

Story: $ARGUMENTS (default: the next unfinished story of the feature in progress, printed by the session-start hook). A story is worked by a fresh worker in the background ([its brief](../worker-brief/SKILL.md)); the main thread makes the worktree and finishes it.

1. **May it start?** Status `ready` in the records, or the Product Owner's go-ahead in this conversation (a `/run` is one); otherwise ask.
2. **Read** it from the records (`target/release/knowledge show <ID>`), its feature, and the rules, decisions, derivations and sources it cites. A rule or a source missing: stop and ask (`AGENTS.md`).
3. **Worktree**: `git fetch origin`, then `target/release/worktree-new <ID>` from the main checkout, which is never switched; the status set by `target/release/backlog-status <ID> in-progress`.
4. **The worker** does the tasks, a commit each (`Refs: <ID>.Tn`), tests before or with the code, the records with the code, and pushes; the push runs only the checks the change needs.
5. A **user story** gets its Product Owner test card in its row (`test_card`): the steps in the playground and what each shows.
6. **Finish**: `target/release/heavy-lock --lock merge target/release/story-finish feature/f<nn>-<topic> "<title>" <body file>` in the worktree: it rebases the story on its feature branch, opens the pull request with the `autonomous` label, waits for GitHub's merge by a merge commit, sets the story `done` in the records and removes the worktree and branches.
7. **Report** in French, short: what was built, the evidence, what the Product Owner must decide.

What `story-finish` left behind goes with `target/release/branches-clean`.
