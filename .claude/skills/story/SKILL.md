---
name: story
description: Implement one backlog story (US, EN) from branch to pull request. Use when the Product Owner says to do the next story or names a ready story.
argument-hint: "[story id, e.g. EN-005]"
---

# Implement a story

Story: $ARGUMENTS (default: the next unfinished story of the started feature, printed by the session-start hook). In a run, a worker does steps 2 to 5 by its [brief](../worker-brief/SKILL.md) and the main thread step 6 (`/run`).

1. **Check it may start**: status `ready` in the records, or the Product Owner's explicit go-ahead in this conversation (a `/run` is one); otherwise stop and ask. `/triage` if issues are open (`gh issue list`).
2. **Read** the story and its feature from the records (`knowledge sql "SELECT text, criteria, tasks FROM stories WHERE id = '<ID>'"`), the rules, ADRs, derivations and `REF-*` they cite (`knowledge show <id>`), and the research note of the feature's spike. A business rule or a source missing: the stop table of `AGENTS.md`.
3. **Worktree and branch** from the feature branch on GitHub: `git fetch origin`, then `target/release/worktree-new <ID>` from the main checkout's root (`.claude/worktrees/<id>` on `<type>/<id>-<topic>`). Every Bash call then starts with `cd "<worktree>" || exit 9`; the main checkout is never switched. A story not `ready` yet: `target/release/backlog-status <ID> ready`, a row of the records, no commit.
4. **Per task** (`T1`, `T2`…), one commit each, footer `Refs: <ID>.Tn`, the step written to `.git/metrics/step`:
   - tests first (`AGENTS.md`, Code), the story's edge cases among them, each new test broken once on purpose, run and seen failing;
   - the implementation, as `AGENTS.md` and `docs/conventions/` say;
   - `npm run fix`, then the touched tests (`npx vitest run <paths>`, `cargo test -p <crate>`); `check:all` runs at the push; committed by `target/release/git-sequences commit -F <message file> <paths>`.
5. **The records** in the same task as the code they describe: a rule changed or added, its example with it; sources, derivations; a user story (`US`) gets its **Product Owner test card** in its row (`test_card`).
6. **Push and pull request** from the worktree; the pre-push hook runs `check:all`, only the checks the change needs, and no CI runs on a story pull request:
   - in a run: `npm run story:finish -- feature/f<nn>-<topic> "<title>" <body file>`, which pushes, opens the pull request with the `autonomous` label, waits for GitHub's merge, marks the story done in the records and cleans up (`/run`);
   - outside a run: `git push -u origin HEAD`, then `gh pr create --base feature/f<nn>-<topic> --title "<type>(<scope>): <summary>" --body-file <filled .github/pull_request_template.md>`, not a draft, without the `autonomous` label.
7. **Stop**: a summary to the Product Owner in French (`AGENTS.md`, Definition of done), with the functions and their `@kind` and `@see`, the sources, what was checked and how, the pull request's link. Never merge it.

After the merge, anything `story:finish` did not remove goes with `target/release/branches-clean`. The feature reaches `develop` through its pull request once its audit ends, then `main` at the next review.
