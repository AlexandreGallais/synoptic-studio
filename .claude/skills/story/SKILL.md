---
name: story
description: Implement one backlog story (US, EN) from branch to pull request. Use when the Product Owner says to do the next story or names a ready story.
argument-hint: "[story id, e.g. EN-005]"
---

# Implement a story

Story: $ARGUMENTS (default: the next unfinished story of the started feature, printed by the session-start hook).

1. **Check it may start**: status `ready`, or the Product Owner's explicit go-ahead in this conversation (a `/run` is one: ADR-0072). Otherwise stop and ask. Run `/triage` if issues are open (`gh issue list`).
2. **Read**: the story, its feature (acceptance criteria, plan), the domain file, linked ADRs, derivations and `REF-*` it cites, the research note of the feature's spike; a source to read again → `library_search` / `library_read` first. Missing business rule or source → the stop table of `AGENTS.md`.
3. **Worktree and branch** from the feature branch `feature/f<nn>-<topic>` on GitHub (created from `develop` when the feature starts, ADR-0072), never from another story branch: `git fetch origin`, then `npm run worktree:new -- <ID>` (`.claude/worktrees/<id>` on `<type>/<id-lowercase>-<topic>`, EN-052). Every Bash call then starts with `cd "<worktree>" || exit 9`; the main checkout is never switched.
4. **T0 commit** if needed: status `ready` by `npm run backlog:status -- <ID> ready` (the story file, the feature's table, and `docs/backlog/stories/README.md` generated again, EN-049) — `docs(backlog): set <ID> ready`, footer `Refs: <ID>`.
5. **Per task** (`T1`, `T2`…), one commit each, footer `Refs: <ID>.Tn`; the step written to `.git/metrics/step`:
   - `math` / `geometry`: write the tests first — examples with hand-computed values justified in a comment, degenerate cases, the story's edge cases, then properties (`test.prop`); break each new test once on purpose (no per-story review, ADR-0064); run them and see them fail;
   - implement: one function per file, kebab-case name, `@kind`, `@see` to a verified source, austere TSDoc; a new script, hook or agent tool in Rust (ADR-0073), a crate of `tools/` in the root Cargo workspace, checked by `check:rust` (E17, 2026-10-16, EN-068);
   - `npm run fix`, then `npm run check`; commit tests with their implementation (the pre-commit hook needs a compiling tree).
6. **Docs** in the same task as the code they describe: `docs/domain/`, derivations, references, playground. A user story (`US`) also gets its **Product Owner test card** (section `## Product Owner test`).
7. **Last commit**: tick the tasks, status `done` by `npm run backlog:status -- <ID> done` (the story file, the feature's table, the stories index generated again) — `docs(backlog): set <ID> done`, footer `Closes: <ID>`. When the story's commits change a rule in `docs/domain/`, the story brings, by this commit at the latest, the Q row of `docs/domain/README.md`, the decisions to confirm of its feature's VAL (`docs/backlog/stories/*-F<nn>-VAL-*.md`) and its guide page (`docs/guide/f<nn>-*.md`, `e<nn>-*.md` for an epic's story); a doc none of the change affects is stated in the footer by the line `Rule-docs: none affected, <why>`, exactly so, its reason after the comma. The `commit-msg` hook refuses the commit otherwise and names each missing doc (`tools/rule-docs`, EN-059).
8. **Push and pull request**, from the worktree; the pre-push hook runs `check:all`, the story's only check (ADR-0072; a failure blocks the push: fix and push again):
   - in a run: `npm run story:finish -- feature/f<nn>-<topic> "<title>" <body file>` — the merge lock, the push, the pull request with the `autonomous` label, GitHub's merge, the clean-up (`/run`, EN-051);
   - outside a run: `git push -u origin HEAD`, then `gh pr create --base feature/f<nn>-<topic> --title "<type>(<scope>): <summary>" --body-file <filled .github/pull_request_template.md>`, not a draft, without the `autonomous` label: the Product Owner merges it (`story:finish` always adds the label).
9. No CI runs on a story pull request: do not wait for one.
10. **Stop**: summarize to the Product Owner in French — functions with `@kind` and `@see`, sources, what was checked and how, link to the pull request; the session report only at a session's end (`--all`). Never merge it.

After the merge: `story:finish` has removed the worktree and the story's branch; anything left (a merge by the Product Owner, a worktree of a stopped story) goes with `npm run branches:clean`. The feature reaches `develop` through its feature pull request at `VAL`, then `main` with the next release (`/run`, Validation).
