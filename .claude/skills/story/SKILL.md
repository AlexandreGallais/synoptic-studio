---
name: story
description: Implement one backlog story (US, EN) from branch to pull request. Use when the Product Owner says to do the next story or names a ready story.
argument-hint: "[story id, e.g. EN-005]"
---

# Implement a story

Story: $ARGUMENTS (default: the next unfinished story of the started feature, printed by the session-start hook).

1. **Check it may start**: status `ready`, or the Product Owner's explicit go-ahead in this conversation (a `/run` is one: ADR-0045). Otherwise stop and ask. Run `/triage` if issues are open (`gh issue list`).
2. **Read**: the story, its feature (acceptance criteria, plan), the domain file, linked ADRs, derivations and `REF-*` it cites, the research note of the feature's spike; a source to read again → `library_search` / `library_read` first. Missing business rule or source → the stop table of `AGENTS.md`.
3. **Branch** from the up-to-date feature branch `feature/f<nn>-<topic>` (ADR-0028; created from `develop` when the feature starts, ADR-0038), never from another story branch:
   `git switch feature/f<nn>-<topic> && git pull --prune && git switch -c <type>/<id-lowercase>-<topic>`
4. **T0 commit** if needed: status `ready` in the story file, `docs/backlog/stories/README.md` and the feature's table — `docs(backlog): set <ID> ready`, footer `Refs: <ID>`.
5. **Per task** (`T1`, `T2`…), one commit each, footer `Refs: <ID>.Tn`; the step written to `.git/metrics/step`:
   - `math` / `geometry`: write the tests first — examples with hand-computed values justified in a comment, degenerate cases, the story's edge cases, then properties (`test.prop`); break each new test once on purpose (no per-story review, ADR-0064); run them and see them fail;
   - implement: one function per file, kebab-case name, `@kind`, `@see` to a verified source, austere TSDoc;
   - `npm run fix`, then `npm run check`; commit tests with their implementation (the pre-commit hook needs a compiling tree).
6. **Docs** in the same task as the code they describe: `docs/domain/`, derivations, references, playground. A user story (`US`) also gets its **Product Owner test card** (section `## Product Owner test`).
7. **Last commit**: tick the tasks, status `done` in the story file, the stories index and the feature's table — `docs(backlog): set <ID> done`, footer `Closes: <ID>`.
8. `git push -u origin <branch>` — the pre-push hook runs `check:all`, the story's only check (ADR-0038; a failure blocks the push: fix and push again) — then
   `gh pr create --base feature/f<nn>-<topic> --title "<type>(<scope>): <summary>" --body-file <filled .github/pull_request_template.md>` (not a draft; in an autonomous run, `/run` adds `--label autonomous` after the auditor's review).
9. No CI runs on a story pull request: do not wait for one.
10. **Stop**: summarize to the Product Owner in French — functions with `@kind` and `@see`, sources, what was checked and how, link to the pull request; the session report only at a session's end (`--all`). Never merge it.

After the merge: `git switch feature/f<nn>-<topic> && git pull --prune && git branch -d <branch>`. The feature reaches `develop` through its feature pull request at `VAL`, then `main` with the next release (`/run`, Validation).
