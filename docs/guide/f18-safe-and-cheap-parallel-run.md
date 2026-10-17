# F18 — A safe and cheap parallel run, and E15's leftovers

## What changed for you

During a run, the agent works with up to three helpers at once, each in its own copy of the repository. In E15 four of them ran their checks together and froze the computer, and their branches conflicted on the same files. F18 makes the tools enforce the limits instead of a sentence in the rules: a fourth helper is refused, a second heavy check waits for the first, and the files every branch writes are written once, at merge. In the editor, one thing changed: the font list stays greyed while the default font is read a second time (US-061).

## The words

| Word           | In plain words                                                                                                                                                                |
| -------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| subagent       | a helper the agent starts for one story; it works in the background and reports at its end                                                                                    |
| worktree       | a second working copy of the repository, on its own branch, in `.claude/worktrees/<story>`; several stories can then be worked on at once without touching each other         |
| heavy command  | a command that keeps every core of the computer busy for minutes: `check:all`, `npm run test`, `docs:build`, `golden:check`, `mutate`, and `git push`, which runs `check:all` |
| lock           | a file in `.git/` that a heavy command holds while it runs; the next one waits, and says who holds it                                                                         |
| shared file    | a file every story would write: the logs, the backlog's indexes, the feature file, the checklists                                                                             |
| merge by union | Git keeps the lines both branches added, one after the other, instead of stopping on a conflict                                                                               |
| mutation       | a small mistake put in the code on purpose (`*` turned into `/`): a good test fails on it, the mutation is then **caught**; a test that still passes lets it **survive**      |

## The figures, before and after

Before: E15, as its retrospective (RET-003) measured them. After: F18's tools.

| What                                    | In E15                                                      | After F18                                                                                        |
| --------------------------------------- | ----------------------------------------------------------- | ------------------------------------------------------------------------------------------------ |
| Subagents at once                       | up to 4, on 2026-10-16: the computer froze, a forced quit   | 3 at most: a fourth refused by a hook, naming the three running, and by Claude Code's own cap    |
| Heavy commands at once                  | several; a load average of 93 at the end of one `check:all` | one: the next waits on `.git/heavy.lock` and says once which process, command and folder hold it |
| A lock left by a forced quit            | no lock                                                     | taken over at once: the system frees it when its process ends                                    |
| `check:all` under load                  | 217 s failed, then 170 s passed (54–57 s alone)             | never two at once; the tests on half the cores, timing tests given four times their time         |
| `[F15.AC4]` under load                  | 157 ms where 100 ms was the limit                           | 1 000 texts measured in under 0.4 s                                                              |
| The stories index                       | rewritten by hand, whole, 18 times                          | generated from the story files; `check:all` checks it                                            |
| Conflicts on the logs                   | left open between parallel branches                         | none: the three logs merge by union, a test keeps their rows unique and dated                    |
| A story branch writing a shared file    | possible                                                    | refused at the push, naming the file and the commit; the main thread writes it at merge          |
| Two stories finishing at once           | `story:finish` failing from a worktree                      | one merge at a time, from a worktree too; a branch deleted only once merged                      |
| Compound commands refused in a worktree | 549                                                         | the worker brief, and the guard naming the plain form                                            |
| The session report                      | one turn only                                               | the whole conversation with `--all`; background subagents on the status line                     |
| A checklist                             | 53 lines, its panel over the drawing                        | at most 12 lines per feature, a column beside the drawing, drafted by `val:prepare`              |
| Font tests                              | four read macOS's own fonts, skipped elsewhere              | every font test runs on every system, from Inter Regular's tables (EN-067)                       |

The tokens per story (about 24 M in E15) are measured over the whole of E17 at its retrospective.

## Each criterion and its evidence

Every test named here carries the criterion's tag, `[F18.ACn]` in TypeScript or `f18_acn_…` in a Rust tool.

