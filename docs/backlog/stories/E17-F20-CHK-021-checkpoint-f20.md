---
id: CHK-021
epic: E17
feature: F20
title: Checkpoint in the middle of F20
status: ready
points: 1
---

# E17 · F20 · CHK-021 — Checkpoint in the middle of F20

Bring the docs and methods up to date in the middle of F20, with a light evolvability check (ADR-0028).

## Acceptance criteria

- Given the stories merged so far, when the checkpoint ends, then the docs, rules and tools describe what was built.

## Tasks

One task = one commit, referenced as `CHK-021.Tn`.

- [ ] T1 — Docs and rules in step; light evolvability check (1 h)

## Consistency after the four rewrites

Checked on 2026-10-17 over `AGENTS.md`, `CLAUDE.md`, `.claude/` (rules, skills, agents), `docs/conventions/`, `docs/process/` and `docs/tooling/`, after EN-096 to EN-099 were merged into F20.

| Check | How | Result |
| --- | --- | --- |
| Links to files | every relative link resolved from its page | none broken (the template `docs/backlog/_audit.md` resolves once copied into `stories/`) |
| Anchors across pages | every `page.md#anchor` against the target's headings, with the slugs of VitePress (`slugify` of `vitepress/dist/node`) | five links break on the site and seven on GitHub: below |
| Commands named | every `npm run <script>` of those pages against `package.json` | all exist; `lint:fix` is named only as dropped (`rules.md`) |
| Files named | every path in code font under `.claude/`, `docs/`, `tools/`, `scripts/`, `tests/`, `src/`, `playground/` | all exist, but placeholders and the layers still to come (`src/routing/`, `src/interaction/`, `architecture.md`) |
| Story references | the `/story` steps cited by `run`, `audit`, `retro`, `spike`; `/run` Validation step 8 cited by `retro` | all point to the right step after EN-096's renumbering |
| Rules in two places | the four stops (`docs/process/README.md`, run skill), the heavy commands (brief, `CLAUDE.md`, `commands.md`), the models (`agent.md`, each agent file), the Git tests' `GIT_*` rule (`tooling.md`, `commands.md`) | they agree |
| Rules lost | the trace tables of EN-096, EN-097, EN-098 and EN-099, a sample of the merged and moved rules looked for at their new place (WB13, WB16, CL4, AG47, AG48, To6, To21, AT3, Te21) | each found where its trace says |

Fixed here: EN-066's T3 named EN-096 as the Sonnet story; the trial ran on EN-098.

Not fixed, an improvement row below: the Markdown lint checks a page's own anchors with GitHub's slugs (`tools/lint/src/markdown/slugger.rs`), the site uses VitePress's, and they differ on a heading with `'` or `/`. `#a-runs-totals` (`metrics.md`, twice, and `agent-tools.md`) and `rules.md#the-formatters-settings` (`formatting.md`, `rule-catalogue.md`) follow GitHub and break on the site; seven links of `lint.md`, `functions.md` and `typescript.md` use VitePress's slugs and break on GitHub. Changing the in-page ones to the site's form fails the lint, so the choice of one slug for both is the tool's story, not a checkpoint edit.

## Evolvability, light

