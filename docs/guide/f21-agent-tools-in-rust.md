# F21 — Agent tools and the library server in Rust

## What changed for you

Every tool the project wrote for itself — the Claude Code hooks that run before and after each of the agent's calls, the status line, the scripts behind `npm run`, the Git hooks of each commit and push, the library's server, the dependency checks, the secret scan and the release — was a TypeScript program started by Node.js, or a package of npm. F21 rewrote each one in Rust, as a program of the project's own workspace, and removed what it replaced. Each call now answers in a few milliseconds instead of 60 to 280: the agent waits less at every step, a commit takes half the time, `check:all` runs in about 75 s instead of 170 to 217 s at E17's start, and 10 dev packages fewer are installed. A release is now prepared on a branch of its own and published by our own tool, its notes written for the library's users. Each tool records its own uses, and one command reads one row or one section of a long page. Nothing changes in the editor.

## The words

| Word | In plain words |
| --- | --- |
| hook | a small program Claude Code runs before or after each of the agent's calls, to refuse a dangerous command or to record the call |
| Git hook | a program Git runs at each commit or push: here the format, the lint, the commit message's check, the secret scan and `check:all` |
| binary | a compiled program, ready to run; Rust's are in `target/release/` |
| by its path | the binary started directly (`target/release/guard-bash`), not through `cargo run` or `npm run`, which add 47 to 150 ms before its first line |
| dev package | a package of npm installed for the project's tooling, never shipped with the library |
| the library's server | the program that serves the agent the pages it has read, kept in a SQLite file shared by every project of this Mac (ADR-0039) |
| `rmcp` | the official Rust kit for writing such a server (the MCP protocol) |
| release notes | the changes of a version, written for the library's users, in `CHANGELOG.md` and on its GitHub release |
| poll | a call another program makes on a timer, such as the status line's every second |

## The time gained, tool by tool

Medians on this Mac, each tool before its port (SP-023, research note 0025) and after (its story), then measured again by this validation on 2026-10-18 where it could be. A tool's time is one process started and waited for.

| Tool | When it runs | Before (TypeScript or package) | After (Rust, by its path) | This validation |
| --- | --- | --- | --- | --- |
| `guard-bash` | before every shell command | 58.8 ms | 2.9 ms | 3.0 ms |
| `track-tool-call`, `log-tool-call` | before and after every call | 89.3, 106.1 ms | 3.0, 3.0 ms | 3.0 ms (`track-tool-call`) |
| the status line | every second | 180 ms, a quarter of a core (24.5 %) | 10 ms, 1.9 % of a core | 23 ms, 2.3 % of a core, a run open: its totals take 15 ms of it |
| the library's server, its start | once a session | 86 ms | 4 ms | 3.9 ms |
| the library's command line (`search` to `stats`) | when asked | 83 to 104 ms | 5.7 to 11.3 ms | 4.9 ms (`search`) |
| `metrics`, the report of the agent's work | when asked | 9.4 s | 0.46 s | 0.51 s, on more transcripts |
| `session-report`, the current turn | when asked | 10.1 s | 0.10 s | — |
| `replace`, an edit across files | the agent's most used edit tool | 99 ms | 7 ms | — |
| the secret scan, every tracked file | each commit and `check:all` | 970 ms | 106 ms | 112 ms |
| the commit message's check | each commit | 218 ms | 24 ms | — |
| a commit of one Markdown file, every hook run | each commit | 716 ms | 356 ms | — |
| the unused files and packages (`knip`, then ours) | `check:all` | 560 ms | 120 ms | — |
| `check:all`, nothing changed since its last run | before each push | 170 to 217 s at E17's start | 74.6 s (EN-113) | 74.9 s, load 6 to 16 |

One row or one section of a page: `page REF-CC-HOOKS` prints the 2,568 characters of that reference's row, of the 212,137 of `docs/references.md`, in 3 ms. `guard-bash` alone ran 61,388 times since EN-102 began to count, the night of 2026-10-18: at 56 ms saved each, about 57 minutes the agent did not wait.