| Criterion                                                 | What you get                                                                                              | Evidence                                                                      |
| --------------------------------------------------------- | --------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------- |
| AC1 release pull request on `develop`                     | release-please opens its release pull request into `develop`; on `main` it only tags (ADR-0067)           | `tests/release-flow.test.ts`                                                  |
| AC2 a fourth subagent refused                             | the hook `limit-agents` refuses an `Agent` or `SendMessage` call that would start a fourth                | `tools/hooks/tests/limit.rs`, `tasks.rs`                                      |
| AC3 one heavy command at a time                           | `tools/heavy-lock/`, the waiting line, a lock left behind taken over                                      | `tools/heavy-lock/tests/lock.rs`                                              |
| AC4 logs merged by union                                  | `.gitattributes`: `merge=union` on the journal, the improvement and feedback logs                         | `tests/logs.test.ts`                                                          |
| AC5 the stories index generated                           | `npm run backlog:index` (`tools/backlog-index/`)                                                          | `tools/backlog-index/tests/check.rs`, `tests/backlog.test.ts`                 |
| AC6 a shared file refused on a story branch               | the pre-push hook names the file and the commit                                                           | `tools/shared-files/tests/shared.rs`                                          |
| AC7 one merge at a time                                   | `story:finish` under `.git/merge.lock`, from a worktree                                                   | `scripts/story-finish.test.ts`                                                |
| AC8 worktrees named after their story                     | `npm run worktree:new -- <ID>`; `golden:write` refused in a worktree                                      | `tools/worktree/tests/plan.rs`, `.claude/hooks/guard-bash.test.ts`            |
| AC9 the whole session reported                            | `npm run metrics:session -- --all`; background subagents on the status line                               | `scripts/session-report.test.ts`, `.claude/hooks/status-line.test.ts`         |
| AC10 timing tests under load                              | `[F15.AC4]` at 1 000 texts in under 0.4 s                                                                 | `src/io/font-face/parse-font-face.test.ts`                                    |
| AC11 `epic:check` says what failed                        | failing tests, refusals counted apart, calls lost at a forced quit; `golden:check` at the push            | `scripts/epic-check.test.ts`, `tests/pre-push.test.ts`                        |
| AC12 a short checklist                                    | at most 12 lines per feature, beside the drawing; `npm run val:prepare -- <feature>`                      | `playground/checklists/checklist-line-limit.test.ts`, `playground/checklist/` |
| AC13 the register of exceptions; a 192 MB collection read | every `eslint-disable` directive listed with its reason; a font collection read without copying its bytes | `tests/exceptions.test.ts`, `src/io/font-collection/`                         |
| AC14 the font list held, a refused face not measured      | US-061                                                                                                    | `playground/default-font/default-font.test.ts`                                |
| AC15 `npm run mutate`                                     | caught and surviving mutations, under the heavy lock                                                      | `tools/mutate/tests/run.rs`                                                   |
| AC16 a done commit that changes a rule brings its docs    | the `commit-msg` hook refuses it without the Q row, the decisions to confirm and the guide page           | `tools/rule-docs/tests/`                                                      |
| AC17 E15's long pages shortened                           | the Text section of `shapes.md`, the Colors of `interaction.md`, `DERIV-text-box`                         | `tests/long-page-bullets.test.ts`                                             |
| AC18 modules per folder counted; no spread in a fold      | ADR-0043's signal of 15 modules counted on disk                                                           | `tests/folders.test.ts`, `eslint/settings/identifiers.test.ts`                |
| AC19 the CI reads a real font file                        | Inter Regular read by every font test, on every system                                                    | `src/io/font-face/`, `src/io/font-collection/`, `playground/fonts/`           |
| AC20 explicit targets; Vitest on half the cores           | no check walks the copies in `.claude/worktrees/`                                                         | `tests/check-targets.test.ts`, `tests/vitest-workers.test.ts`                 |

## How to try it

Run `npm run dev` and open `http://localhost:5173/?checklist=E17`: E17's checklist, F18's 12 lines, each with a check box; the ticks stay in your browser. Most lines are commands to run in a terminal at the repository's root, the main checkout on `develop`:

| Try                                                                          | You should see                                                                                                                     |
| ---------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------- |
| `npm run test` in two terminals at once                                      | the second: `heavy lock held by process <pid> (npm run test in <folder>, for <n> s): waiting`, then it runs                        |
| ask the agent to start four subagents at once                                | the fourth refused: `At most 3 subagents at once (ADR-0063): 3 running — …`                                                        |
| `npm run worktree:new -- VAL-019`, then `npm run worktree:new -- US-61`      | answered at once: `… val-019 already exists: remove it or name another story`, then `` `US-61` is not a story id such as US-061 `` |
| `npm run backlog:index`, then `git status`                                   | no file changed                                                                                                                    |
| `git check-attr merge docs/process/journal.md`                               | `merge: union`                                                                                                                     |
| `npm run metrics:session -- --all --markdown`                                | the time and tokens of the whole conversation                                                                                      |
| `npm run epic:check -- E17 2026-10-16` (a few minutes: it runs `check:all`)  | a table: failed calls, `apart: <n> refused by the worktree isolation`, calls lost at a forced quit                                 |
| `npm run mutate -- src/math/angle/central-angle.ts -- npx vitest run {test}` | three `caught` lines, then `3 caught, 0 survived, 0 file(s) skipped`                                                               |
| Chrome, Inter not installed: Text tool, **Installed fonts**, allow           | when the counting line ends, the list stays greyed: `Reading <family>, the default font: …`; then enabled                          |
| `npx vitest run src/io/font-face src/io/font-collection playground/fonts`    | every test passed, none skipped                                                                                                    |

The documentation site, this page included: `npm run docs:dev`.

## Decisions confirmed

Answered by the Product Owner on 2026-10-17; each also has its line in the checklist.

| Decision               | Answer                                                                                                                                     |
| ---------------------- | ------------------------------------------------------------------------------------------------------------------------------------------ |
| US-061, a refused face | dropped: a face the browser refuses at the second reading is no longer measured; a text asking for it uses the nearest face held           |
| ADR-0067, decision 2   | before each release, the Product Owner merges release-please's release pull request into `develop`, then `develop` → `main` (#259 for 0.3) |
| ADR-0068               | the Markdown tables without padding move to F19's formatter; F18 keeps the logs merged by union                                            |
| EN-062                 | not accepted: font tests on macOS's own fonts only; EN-067 made every font test run on every system                                        |

## Limits

- The tables of the logs are still padded by Prettier; F19's formatter removes the padding (ADR-0068).
- The tools are written in Rust from now on; the TypeScript ones (`story:finish`, `epic:check`, the status line) are rewritten by F19.
- The guided steps and the E15 checklist lines that import Arial, Helvetica.ttc, STIXGeneral or Apple Color Emoji from macOS's folders stay as they are: they are F15's and F17's lines, tried on a Mac.
- The tokens per story are compared with E15's at E17's retrospective.

## Validation

Not yet validated (`VAL-019`).

| Remark | What was done or where it went |
| ------ | ------------------------------ |
