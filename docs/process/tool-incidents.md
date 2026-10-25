# Tool incidents

One card per time a tool failed or did not do what was needed: how it was used, why it failed, what was done instead (ADR-0085). A tool is never fixed during a feature's work: the cards are read at the next review, held at each return of the Product Owner, at its "method and tools" point, and the tools change in that review or in the stories it decides (ADR-0086). Moved by the main thread in the story's `done` commit (ADR-0091); a worker's card goes into its story file under `### Incidents`, its heading `#### <date> — <title>`.

The cards met up to 2026-10-25 were fixed or decided at REV-009 (its story, the tool cards and the telemetry), those of E17 at RET-004 ([record](./retrospectives/E17-retrospective.md)); the cards below are still open.

## 2026-10-23 — `mutate`, a run stopped

- Used: `npm run mutate -- <27 files> -- npx vitest run {test}` in the background, stopped before its end.
- Failed: the run, stopped, left its temporary worktree registered, its clean-up never run; `{test}` must be quoted in zsh, or it reaches the tool dropped.
- Instead: `git worktree remove --force` on the copy, `'{test}'` quoted.

## 2026-10-24 — `git push origin --delete`, over 3 minutes

- Used: `git push origin --delete <branch>` from the main checkout after a merge, 12 calls since 2026-10-24 (REV-009's telemetry).
- Failed: a median of 3 min 15 s, though the pre-push skips its checks on a deletion-only push; the cause, a lock wait or another ref pushed with it, is not found.
- Instead: `gh api -X DELETE repos/<owner>/<repo>/git/refs/heads/<branch>`, as `.claude/rules/tooling.md` says.

## 2026-10-25 — the pre-push `check:all`, a test lane failing once

- Used: `git push origin origin/develop:refs/heads/feature/f48-the-knowledge-database` from the main checkout, to create F48's feature branch.
- Failed: `check:all failed in 189.2 s: test`, the one line printed; the failing test is not named, and `npm run test` alone then passed (640 files), as did the same push again.
- Instead: the push run again in the background, its whole output kept in a file; the cause, a test slowed or racing under the full check, is not found.