| Question | Answer |
| --- | --- |
| Size | F20 changed no file of `src/` nor `playground/`; `tools/` grew by 47 files and 6,200 lines in four crates (`hooks`, `progress-page`, `run-totals`, `git-sequences`); `scripts/` still holds 53 TypeScript files, F21's to rewrite. Nothing yet |
| Tools | the three rows of Considered for later (`dependencies.md`): no trigger (no npm publication, public API not stable, Stryker's issue open). Nothing yet |
| Fitness functions | cross-page anchors are checked by no test, and the lint's slugs are not the site's (above): a story |
| Patterns | the four rewrites measured tokens three ways (a measured 2.30 characters per token in EN-097 and EN-096, characters / 4 in EN-098, EN-099's own figures): already EN-097's improvement row, a `context:size` mode for files |
| Agent configuration | `AGENTS.md` and `CLAUDE.md` 154 lines together, under 200; rules and skills agree (above); `npm run metrics:session` fails in a worktree with a stack trace, its transcripts' folder derived from the worktree's path: an improvement row |
| Process | the checkpoint ran after the four rewrites, as planned; nothing yet |

## Model trial (EN-066)

The `story-worker` on Opus 5.5 (EN-097, and EN-096 and EN-099 beside it) against Sonnet 5.5 (EN-098), the same kind of story: a family of pages rewritten as principles, measured and traced. Figures from each worker's transcript (`subagents/agent-*.jsonl` of session `48a404b1`), a request counted once from its last line; cost from the prices of `REF-CLAUDE-PROMPT-CACHING` read on 2026-10-17 (per million tokens: Opus 5.5 input $4, 5-minute cache write $5, cache read $0.20, output $20; Sonnet 5.5 half of each); failed calls as each story reports them.

| Story | Model | Pages, tokens before | Requests | Duration | Cache writes | Cache reads | Output | Cost | Failed calls |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| EN-097 | Opus | 78,300 (16 pages) | 70 | 13.3 min | 179,559 | 8,391,861 | 25,853 | $3.09 | 1 |
| EN-096 | Opus | 44,900 (23 pages) | 76 | 16.5 min | 182,002 | 8,842,377 | 13,587 | $2.95 | 2 |
| EN-099 | Opus | 53,711 (9 pages) | 53 | 11.6 min | 151,319 | 5,098,492 | 31,492 | $2.41 | 1 |
| EN-098 | Sonnet | 2,968 (3 pages, characters / 4) | 22 | 5.4 min | 42,493 | 586,326 | 279 recorded | $0.17 recorded | 2 |

- **Not comparable in size**: EN-098's three pages weigh 15 to 26 times less (9 to 15 at 2.30 characters per token) than the Opus stories' pages, so its 13× lower cost says nothing of the model. At the same tokens, the same story on Opus would have cost $0.34: Sonnet's price is half, its request count unknown on a story of EN-097's size.
- **Sonnet's output is under-recorded**: its transcript gives 279 output tokens for about 28,800 characters of text, thinking and tool input (Opus: 25,853 for 108,200, about 4.2 characters per token); at Opus's ratio it wrote about 6,900, $0.07 more. The session report and the status line read the same fields.
- **Quality, read in the stories** (AUD-022 has not run, so no audit finding is counted): EN-098 rewrote whole pages by `cat >` and edited with `python3` scripts, which `project-rules` forbids; it estimated tokens as characters / 4 where EN-097 had measured 2.30 the same day, so its figures are not comparable with the other three; its rule list has 11 coarse rules for two pages, EN-097's about 155 for sixteen. Its pages were already short and tabular, which its own Measures say.
- **Recommendation**: the `story-worker` stays on Opus; the evidence does not settle the question either way, so EN-066's Decisions and the Models section of `docs/tooling/agent.md` are left as they are (the latter already says Opus until a measured trial shows Sonnet does as well).
- **A fair trial needs**: one story worked twice from the same base, once per model, with the same prompt, one branch kept (or two stories of the same kind within 20 % of each other in pages and tokens); the cost from Claude Code's own `modelUsage` (`claude -p --output-format json`, as EN-066's Measures did) rather than the transcript, whose Sonnet output is wrong; failed calls counted from the transcript; and the same audit (`auditor`, `verifier`) on both, its findings counted, plus the rules of `project-rules` each broke.

## First requests after EN-096

EN-096 was merged into F20 at 20:38:51 UTC; one subagent of this session started after it, this checkpoint's own worker (20:39:43 UTC), from a main checkout already at `c5f673a2`. `npm run context:size` on the session's `subagents` folder:

| Agent type | Before EN-096 | Expected | After | Started after the merge |
| --- | --- | --- | --- | --- |
| `story-worker` | 10,694 (median of 7) | about 10,000 | 10,562 | CHK-021, a task of 2,081 characters against a median of 1,347 before; with the before tasks' length, about 10,250 |
| `researcher` | 11,201 | about 10,500 | not measured | none started in this session |
| `clerk` | 6,901 | about 7,200 | not measured | none started in this session |

One story worker is a single sample: about 450 tokens lighter where EN-096 expected 660. The researcher and the clerk wait for the next run's first ones (EN-096's For the main thread).

## For the main thread

- Commits of this story are docs only; `Rule-docs:` none affected, no rule of `docs/domain/` changed.
- Of the eight frictions below, seven are in F21's stories on `docs/f21-refinement` (`0d1a6db9`); the hand-back check's wrong base is in none of EN-103, EN-104 nor EN-108 there: EN-104, the Claude Code hooks, is where it belongs.

### Improvements

| 2026-10-17 | CHK-021 | `story:finish` did not push a done commit rebased after its push, so `gh pr create` failed; folded into EN-103 (push with lease when the heads differ) | tool |
| 2026-10-17 | CHK-021 | The status rows of a feature's Stories table conflicted at every merge, each done commit editing the same table; folded into EN-103 (rows written by `story:finish` after the rebase) | tool |
| 2026-10-17 | CHK-021 | `guard-bash` judged a subagent's Git commands against the session's folder, the main checkout, not the worktree its `cd` entered; folded into EN-104 | tool |
| 2026-10-17 | CHK-021 | `git:sync` on a tree with uncommitted changes exited 1 with no message; folded into EN-103 (it names the files) | tool |
| 2026-10-17 | CHK-021 | `npm run -s git:*` in a branch older than EN-100 failed silently, the script missing from its `package.json`; folded into EN-104 (`guard-bash` names the missing script) | tool |
| 2026-10-17 | CHK-021 | `check-handback` took the wrong base for a branch made from `develop`; not yet in F21's stories, EN-104 (the Claude Code hooks) its place | tool |
| 2026-10-17 | CHK-021 | A test inherited the pre-push hook's `GIT_DIR` and wrote `core.bare=true` and a test user into the shared `.git/config`; folded into EN-103 (a shared Rust test helper, `.git/config` compared by `check:all`) | tooling |
| 2026-10-17 | CHK-021 | The formatter printed "353 of 537 Markdown files changed" when it changed nothing; folded into EN-108 (silent when nothing changes) | tool |
| 2026-10-17 | CHK-021 | The Markdown lint checks a page's anchors with GitHub's slugs and the site uses VitePress's, which differ on a heading with `'` or `/`; five links break on the site and seven on GitHub, and no check reads anchors across pages: one slug for both, checked across pages | tool |
| 2026-10-17 | CHK-021 | `npm run metrics:session` in a worktree fails with a stack trace: it derives the transcripts' folder from the worktree's path; read the main checkout's (`git rev-parse --git-common-dir`) | tool |
| 2026-10-17 | CHK-021 | A Sonnet subagent's transcript records almost no output tokens (279 for about 28,800 characters in EN-098), so per-model figures read from transcripts understate Sonnet; cost a model trial from `modelUsage` | tool |
