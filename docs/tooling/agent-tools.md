# The agent's tools

**What the agent writes by hand again and again becomes a tool**, tested, linted and named after what it does (ADR-0050). The model writing a command costs more than the command running: at E14, 1,147 hand-written Python edit scripts took 389 minutes, most of it writing them. A tool also saves round trips: each call avoided saves about 3 s and a context read again ([research note 0024](../research/notes/0024-f20-context-diet.md)). So the agent uses these tools rather than ad-hoc scripts, and a sequence it repeats is a candidate for the next one.

## Tools

| Command | Does | Replaces |
| --- | --- | --- |
| `npm run replace` | reads blocks `@@ file`, `<<<`, old text, `===`, new text, `>>>` on standard input, and writes nothing unless each old text is found exactly once | a Python script of `replace` calls |
| `npm run backlog:status -- <ID> <status>` | changes an item's status in its file, its folder's index and its parent's table, the three places `backlog.test.ts` checks; a story's index is written again rather than edited | three hand edits |
| `npm run backlog:index` | writes `docs/backlog/stories/README.md` from each story's front matter, in the order of the epics' and features' tables (`tools/backlog-index/`, ADR-0065) | the index edited by hand on every branch |
| `npm run worktree:new -- <ID> [--type <type>]` | adds `.claude/worktrees/<id>` on `<type>/<id>-<topic>` from the story's feature branch on GitHub (`origin/develop` for an epic's story), the type from its kind (`US`, `VAL` `feat`; `EN` `chore`; `NAM` `refactor`; others `docs`), links `node_modules`, runs Husky so that a push runs the branch's own hooks ([Git workflow](./git-workflow.md#hooks-husky)); refuses an existing folder and a branch name that is not a story's (`tools/worktree/`) | worktrees named `agent-a13cbef…` |
| `npm run git:sync [-- --base <ref>]` | in a story's worktree, fetches `origin` and rebases the story on its feature branch, found as `worktree:new` finds it, or on the base given; on another branch, fast-forwards it to its upstream, never rebasing a shared branch; refuses `main` and a detached head; stops at the first failing step with Git's message; prints the branch, its base and the commits ahead (`tools/git-sequences/`) | `git fetch`, then `git rebase`: two calls |
| `npm run git:commit -- -F <file\|-> [--push\|--push-with-lease] [--] <paths>` | stages the paths named (none: what is staged), commits with the message of the file or of the standard input, pushes with `-u origin HEAD` when asked, `--push-with-lease` on a story branch only, never a push of `develop`; refuses `main`, a detached head and an empty message before staging; the hooks always run; shows a step's output only when it fails, then stops; prints the hash and subject (`tools/git-sequences/`) | `git add`, `git commit`, `git push`: three calls |
| `npm run story:finish -- <base> <title> <body>` | in the story's worktree, under the [merge lock](./commands.md#one-heavy-command-at-a-time): rebases the story if its base moved — a conflict only in the status rows another story's done commit wrote beside its own (the stories index, a feature's or an epic's table rows) is left to the base, then the story's status is written again from its front matter and folded into its done commit, so that two stories of one feature finish one after the other without a conflict; any other conflict stops it, the branch as it was —, pushes the branch with lease when its head differs from GitHub's (a rebase, or a done commit made again by hand), opens its pull request labelled `autonomous`, waits up to three minutes for GitHub's merge, brings the base up to date where it is checked out, moves the story's rows into the logs ([below](#shared-files-and-their-rows)), commits and pushes them, then removes the worktree and the branch here and on GitHub; not merged, it keeps them. Run again after a stop, it resumes where it stopped, and prints the commands left when a clean-up fails | a waiting loop and four cleaning commands |
| `npm run run:start`, `run:end` | opens and closes the run's marker, and prints its duration and tokens by kind ([metrics](./metrics.md#a-runs-totals), `tools/run-totals/`) | a turn's figures, restarting at each message |
| `npm run run:should-stop` | before each story: `stop` and exit 1 once the week's usage reaches 90 %, else `go` and exit 0 (unknown usage: `go`), then the parked stories of the checkout it runs in, so run it in a worktree of the feature branch; exit 2 outside a repository | the usage read by eye |
| `npm run context:size -- <folder>…` | for each subagent of a session's folder: its type, its first request's tokens, the instruction files loaded before it, the size of its skill listing and task; then the median per type (`tools/hooks/`) | first requests counted by hand |
| `npm run -s page -- [<file>] <id or heading>` | beside `Read`, for a row or a section: the rows whose first cell is the id, under their table's header line, else each section under that heading down to the next of its level or higher, after `<file>:<line>`; without a file, a `REF-` id's row of `docs/references.md`, or the first section of an ADR's, a derivation's or a backlog item's file (`ADR-0069`, `DERIV-fillet-arc`, `EN-092`, `F21`, `E17`); nothing found exits 1 naming the file. 3.2 ms against `cat`'s 2.9 ms, the binary alone, and about 165 ms through `npm` ([below](#reading-one-row-or-one-section)) (`tools/page/`) | `Read` of a whole page for one row: `docs/references.md` 121 times in three days |
| `npm run golden:write` / `golden:check` | about 400 results of the library from shapes drawn with a fixed seed, written before a refactor and compared bit for bit after each change (`NAM`, ADR-0049); `golden:write` refuses in a worktree, the file being shared; the pre-push runs `golden:check` when the file is written | a probe written for F13 |
| `npm run activity` | the calls running, the latest calls of the main agent and each subagent, the end of the background commands' output | reading transcripts |
| `npm run epic:check -- <epic> <first day>` | an epic's full check for its retrospective: failed `check:all` runs with their failing checks and tests, branches, pull requests, issues, failed calls (the worktree refusals apart), calls lost at a force-quit, backlog, records | a dozen commands |
| `npm run val:prepare -- <feature>` | at a `VAL`, one checklist line per numbered criterion of the feature, to rewrite as what to try and what to see; registers the epic's checklist and prints its address; refuses a feature without criteria or whose lines exist, warns above 12 (`tools/val-prepare/`) | guided steps written for each validation |
| `npm run mutate -- [--all] <files> [-- <command>]` | under the heavy lock, in a temporary copy of the repository, tries mutations on each file and reports each caught, timed out or `SURVIVED` with its line; exits 1 when one survives ([below](#mutate), `tools/mutate/`) | a runner written again at each audit |
| heavy lock (`tools/heavy-lock/`) | one heavy command at a time across every worktree ([commands](./commands.md#one-heavy-command-at-a-time)) | a lock taken by hand with `mkdir` |

### mutate

| Step | What it does |
| --- | --- |
| copy | the repository as a temporary worktree outside it, tracked files with their uncommitted changes; removed at the end |
| tests | each file's related tests (`npx vitest related {file} --run`), which through the barrels are most of the suite; `-- npx vitest run {test}` runs only the file's own test file |
| unchanged | the tests run once first; a file whose tests fail, or that has none, is skipped |
| mutations | the first of each kind, every one with `--all`: a comparison flipped (`<` to `>=`, `===` to `!==`), `+` and `-` or `*` and `/` swapped, an integer `n` made `n + 1`, an `if` condition negated, a returned value made `undefined` |
| where | found in the text, comments and strings skipped, operators only with a space on each side |
| timeout | over four times the unchanged run plus 30 s |

### Reading one row or one section

`page` (EN-105) answers the agent's most frequent reads: `docs/references.md` was read 121 times in three days for one row each ([research note 0024](../research/notes/0024-f20-context-diet.md)). Measured on 2026-10-18, 20 calls each after one warm-up, interleaved, the release binary run by its path (load average about 3):

| Read | Characters printed | Median | Mean |
| --- | --- | --- | --- |
| `cat docs/references.md` | 198,307 | 2.9 ms | 3.0 ms |
| `page REF-CC-HOOKS`: its row and its table's header | 2,335 (1.2 %) | 3.2 ms | 3.2 ms |
| `cat docs/tooling/agent.md` | 14,495 | 2.8 ms | 2.9 ms |
| `page docs/tooling/agent.md Hooks` | 3,741 (26 %) | 3.1 ms | 3.1 ms |

Through `npm run -s page`, npm adds about 160 ms (median 167 ms): still one call, and the characters are what the context keeps. `docs/process/improvements.md` is one table whose rows start with a date (SP-023): its rows are not found by an id.

## Specifications for later

A tool is written when a retrospective's metrics show its signal — the hand work it would replace, measured — and not before: a tool nobody needed is code to keep. Each is written in Rust ([below](#rust)).

| Tool | Does | Signal |
| --- | --- | --- |
| `story:start <ID>` | branch from the up-to-date feature branch, step file written, status `in-progress` in the three places | the same five commands at each story's start |
| `story:new <type> <feature>` | the next free id, the story file from its template, its rows in the index and the feature's table | ids and rows corrected by `backlog.test.ts` |
| `story:show <ID>` | the story's criteria, tasks, feature plan row and linked files, in one short text | story, feature and index files read again and again |
| `backlog:next` | the next story of the feature in progress, or the next feature in the Product Owner's order | the session-state hook not enough at a run's start |
| `adr:new <title>` | the next number, the file from its template, its row in the index | ADR numbers or index rows fixed after a review |
| `journal:add <tags> <text>`, `log:feedback`, `log:idea` | a dated entry or row with its tags or columns in order | entries and rows with a missing column |
| `commit:task <ID.Tn> <subject>` | the commit with its scope checked, its `Refs` and attribution lines; on a refused commit, the errors only | refused commits (scope, sentence case, footer) |
| `pr:body` | the pull request's body from the branch's commits, stories and evidence | bodies written by hand |
| `code:outline <path>` | a file's or folder's exports, kind, `@see` and first JSDoc line, without the bodies | `cat` and `sed -n` on whole files |
| `code:find <name>` | where a function or type is defined, its tests, and every file that uses it | chains of `grep`, `ls` and `cat` |
| `refs:show <REF>` | one `REF-*` or `DERIV-*` row, whether it is verified, and the functions that cite it | the bibliography read whole to check one row |
| `check:changed` | fix, lint and the tests related to the files changed since the base, errors only | lint and tests run on chosen files by hand |
| `api:diff` | the public exports before and after a change | a renamed export noticed late |
| `svg:draw <shape.json>` | the SVG of a shape with its dashed box, for the guide's pictures | pictures written by hand from a probe |
| `playground:probe <steps>` | runs guided steps without a browser and prints what each shows | probes written in the scratchpad |
| the status line, `guard-bash`, `track-tool-call`, `log-tool-call` in Rust | the same lines, checks and records, each in a few milliseconds instead of a Node.js start (F21) | 65 ms before each shell command, 149 ms per refresh (2026-10-14) |
| `transcript:find <text>` | the transcript lines holding an id or a text, with their time and type | a stuck task found by `grep` in a 30 MB transcript |
| a summary of OpenType and CSS Fonts | the fields the project uses, read once | OS/2 read 38 times, CSS Fonts 4 27 times in E15 |

## Rust

**Every new script, hook or agent tool is written in Rust** (ADR-0073; Product Owner, 2026-10-16): the agent runs its tools hundreds of times a session, a Rust tool starts without Node.js and reads Oxc's crates directly, and a tool written in TypeScript now would be rewritten later. The TypeScript tools may still be edited until F21 rewrites them; the rule covers the tools around the library, never `src/` nor the playground. A tool is a crate of `tools/`, a member of the root workspace, run by `cargo run --release -q -p <crate>`; the library itself stays dependency-free.

| Crate | Run by | Does |
| --- | --- | --- |
| `backlog-index` | `backlog:index`, `backlog:status` | the stories index from each story's front matter; `--check` names the first row that differs |
| `format` | `format`, `format:check`, `fix`, `lint-staged`, `backlog:index`, `story:finish` | the one formatter: oxfmt, then Markdown tables without padding (ADR-0068, [formatting](../conventions/formatting.md)); `--check` changes nothing |
| `heavy-lock` | `check:all`, `test`, `golden:check`, `docs:build`, `mutate`; `story:finish` (`--lock merge`) | one heavy command at a time across worktrees, and the merge lock |
| `hooks` | the `limit-agents`, `library-pages` and `check-handback` hooks, `context:size` | the Claude Code hooks in Rust and what they share ([agent](./agent.md#hooks)) |
| `shared-files` | the `pre-push` hook; `story:finish` | `refuse-shared-files` and `move-rows` ([below](#shared-files-and-their-rows)) |
| `worktree` | `worktree:new` | a story's worktree with `node_modules`, Husky and the release binaries |
| `git-sequences` | `git:sync`, `git:commit` | the frequent Git sequences in one call each |
| `run-totals` | `run:start`, `run:end`, `run:should-stop`, `metrics:session -- --run`, the status line | a run's marker, duration and tokens, counted incrementally; whether it stops at the week's limit |
| `val-prepare` | `val:prepare` | a feature's checklist lines |
| `page` | `page` | one row or one section of a repository page, by its id or its heading ([above](#tools)) |
| `mutate` | `mutate` | the mutations in a copy of the repository |
| `lint` | `lint`, `fix`, `lint-staged`, after oxlint | the project's linter on Oxc: the project's rules, those oxlint lacks and the Markdown rules, each file parsed once ([lint](../conventions/lint.md)) |
| `rule-docs` | the `commit-msg` hook, at a `done` commit | asks a story that changes `docs/domain/` for its Q row, its VAL's decisions to confirm and its guide page, unless the message says `Rule-docs: none affected, <why>` |
| `api-pages` | `docs:api`, `docs:dev`, `docs:build` | the API pages from `src/index.ts` and its TSDoc ([documentation site](./documentation-site.md)) |
| `declarations` | `build` | the library's `.d.ts` from each module alone (ADR-0071, [build](./commands.md#build)) |
| `library` | `.mcp.json` (`library-mcp`), `library`, `metrics`, `records:save` | the local source library: its MCP server on `rmcp` and its command line, over one SQLite file ([sources](./sources.md)) |
| `tool-use` | every binary of the workspace, by its `main` | each use of a tool, one line of `.git/metrics/tool-uses.jsonl` ([metrics](./metrics.md#the-uses-of-each-tool)) |
| `repo-tools` | `check:all`, `backlog:status`, `epic:check`, `story:finish` | the backlog and repository tools ported from `scripts/` with their output unchanged (EN-103), one binary each: `check-all` ([the lanes](./commands.md#the-lanes-of-the-full-check)), `backlog-status`, `epic-check`, `story-finish` |
| `test-support` | the Rust tests | `test_support::command`, a command without the inherited `GIT_*` variables, for every test that runs `git` ([tests that run Git](./commands.md#tests-that-run-git)) |

`worktree` and `git-sequences` decide what they do in their library, without Git nor files, and their binary only runs it, so that the decisions are tested without a repository (functional core, ADR-0014).

- **The workspace**: one virtual workspace at the root (`Cargo.toml`, `members = ["tools/*"]`), whose edition, licence, `publish = false` and lints (`clippy::pedantic`, `unsafe_code` forbidden) each crate inherits by `[lints] workspace = true`; the toolchain pinned by `rust-toolchain.toml`, with rustfmt, clippy, llvm-tools and the `x86_64-pc-windows-msvc` target; warnings denied (`.cargo/config.toml`); one `Cargo.lock` tracked and one `target/` ignored.
- **Each use recorded** (EN-102): every binary's `main` hands its work to `tool_use::run` under its own name, so that `npm run metrics` lists its calls, failures and times ([metrics](./metrics.md#the-uses-of-each-tool)); `tools/tool-use/tests/binaries.rs` fails on a binary that does not.
- **Dependencies**: none from crates.io but the reviewed ones of `[workspace.dependencies]`, each pinned exactly with its row under [Crates](./dependencies.md#crates).
- **Checks**: `npm run check:rust` — `cargo fmt`, Clippy, the advisories, the tests under the coverage floor, and a check for Windows — in `check` and the `rust` lane of `check:all`.
- **Portable as a goal** (2026-10-17): a crate uses the standard call where one exists (`File::try_lock`, `fs::hard_link`); a Unix-only call — a file's inode, a process group, a folder symbolic link, an execute bit — sits behind `cfg(unix)` with its reason in a comment, and the Windows check fails on one planted outside it. Nothing runs on Windows in a CI.
- **Coverage**: `npm run rust:coverage` fails under 93 % of lines or of functions of the whole workspace, the measure of 2026-10-17 rounded down. The floor is the workspace's, not each crate's: the table printed on failure shows which crate lacks tests. It is raised when a story raises the measure, never lowered.
- **Advisories**: `npm run rust:advisories` checks `Cargo.lock` against RustSec, unmaintained and yanked crates failing too, no advisory ignored (`deny.toml`); `tests/rust-advisories.test.ts` plants a vulnerable crate and sees the check fail.
- **Tests**: beside their crate's code; every test removes every inherited `GIT_*` variable from each command it starts, Git itself or the crate's binary that runs it ([tests that run Git](./commands.md#tests-that-run-git)).
- **A fresh machine** installs rustup (`https://rustup.rs`), which installs the pinned toolchain at the first `cargo` call, then `cargo install --locked cargo-llvm-cov@0.9.1 cargo-deny@0.20.2` and `cargo deny fetch db`. Without them `rust:coverage` and `rust:advisories` fail and name the command to run: a missing tool never passes a check.

## Shared files and their rows

The shared files — the backlog's three indexes, the three logs of `docs/process/`, the feature files, the checklists `playground/checklists/epic-*` — are written by the main thread only, at merge (ADR-0065, decision 4): parallel workers editing them would conflict at every merge.

- `refuse-shared-files`, first in the `pre-push` hook, refuses a worker's story branch (`US`, `EN`) whose commits not yet on the remote change one, naming each file and its commit. It lets through the story's `done` commit (`Closes: <ID>`) setting its status, and a change that only pads or unpads a shared file's Markdown tables, so that the formatter can rewrite them; a changed cell, row or other line is still refused. The other branches — `develop`, the feature branches, the stories the main thread writes (`SP`, `CHK`, `AUD`, `VAL`, `NAM`, `REV`, `RET`) — pass.
- Its limits are accepted (AUD-020): it runs on the pushing machine only, so a commit pushed first under another branch passes, and GitHub checks nothing. It keeps workers from editing shared files by mistake, not on purpose.
- A worker writes the rows meant for a log in its story file, under `## For the main thread`, beneath `### Journal`, `### Improvements` or `### Feedback`. `move-rows`, run by `story:finish` once the story is merged, moves them into the logs and drops those headings; `story:finish` then formats, commits and pushes them on the base.

| Log | Rows moved | Dropped | Left in the story file, named on standard error |
| --- | --- | --- | --- |
| `journal.md` | a bullet opening with its tag (``- `#run` …``), at the end of the newest section, or under a new section `## <today> — Run continued` when that one is older | a placeholder (`- None.`) | any other line |
| `improvements.md`, `feedback.md` | a table row whose first cell is a date, after the table's last row | a placeholder, a header and separator | any other line |

`tests/logs.test.ts` fails on a line that breaks a log's table or an untagged journal bullet.

`rule-docs` gathers the files of the story's commits — those whose `Refs:` or `Closes:` names the story or one of its tasks — and the files staged; a file changed counts, not the rows inside it. It runs at commit, not at push, so that a refused `done` commit is made again rather than amended after a push.