The library's server is still started through `cargo run` by `.mcp.json`, about 86 ms once a session, so that a session always runs the server of its sources (ADR-0078, decision 4).

## Fewer packages

The dev packages went from 38 at E17's start to 23 at F21's (F19 removed ESLint, Prettier and TypeDoc) and to 13 now. F21 rewrote and removed husky, lint-staged, the three of commitlint, validate-branch-name, secretlint and its preset, npm-check-updates and knip; release-please, a GitHub Action, went too. What is left is each kept with its reason in F21's table: Vite, Vitest and VitePress, TypeScript, oxlint and oxfmt, happy-dom, fast-check, the Node.js types and Mermaid. The library itself still has no dependency. Every package whose code or test cases were ported is named with its licence in `NOTICE` and `THIRD-PARTY-LICENSES.md` (ADR-0076).

## The release, by our own tool

A release used to be two merges by you: release-please's pull request into `develop`, then `develop` into `main`, with a personal token and a third-party action. Now (ADR-0079, removed since: ADR-0101), when you ask for a release, `release prepare` makes a branch `release/x.y.z` from `develop`, sets the version and drafts the entry of `CHANGELOG.md` from the commits; the agent rewrites each line for the library's users, `release check` refuses an entry that breaks a rule of good notes ([releases](../tooling/releases.md), from Keep a Changelog and Common Changelog), and the pull request into `main` holds that commit before it merges. You merge it; `release publish` tags the merge and publishes the GitHub release with the same notes. No commit follows the merge.

The notes of 0.2.0, drafted again by the tool and rewritten by the agent (EN-112), against release-please's:

| Entry | Lines of changes | Lines | Words |
| --- | --- | --- | --- |
| release-please's, in `CHANGELOG.md` | 163: one per commit, 96 of them documentation, the tooling's among the library's | 192 | 1,855 |
| the tool's draft | 23: one per story, the tooling, tests and backlog left out | 34 | 302 |
| the draft rewritten | 14: related stories merged, a change undone within the release dropped | 24 | 236 |

