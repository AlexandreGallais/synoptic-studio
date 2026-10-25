# Lessons

What went wrong in this project, and the check that now prevents it. Newest first; added at each retrospective, or as soon as a lesson is clear.

| Date | What happened | Now prevented by |
| --- | --- | --- |
| 2026-10-16 | Four subagents ran tests and checks at once; the computer froze and the session was force-quit | at most three subagents, one heavy command at a time, enforced by a hook and a lock (ADR-0072, EN-047) |
| 2026-10-16 | Parallel story branches edited the same indexes, logs and checklist; the conflicts stayed open | shared files written by the main thread at merge, generated indexes, unpadded tables, one merge at a time (ADR-0065) |
| 2026-10-16 | 549 commands refused in worktrees, each a round trip | the worker brief names the forms that fit a worktree; a guard answers at once (EN-052) |
| 2026-10-15 | The session report showed 10 h 40 of work as 2.8 s | cause found (one turn without `--all`); the report at a session's end with `--all` |
| 2026-10-15 | Validating E15 took 176 guided steps one after the other, slow and tiring for the Product Owner | one checklist page per epic: a line per option and per decision, a check box, tried in any order (US-051, EN-044) |
| 2026-10-13 | The agent waited for the CI after every story, at least 40 minutes over E01, while the pre-push hook had already run the same checks | checks run locally; one CI, on the pull request into `main` (ADR-0072, ADR-0088) |
| 2026-10-13 | A package released 36 seconds before the version check turned `main` red | a 7-day cooldown on versions (`.ncurc.cjs`) |
| 2026-10-13 | Dependencies were upgraded, a major among them, without reading what changed | the upgrade procedure reads the release notes first (`docs/tooling/versions-and-security.md`) |
| 2026-10-13 | The Product Owner's spoken words were quoted throughout a public repository | writing rules: decisions in neutral words; the `doc-editor` agent checks |
| 2026-10-13 | Choosing a rectangle after a converted free shape left its fractional width and empty radius, and every field turned red | whole fields and one radius on choosing a shape; a test of that path |
| 2026-10-13 | Nobody knew how long a story took nor how many tokens it used | metrics hooks, `target/release/metrics`, a session report at each session's end |
| 2026-10-13 | Hooks and scripts run by Node.js lost their `.ts` extensions to the import autofix | a `node-run` lint scope |
| 2026-10-09 | Validating a feature from a test card read beside the page was slow for the Product Owner | a guided test inside the playground: one step at a time, values typed by Next, what to look at; tested end to end |
| 2026-10-09 | Story branches merged by GitHub auto-merge stayed on the remote; deleting them with `git push --delete` ran the full pre-push check each time | delete each merged branch right after the merge, through the GitHub API (`/run` step 9) |
| 2026-10-09 | A wait loop that only looked for "merged" kept waiting on a failed check | statuses read, stop at the first failure; since ADR-0072, no CI on work branches |
| 2026-10-09 | `atan2(+0, −0) = π`: a zero-length edge after a negative edge gave a half turn instead of none; found by the independent auditor, not by the tests | zero vectors handled explicitly; signed zeros in the library rules; the auditor reviews every story |
| 2026-10-09 | The unused-imports autofix removed a type import written before its use; Vitest (types stripped) stayed green | `tsc` before each commit (library rules); `check:all` before push |
| 2026-10-09 | A spike (contour turning back) is consumed by its fillet: a behavior no rectangle test could show | auditor probes on non-rectangular contours; business question raised (Q16) instead of a silent choice |
| 2026-10-08 | Story statuses drifted in a feature table (four stories shown `draft` after being done) | `backlog.test.ts` checks parents' tables |
| 2026-10-08 | Dates written one day off in docs | session-start state; dates read from the environment, never guessed |
| 2026-10-08 | A mutation tool (StrykerJS) reported 4 % because, with Vitest 5, no test ran against mutants | trial before adoption; tool deferred with its tracking issue (ADR-0022) |
| 2026-10-08 | A commit refused (unknown scope) let the next commit swallow its files | commit one by one, stop at the first failure; `git status` before retrying |
| 2026-10-08 | A guard hook denied text quoting a guarded command inside a heredoc | the guard skips here-document bodies (2026-10-13) |
| 2026-10-08 | A cited reference (Graphics Gems) could not be read | derivation from read sources (`DERIV-fillet-setback`); never cite unread |
| 2026-10-08 | A stacked pull request was merged into another story branch | one branch per story from its feature branch, never stacked |
| 2026-10-08 | Release pull requests opened with `GITHUB_TOKEN` triggered no CI | release-please with a personal token, auto-merge when green |
| 2026-10-08 | TypeDoc failed in CI on undocumented type members, after a green local check | pre-push runs `check:all`, including the docs build |
| 2026-10-08 | Barrel autofix kept `export type *` once a folder held values | `barrels.test.ts` |
| 2026-10-08 | A status test silently skipped one index | every index tested to list its items; mutation-check of file-reading tests |
| 2026-10-07 | Plausible but invented details (an author name, a "holes counter-clockwise" rule) | sources recorded before use; business rules only from the Product Owner |
| 2026-10-07 | Linters pulled a vulnerable transitive package (`braces`) | `npm audit` blocking; alternative chosen (`@eslint/markdown`) |
| 2026-10-07 | An autofix removed a trailing slash and broke paths | paths built with `join()`; pitfall in rules |
