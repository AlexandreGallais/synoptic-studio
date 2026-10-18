---
name: story
description: Implement one backlog story (US, EN) from branch to pull request. Use when the Product Owner says to do the next story or names a ready story.
argument-hint: "[story id, e.g. EN-005]"
---

# Implement a story

Story: $ARGUMENTS (default: the next unfinished story of the started feature, printed by the session-start hook). In a run, a worker does steps 2 to 6 by its [brief](../worker-brief/SKILL.md) and the main thread steps 7 and 8 (`/run`).

1. **Check it may start**: status `ready`, or the Product Owner's explicit go-ahead in this conversation (a `/run` is one, ADR-0072); otherwise stop and ask. `/triage` if issues are open (`gh issue list`).
2. **Read** the story, its feature (criteria, plan), the domain file, the ADRs, derivations and `REF-*` it cites, and the research note of the feature's spike; a source read again comes from the library first. A business rule or a source missing: the stop table of `AGENTS.md`.
3. **Worktree and branch** from the feature branch on GitHub, never from another story's: `git fetch origin`, then `target/release/worktree-new <ID>` from the main checkout's root (`.claude/worktrees/<id>` on `<type>/<id>-<topic>`, EN-052). Every Bash call then starts with `cd "<worktree>" || exit 9`; the main checkout is never switched.
4. **The T0 commit** when the story is not `ready` yet: `target/release/backlog-status <ID> ready`, `docs(backlog): set <ID> ready`, footer `Refs: <ID>`.
5. **Per task** (`T1`, `T2`…), one commit each, footer `Refs: <ID>.Tn`, the step written to `.git/metrics/step`:
   - tests first (`AGENTS.md`, Code), the story's edge cases among them, each new test broken once on purpose (ADR-0064), run and seen failing;
   - the implementation, as `AGENTS.md` and `docs/conventions/` say;
   - `npm run fix`, then the touched tests (`npx vitest run <paths>`, `cargo test -p <crate>`); `check:all` runs once, at the push (EN-093); tests are committed with their implementation, since the pre-commit hook needs a compiling tree, by `target/release/git-sequences commit -F <message file> <paths>` (EN-100); every agent tool is called by its binary from the worktree's root, as `docs/tooling/agent-tools.md` maps its script (ADR-0078).
6. **Docs** in the same task as the code they describe: domain, derivations, references, playground. A user story (`US`) gets its **Product Owner test card** (`## Product Owner test`).
7. **The `done` commit**: the tasks ticked, `target/release/backlog-status <ID> done`, `docs(backlog): set <ID> done`, footer `Closes: <ID>`. A story that changes a rule in `docs/domain/` brings, by this commit at the latest, the Q row of `docs/domain/README.md`, the decisions to confirm of its feature's `VAL` (`docs/backlog/stories/*-F<nn>-VAL-*.md`) and its guide page (`docs/guide/f<nn>-*.md`, `e<nn>-*.md` for an epic's story), so that the rule's readers see it changed; a doc the change does not affect is named in the footer as `Rule-docs: none affected, <why>`, exactly so. The `commit-msg` hook refuses the commit otherwise and names each missing doc (`tools/rule-docs`, EN-059).
8. **Push and pull request** from the worktree; the pre-push hook runs `check:all`, the story's only check, and no CI runs on a story pull request (ADR-0072):
   - in a run: `npm run story:finish -- feature/f<nn>-<topic> "<title>" <body file>`, which pushes, opens the pull request with the `autonomous` label, waits for GitHub's merge and cleans up (`/run`, EN-051);
   - outside a run: `git push -u origin HEAD` (or `--push` on the last `target/release/git-sequences commit`), then `gh pr create --base feature/f<nn>-<topic> --title "<type>(<scope>): <summary>" --body-file <filled .github/pull_request_template.md>`, not a draft, without the `autonomous` label: the Product Owner merges it.
9. **Stop**: a summary to the Product Owner in French (`AGENTS.md`, Definition of done), with the functions and their `@kind` and `@see`, the sources, what was checked and how, the pull request's link. Never merge it.

After the merge, anything `story:finish` did not remove (a merge by the Product Owner, a stopped story's worktree) goes with `target/release/branches-clean`. The feature reaches `develop` through its pull request at `VAL`, then `main` with the next release.