The first lines of each, to compare (release-please's whole section is in the GitHub release of 0.2.0, `CHANGELOG.md` being gone since EN-117, ours in EN-112):

| release-please's 0.2.0 | Ours |
| --- | --- |
| `### ⚠ BREAKING CHANGES`, a block of its own: `**geometry:** contourToPathData is removed from the public API; …` | `### Changed`, first: `**Breaking:** remove contourToPathData from the public API; filletArc, cornerSetbacks and turningAngles take the turning angles or the triples` |
| `### Features`: `**eslint:** folders by business domain, never files beside folders` | `Arrange the library's and the playground's modules in folders by business domain` |
| `F10 round shape and F13 every calculation named, validated`, then `**model:** the round shape, its points on the curve and its two conversions`, then `**playground:** convert a round shape to its points or to a rounded rectangle` | `Add the round shape, a circle or an ellipse drawn with the Round tool, and convert it to a free shape's points` |
| `**playground:** handles in Points mode, typed as x and y`, `… typed as dx and dy from their point`, `… typed as an angle and a length` | `Switch a point to handles in Points mode and type them as x and y, dx and dy, or an angle and a length` |
| 96 lines under `### Documentation` | none: the docs and the backlog are not the library's changes |

The next release, 0.3.0, drafts 57 lines from the 753 commits since 0.2.0, in 0.15 s; it will be the tool's first publication.

## Each tool records its use

Each Rust tool writes one line per use — the tool, its start, its duration, its exit code — into `.git/metrics/tool-uses.jsonl`, through one shared function (EN-102), and `target/release/metrics` lists them per tool in its section Tools: at this validation, 68,596 uses, `guard-bash` 61,388 of them, none failed. The status line's poll, `run-totals status`, called every second during a run, is not recorded: it would add about 18,000 lines a run of 5 hours and say nothing of the tool's use.

## What the audit found

AUD-023 had two independent reviews (auditor, verifier); each finding was reproduced before it was fixed, each new test made to fail once.

| Finding | Outcome |
| --- | --- |
| `guard-bash` matched text: 68 of 77 cases judged wrong (a force push written `-uf` or `"-f"`, a command inside `bash -c`, `eval` or `$(…)`, `--no-veri`, a push to `main` or `develop` let through; a message `-m "fix -n flag"` refused) | **rewritten**: each command read word by word as the shell reads it, wrappers looked through, Git's options read as Git reads them, push destinations checked; 0 of 77 wrong |
| the settings allowed `Bash(target/release/*)`: `target/release/heavy-lock git push -f` ran without asking | **narrowed**: each safe binary allowed by name, three by subcommand; `heavy-lock`, `branches-clean`, `git-hooks` and `release prepare` ask first |
| a hook whose binary is missing let its call through | the guard now blocks when it cannot run (`onFailure: block`); a person runs `npm run rust:build` once |
| the `.git/config` check missed `[url]`, `[include]`, `[includeIf]`, `[extensions]` | guarded; F21.AC9 says so |
| `curl` read `~/.curlrc`; the library's had no time limit | `--disable` first, 60 s at most |
| criteria without a tagged test, two functions and three Git helpers written twice, boundaries and guards that a mutation survived | each tagged, merged into one, or now caught by a test |

Left for E17's naming and tooling pass (NAM-002): 25 places starting Git each their own way, and a JSON reader of the hooks beside the workspace's `serde_json`.

## Decisions for you to confirm

| Decision | Why | Where |
| --- | --- | --- |
| The library's file is named by `AGENT_LIBRARY_FILE` when set, never `LIBRARY_PATH` | `LIBRARY_PATH` is the C toolchain's search path: Apple's `python3` sets it, and a library script started from it opened a store there | ADR-0077 |
| The hooks, the status line, the Git hooks and the agent call each Rust tool by its binary's path, `npm run` kept for you | 3 ms a call instead of 47 to 150 ms; a binary older than its sources is rebuilt after each sync and merge, and `npm run rust:fresh` names it | ADR-0078, ADR-0080 for `.githooks/` |
| A release is prepared on a branch `release/x.y.z` from `develop`, not by a commit on `develop` | `develop` moves often; the pull request into `main` keeps its checks and its version while `develop` goes on | ADR-0079 (removed since: ADR-0101) |
| After the worktree's `cd`, `guard-bash` lets one Git command through piped into read-only filters (`head`, `grep`, `wc`, `sed -n`…); a second command is still refused | a refusal costs the agent a call; a filter only reads | EN-104 |
| `run-totals status` is not recorded | the status line calls it every second during a run: about 18,000 lines a run, no information | EN-102, `docs/tooling/metrics.md` |
| The story worker stays on Opus | F20's trial on Sonnet was too small to compare; a fair trial works one story twice from the same base | EN-066, CHK-021 |

## Each criterion and its evidence

Proven again on 2026-10-18 by VAL-022: its story holds the command and what was seen for each. The workspace's 1,554 Rust tests passed, and the 16 TypeScript tests tagged `[F21.ACn]`.

| Criterion | What you get | Evidence |
| --- | --- | --- |
| AC1 every hook a Rust binary | `.claude/hooks/` gone; each hook and the status line run `target/release/<binary>`, 3 ms a hook | `tools/hooks/tests/`, `tools/stale-binaries/tests/`, 8 tests `f21_ac1_` |
| AC2 every script a Rust binary | `scripts/` holds `default-font-metrics.ts` only, which runs the library itself | `tools/repo-tools/tests/`, `tools/git-sequences/tests/`, `tools/metrics/tests/`, 11 tests |
| AC3 the library's server on `rmcp` 3.5.1 | the same tools, answers and store, started in 4 ms | `tools/library/tests/`, 3 tests |
| AC4 each dev package kept with its reason or rewritten | 13 packages; the Git hooks, the dependency checks and the secret scan ours; the CI on the cached build | `tools/git-hooks`, `tools/deps`, `tools/secret-scan`, `tools/format`, 49 tests; `tests/ci-caches.test.ts` |
| AC5 nothing of the removed tools left | no file of a rewritten package; their names only where kept with a reason | `tests/package.test.ts`, EN-113's search |
| AC6 the release by our own tool | a release branch, notes written for users, tag and release from `main`, no commit after the merge | `tools/release/tests/`, 15 tests; `tests/release-flow.test.ts` |
| AC7 each tool records its use | `target/release/metrics`, section Tools | `tools/tool-use/tests/`, `tools/metrics/tests/`, `tools/run-totals/tests/`, 13 tests |
| AC8 one row or one section | `target/release/page <id or heading>` | `tools/page/tests/`, 11 tests |
| AC9 `.git/config` guarded, Git in tests without `GIT_*` | `check:all` names a line a lane changed | `tools/repo-tools/tests/`, `tools/test-support/`, 7 tests |

## How to try it

Run `npm run dev` and open `http://localhost:5173/?checklist=E17`: E17's checklist, F21's 12 lines after F20's, each a command to run in a terminal at the repository's root, or something to ask the agent, and five with a decision for you to confirm.

| Try | You should see |
| --- | --- |
| `ls .claude/hooks`, then `grep -o '"command": "[^"]*' .claude/settings.json` | no such folder; each hook running `target/release/<binary>` |
| ask the agent, in a story's worktree, `git log --oneline -3 \| head -3`, then `git status; ls`, each after its `cd` | the first goes through, the second is refused with the form to run |
| `target/release/metrics`, section Tools | each tool's calls, failures and times; no `run-totals status` |
| `ls scripts` | `default-font-metrics.ts` and its test only |
| `target/release/library search fts5` | the stored sections holding the word, in a few milliseconds |
| `npm run deps:unused` | nothing, exit 0 |
| `printf 'Fixed the thing\n' > /tmp/msg`, then `target/release/git-hooks commit-msg /tmp/msg` | `subject may not be empty`, `type may not be empty`, exit 1 |
| `time npm run -s lint:secrets` | nothing found, in about 0.3 s |
| in a fresh clone of `develop` once F21 is merged: `npm install`, `target/release/release prepare` | `v0.2.0 → 0.3.0 on release/0.3.0: 57 line(s) drafted in CHANGELOG.md` |
| `target/release/page REF-CC-HOOKS` | that reference's row and its table's header, nothing else |
| `npm run check:all` | every check ✓ in about 75 s with nothing changed |

## Limits

- The library's server is still started through `cargo run` (about 86 ms, once a session), so that each session runs the server of its sources.
- A tool's recorded duration is its own work, not its process's start (about 3 ms), so `target/release/metrics` reads a little lower than a timer around the process.
- The release tool has not published yet: 0.3.0 will be its first run on `main`, with the CI's caches saved there for the first time (EN-111).
- Starting Git is still written in 25 places, and the hooks keep their own JSON reader: E17's naming and tooling pass (NAM-002) is next.
- `f21_ac2_reports_the_main_checkouts_work_from_a_worktree_its_findings_included` fails when the tests are run with `--release`, its binary recording its own use; `check:all` runs them in a debug build and passes (an improvement row).

## Validation

Validated by the agent on 2026-10-18 (`VAL-022`), F21 being a tooling feature (ADR-0072): each of its nine criteria proven again by its tests and a command, and F21's 12 checklist lines tried. You may try the lines that concern you, and confirm the six decisions above.

| Remark | What was done or where it went |
| --- | --- |
