# The agent's tools

**What the agent writes by hand again and again becomes a tool**, tested, linted and named after what it does (ADR-0050). The model writing a command costs more than the command running: at E14, 1,147 hand-written Python edit scripts took 389 minutes, most of it writing them. A tool also saves round trips: each call avoided saves about 3 s and a context read again ([research note 0024](../research/notes/0024-f20-context-diet.md)). So the agent uses these tools rather than ad-hoc scripts, and a sequence it repeats is a candidate for the next one.

## Tools

| Command | Does | Replaces |
| --- | --- | --- |
| `target/release/replace` | reads blocks `@@ file`, `<<<`, old text, `===`, new text, `>>>` on standard input, and writes nothing unless each old text is found exactly once; a line outside a block but blank lines, or a text holding a line `<<<` or `>>>` (a block that lost its `@@` line or its `>>>`), refuses the whole input, naming the block and its line (EN-103) | a Python script of `replace` calls |
| `target/release/backlog-status <ID> <status>` | changes an item's status in its file, its folder's index and its parent's table, the three places `backlog.test.ts` checks; a story's index is written again rather than edited | three hand edits |
| `npm run backlog:index` | writes `docs/backlog/stories/README.md` from each story's front matter, in the order of the epics' and features' tables (`tools/backlog-index/`, ADR-0065) | the index edited by hand on every branch |
| `target/release/worktree-new <ID> [--type <type>]` | adds `.claude/worktrees/<id>` on `<type>/<id>-<topic>` from the story's feature branch on GitHub (`origin/develop` for a story attached to `develop`, a `REV` or an epic's `SP`); a story whose file the checkout lacks, the main checkout on `develop`, is looked for on `origin`'s feature branches (`git ls-tree`) and started from the one holding it, two holding it refused by name (EN-118); the type from its kind (`US`, `VAL` `feat`; `EN` `chore`; `NAM` `refactor`; others `docs`), links `node_modules`, sets the hooks folder `.githooks` if it is not, so that a push runs the branch's own hooks ([Git workflow](./git-workflow.md#hooks-githooks)), and builds the release binaries from a copy of the main checkout's `target/release` (15.7 s against 34.3 s from nothing, EN-113); refuses an existing folder and a branch name that is not a story's (`tools/worktree/`) | worktrees named `agent-a13cbef…` |
| `target/release/git-sequences sync [--base <ref>]` | in a story's worktree, fetches `origin` and rebases the story on its feature branch, found as `worktree-new` finds it, or on the base given; on another branch, fast-forwards it to its upstream, never rebasing a shared branch; refuses `main` and a detached head, and a rebase of a tree with uncommitted changes to tracked files, naming each file and its change (EN-103); stops at the first failing step with Git's message; prints the branch, its base and the commits ahead; then rebuilds the checkout's release binaries, a failed build stopping it with Cargo's message (ADR-0078) (`tools/git-sequences/`) | `git fetch`, then `git rebase`: two calls |
| `target/release/git-sequences commit -F <file\|-> [--push\|--push-with-lease] [--] <paths>` | stages the paths named (none: what is staged), commits with the message of the file or of the standard input, pushes with `-u origin HEAD` when asked, `--push-with-lease` on a story branch only, never a push of `develop`; refuses `main`, a detached head and an empty message before staging; the hooks always run; shows a step's output only when it fails, then stops; prints the hash and subject (`tools/git-sequences/`) | `git add`, `git commit`, `git push`: three calls |
| `target/release/git-sequences state` | only reads, without a fetch: a line naming the branch, its base (a story's feature branch as `sync` finds it, `origin/develop` for an epic's own story, another branch's upstream; `no base found` otherwise) and the commits ahead, then `status --short`, `diff --stat HEAD` (tracked changes, staged or not) and the commits ahead as `<hash> <subject>`, each under its command, `clean` or `none` when empty (EN-119, `tools/git-sequences/`) | `git status`, `git diff --stat`, `git log`: the reads chained most, about half of 176 refusals in E17 (I93) |
| `npm run story:finish -- <base> <title> <body>` | in the story's worktree, under the [merge lock](./commands.md#one-heavy-command-at-a-time): stops first when the story file still holds rows for the logs, which its `done` commit moves ([below](#shared-files-and-their-rows)); rebases the story if its base moved — a conflict only in the status rows another story's done commit wrote beside its own (the stories index, a feature's or an epic's table rows) is left to the base, then the story's status is written again from its front matter and folded into its done commit, so that two stories of one feature finish one after the other without a conflict; any other conflict stops it, the branch as it was —, pushes the branch with lease when its head differs from GitHub's (a rebase, or a done commit made again by hand), opens its pull request labelled `autonomous`, waits up to three minutes for GitHub's merge — except an epic's `SP` into `develop`, which the auto-merge does not take: its pull request opened without the label, it says at once that it waits for the Product Owner's merge and stops, and run again once merged it cleans up (EN-118) —, brings the base up to date where it is checked out and rebuilds its release binaries there (ADR-0078), commits nothing on it, then removes the worktree and the branch here and on GitHub; not merged, it keeps them. Run again after a stop, it resumes where it stopped, and prints the commands left when a clean-up fails | a waiting loop and four cleaning commands |
| `target/release/run-totals start`, `end` | opens and closes the run's marker, and prints its duration and tokens by kind ([metrics](./metrics.md#a-runs-totals), `tools/run-totals/`) | a turn's figures, restarting at each message |
| `target/release/run-totals should-stop` | before each story: `stop` and exit 1 once the week's usage reaches 90 %, else `go` and exit 0 (unknown usage: `go`), then the parked stories of the checkout it runs in, so run it in a worktree of the feature branch; exit 2 outside a repository | the usage read by eye |
| `target/release/context-size <folder>…` | for each subagent of a session's folder: its type, its first request's tokens, the instruction files loaded before it, the size of its skill listing and task; then the median per type (`tools/hooks/`) | first requests counted by hand |
| `target/release/page [<file>] <id or heading>` | beside `Read`, for a row or a section: the rows whose first cell is the id, under their table's header line, else each section under that heading down to the next of its level or higher, after `<file>:<line>`; without a file, a `REF-` id's row of `docs/references.md`, or the first section of an ADR's, a derivation's or a backlog item's file (`ADR-0069`, `DERIV-fillet-arc`, `EN-092`, `F21`, `E17`); nothing found exits 1 naming the file. 3.2 ms against `cat`'s 2.9 ms, the binary alone, and about 165 ms through `npm`, its script until EN-115 ([below](#reading-one-row-or-one-section)) (`tools/page/`) | `Read` of a whole page for one row: `docs/references.md` 121 times in three days |
| `npx vitest run tests/golden-results.test.ts --mode golden-write` / `npm run golden:check` | about 400 results of the library from shapes drawn with a fixed seed, written before a refactor and compared bit for bit after each change (`NAM`, ADR-0049); `guard-bash` refuses the writing in a worktree, the file being shared; the pre-push runs `golden:check` when the file is written | a probe written for F13 |
| `target/release/activity` | the calls running, the latest calls of the main agent and each subagent, the end of the background commands' output | reading transcripts |
| `target/release/epic-check <epic> <first day>` | an epic's full check, at a review, from the day of the last merge into `main`: failed `check:all` runs with their failing checks and tests, branches, pull requests, issues, failed calls (the worktree refusals apart), calls lost at a force-quit, backlog, records | a dozen commands |
| `target/release/val-prepare . <feature>` | at a `VAL`, one checklist line per numbered criterion of the feature, to rewrite as what to try and what to see; registers the epic's checklist and prints its address; refuses a feature without criteria or whose lines exist, warns above 12 (`tools/val-prepare/`) | guided steps written for each validation |
| `target/release/release notes [<base>]`, `target/release/release check <file>` | at a review, before `develop` → `main`: `notes` prints the draft of the features' text from the commits `origin/main` lacks, each line marked `[draft]`, then on its error output the commits behind each line, writing no file and making no branch; `check` checks the body's file, up to its first line `---`, against the rules of good release notes; `release publish` is the button's, in `release.yml` ([releases](./releases.md), `tools/release/`) | release-please's release pull request |
| `npm run mutate -- [--all] <files> [-- <command>]` | under the heavy lock, in a temporary copy of the repository, tries mutations on each file and reports each caught, timed out or `SURVIVED` with its line; exits 1 when one survives ([below](#mutate), `tools/mutate/`) | a runner written again at each audit |
| heavy lock (`tools/heavy-lock/`) | one heavy command at a time across every worktree ([commands](./commands.md#one-heavy-command-at-a-time)) | a lock taken by hand with `mkdir` |
| `target/release/stale-binaries` (`npm run rust:fresh` in `check:rust`) | each release binary older than a file Cargo built it from (its dep-info, `target/release/<binary>.d`) or missing, then exit 1; the last step of `check:rust`, and named by the `session-state` hook (`tools/stale-binaries/`, ADR-0078) | a hook running the code of a build before its sources changed |

### The binaries the agent calls

The agent calls a tool by its release binary, from the root of its checkout, rather than through `npm run` and `cargo run`, which cost about 150 ms before the tool's first line (ADR-0078, research note 0025): a Bash call of a worker is `cd "<worktree>" || exit 9` then the binary. These tools have no npm script (EN-115, ADR-0082): `package.json` keeps the scripts a person runs (`dev`, `build`, `preview`, `docs:dev`, `fix`, `test`, `check:all`) and those something calls by their name, among them the agent's `story:finish` and `mutate`, run under a lock, `golden:check`, run by the pre-push hook, and `backlog:index`, run by `backlog-status` and `story-finish` ([commands](./commands.md)). A binary is rebuilt at `prepare`, after `git-sequences sync`, after `story:finish` and after a pull that changed a Rust input (the `post-merge` and `post-rewrite` hooks, EN-131); `target/release/stale-binaries` names a stale one. `.claude/settings.json` allows each binary of the list below by its name, and by its subcommand for `git-sequences`, `dependencies` and `release` (`check`, `notes`), never `target/release/*`: `heavy-lock` runs any command given after it, past every permission rule, `branches-clean` deletes branches, `git-hooks install` sets the hooks folder and `release publish` makes a release, so each of them asks (AUD-023, `tools/hooks/tests/settings.rs`).

- `target/release/replace`
- `target/release/backlog-status <ID> <status>`
- `target/release/worktree-new <ID>`
- `target/release/git-sequences sync`, `target/release/git-sequences commit -F <file> [--push] <paths>`
- `target/release/run-totals start`, `end`, `should-stop`
- `target/release/context-size <folder>…`
- `target/release/page [<file>] <id or heading>`
- `target/release/epic-check <epic> <first day>`
- `target/release/val-prepare . <feature>`
- `target/release/release notes [<base>]`, then `release check <file>`
- `target/release/library …`
- `target/release/metrics`, `target/release/session-report …`, `target/release/activity`
- `target/release/branches-clean`, `target/release/records-save`
- `target/release/stale-binaries`
- `target/release/dependencies outdated`, `target/release/dependencies unused`

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

Through `npm run -s page`, its script until EN-115, npm added about 160 ms (median 167 ms): still one call, and the characters are what the context keeps. `docs/process/improvements.md` is one table whose rows start with a date (SP-023): its rows are not found by an id.

### The backlog and repository tools in Rust

EN-103 ported seven scripts to the binaries of `tools/repo-tools/`, each compared with its script on the same inputs — this repository's backlog and metrics in a scratch clone, temporary repositories for `story-finish` and `branches-clean` — for the same output, files written and exit code. Measured on 2026-10-18, the TypeScript script by `node` as `package.json` ran it, the release binary by its path, medians in milliseconds; other agents' work kept the load average between 6 and 33, noted with each measure.

| Tool and call | Script | Binary | Calls | Notes |
| --- | --- | --- | --- | --- |
| `check-all`, every script `true`, in the CI's mode | 230 | 119 | 20 | its own work outside the lanes |
| `backlog-status SP-023 draft` | 327 | 172 | 20 | `backlog:index` and its formatter included, about 150 of the 172 |
| `epic-check E16`, `check:all` a stub | 1,969 | 1,945 | 5 | the three `gh` calls take the rest |
| `story-finish`, up to its pull request, `gh` a stub | 150 | 107 | 20 | a temporary repository |
| `replace`, a two-file edit, the shell's own start taken off | 99 | 7 | 20 | load 15 to 17 |
| `branches-clean`, nothing to clean, a local `origin` | 314 | 235 | 20 | `lsof` takes most of it |
| `records-save`, nothing changed, no transcripts nor library | 92 | 16 | 20 |  |
| `records-save`, the 48 MB library too | 186 | 41 | 5 |  |

Through `npm run -s` and `cargo run`, any of them costs about 150 ms more (`backlog-status` without its arguments: 153.5 ms against 3.1 ms by its path), the floor [research note 0025](../research/notes/0025-f21-agent-tools-in-rust.md) measured: so the agent calls them by their path ([above](#the-binaries-the-agent-calls), ADR-0078).

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
| `transcript:find <text>` | the transcript lines holding an id or a text, with their time and type | a stuck task found by `grep` in a 30 MB transcript |
| a summary of OpenType and CSS Fonts | the fields the project uses, read once | OS/2 read 38 times, CSS Fonts 4 27 times in E15 |
| `bench <command>` | N calls after a warm-up, median, mean and the load average, printed as a Markdown row | six F21 stories wrote the same timer script again (I100) |
| a writer of tool incident cards | a dated card: the tool, how it was used, why it failed, what was done instead | nine cards written by hand on 2026-10-18, a few lines each: not worth a tool yet, a test of their format first (RET-004, P9) |

## Rust

**Every new script, hook or agent tool is written in Rust** (ADR-0073; Product Owner, 2026-10-16): the agent runs its tools hundreds of times a session, a Rust tool starts without Node.js and reads Oxc's crates directly, and a tool written in TypeScript now would be rewritten later. The TypeScript tools may still be edited until F21 rewrites them; the rule covers the tools around the library, never `src/` nor the playground. A tool is a crate of `tools/`, a member of the root workspace, run by `cargo run --release -q -p <crate>` from the scripts, and by its binary's path, `target/release/<binary>`, from the hooks, the status line, the Git hooks and the agent ([above](#the-binaries-the-agent-calls), ADR-0078); the library itself stays dependency-free.

| Crate | Run by | Does |
| --- | --- | --- |
| `backlog-index` | `backlog:index`, `backlog-status` | the stories index from each story's front matter; `--check` names the first row that differs |
| `format` | `format:check`, `fix`, the pre-commit hook, `backlog:index`, `story:finish` | the one formatter: oxfmt, then Markdown tables without padding (ADR-0068, [formatting](../conventions/formatting.md)); `--check` changes nothing; `--write` prints nothing when no file changed, how many otherwise |
| `heavy-lock` | `check:all`, `test`, `golden:check`, `docs:build`, `mutate`; `story:finish` (`--lock merge`) | one heavy command at a time across worktrees, and the merge lock |
| `hooks` | every Claude Code hook (`guard-bash`, `limit-agents`, `library-pages`, `check-handback`, `track-tool-call`, `log-tool-call`, `session-state`), the status line, `context-size` | the Claude Code hooks and the status line in Rust, and what they share ([agent](./agent.md#hooks)) |
| `shared-files` | the `pre-push` hook; the main thread, before a `done` commit (`move-rows`) | `refuse-shared-files` and `move-rows` ([below](#shared-files-and-their-rows)) |
| `worktree` | `worktree-new` | a story's worktree with `node_modules`, the hooks folder and the release binaries |
| `git-hooks` | `prepare`, `worktree-new`, the Git hooks of `.githooks/` | `install` sets the hooks folder; `pre-commit` runs the tasks on the staged files ([Git workflow](./git-workflow.md#hooks-githooks)) |
| `secret-scan` | `lint:secrets`, the pre-commit hook | secretlint's recommended rules and its comment filter in Rust: every tracked file, `--staged`, or the paths given, less `.secret-scan-ignore`'s and the `.gitignore` files'; exits 0, 1 on a finding, 2 on a fatal error (EN-110) |
| `git-sequences` | `git-sequences sync`, `git-sequences commit`, `git-sequences state` | the frequent Git sequences in one call each, and a checkout's state in one read |
| `run-totals` | the agent (`start`, `end`, `should-stop`), `session-report --run`, the status line | a run's marker, duration and tokens, counted incrementally; whether it stops at the week's limit |
| `metrics` | the agent (`metrics`, `session-report`, `activity`), the Monitor (`watch-task`) | the metrics of the agent's work, the rules' findings per epic included, the transcripts read incrementally; what the agents do; the watchdog of a background task ([metrics](./metrics.md#reading-the-transcripts)) |
| `val-prepare` | `val-prepare` | a feature's checklist lines |
| `release` | the agent's `release notes` and `release check`; `release publish`, by the button of `release.yml` | the features' text drafted and checked; the version computed from the tags and the GitHub release made ([releases](./releases.md)) |
| `page` | `page` | one row or one section of a repository page, by its id or its heading ([above](#tools)) |
| `mutate` | `mutate` | the mutations in a copy of the repository |
| `lint` | `lint`, `fix`, the pre-commit hook, after oxlint | the project's linter on Oxc: the project's rules, those oxlint lacks and the Markdown rules, each file parsed once ([lint](../conventions/lint.md)) |
| `rule-docs` | the `commit-msg` hook, at a `done` commit | asks a story that changes `docs/domain/` for its Q row, its VAL's decisions to confirm and its guide page, unless the message says `Rule-docs: none affected, <why>` |
| `api-pages` | `docs:api`, `docs:dev`, `docs:build` | the API pages from `src/index.ts` and its TSDoc ([documentation site](./documentation-site.md)) |
| `declarations` | `build` | the library's `.d.ts` from each module alone (ADR-0071, [build](./commands.md#build)) |
| `library` | `.mcp.json` (`library-mcp`), `library`, `metrics`, `records-save` | the local source library: its MCP server on `rmcp` and its command line, over one SQLite file ([sources](./sources.md)) |
| `stale-binaries` | `rust:fresh`, the last step of `check:rust`; the `session-state` hook | the release binaries older than their sources, from Cargo's dep-info (ADR-0078) |
| `tool-use` | every binary of the workspace, by its `main` | each use of a tool, one line of `.git/metrics/tool-uses.jsonl` ([metrics](./metrics.md#the-uses-of-each-tool)) |
| `repo-tools` | `check:all`, `backlog-status`, `epic-check`, `story:finish`, `replace`, `branches-clean`, `records-save` | the backlog and repository tools ported from `scripts/` with their output unchanged (EN-103), one binary each: `check-all` ([the lanes](./commands.md#the-lanes-of-the-full-check)), `backlog-status`, `epic-check`, `story-finish`, `replace`, `branches-clean`, `records-save` ([their times](#the-backlog-and-repository-tools-in-rust)) |
| `calendar` | every crate with a date, `tool-use` among them | the dates and times of the workspace, once (NAM-002): the days of a date and back, the days of a month, a duration in hours, minutes and seconds (`REF-HINNANT-DATES`), an instant written as ISO 8601 or as Temporal writes it (`REF-TEMPORAL-INSTANT`), today and the clock |
| `git-repository` | `tool-use`, the hooks, and every tool that asks Git | a checkout's folders and branch read from the files Git writes, without starting Git, and a Git command's output or why it gave none (`stdout_of`), once (NAM-002) |
| `test-support` | the Rust tests | `test_support::command`, a command without the inherited `GIT_*` variables, for every test that runs `git` ([tests that run Git](./commands.md#tests-that-run-git)) |

`worktree` and `git-sequences` decide what they do in their library, without Git nor files, and their binary only runs it, so that the decisions are tested without a repository (functional core, ADR-0014).

- **The workspace**: one virtual workspace at the root (`Cargo.toml`, `members = ["tools/*"]`), whose edition, licence, `publish = false` and lints (`clippy::pedantic`, `unsafe_code` forbidden) each crate inherits by `[lints] workspace = true`; the toolchain pinned by `rust-toolchain.toml`, with rustfmt, clippy, llvm-tools and the `x86_64-pc-windows-msvc` target; warnings denied (`.cargo/config.toml`); one `Cargo.lock` tracked and one `target/` ignored.
- **Each use recorded** (EN-102): every binary's `main` hands its work to `tool_use::run` under its own name, so that `target/release/metrics` lists its calls, failures and times ([metrics](./metrics.md#the-uses-of-each-tool)); `tools/tool-use/tests/binaries.rs` fails on a binary that does not.
- **Dependencies**: none from crates.io but the reviewed ones of `[workspace.dependencies]`, each pinned exactly with its row under [Crates](./dependencies.md#crates).
- **Checks**: `npm run check:rust` — `cargo fmt`, Clippy, the advisories and a check for Windows — in the `rust` lane of `check:all`, and `npm run rust:coverage` — the tests under the coverage floor — in the `rust coverage` lane beside it; both skipped when the branch changed no Rust input ([lanes](./commands.md#the-lanes-of-the-full-check), EN-130).
- **Portable as a goal** (2026-10-17): a crate uses the standard call where one exists (`File::try_lock`, `fs::hard_link`); a Unix-only call — a file's inode, a process group, a folder symbolic link, an execute bit — sits behind `cfg(unix)` with its reason in a comment, and the Windows check fails on one planted outside it. Nothing runs on Windows in a CI.
- **Coverage**: `npm run rust:coverage` fails under 93 % of lines or of functions of the whole workspace, the measure of 2026-10-17 rounded down. The floor is the workspace's, not each crate's: the table printed on failure shows which crate lacks tests. It is raised when a story raises the measure, never lowered.
- **Advisories**: `npm run rust:advisories` checks `Cargo.lock` against RustSec, unmaintained and yanked crates failing too, no advisory ignored (`deny.toml`); `tests/rust-advisories.test.ts` plants a vulnerable crate and sees the check fail.
- **Tests**: beside their crate's code; every test removes every inherited `GIT_*` variable from each command it starts, Git itself or the crate's binary that runs it ([tests that run Git](./commands.md#tests-that-run-git)).
- **A fresh machine** installs rustup (`https://rustup.rs`), which installs the pinned toolchain at the first `cargo` call, then `cargo install --locked cargo-llvm-cov@0.9.1 cargo-deny@0.20.2` and `cargo deny fetch db`. Without them `rust:coverage` and `rust:advisories` fail and name the command to run: a missing tool never passes a check.

## Shared files and their rows

The shared files — the backlog's three indexes, the four logs of `docs/process/` (the journal, the improvement and feedback logs, the tool incident cards), the feature files, the checklists `playground/checklists/epic-*` — are written by the main thread only (ADR-0065, decision 4): parallel workers editing them would conflict at every merge. A story's rows reach the logs in its `done` commit, which the main thread makes in the story's worktree (ADR-0091), so that nothing sits between two stories' bubbles in the graph.

- `refuse-shared-files`, first in the `pre-push` hook, refuses a worker's story branch (`US`, `EN`) whose commits not yet on the remote change one, naming each file and its commit. It lets through the story's `done` commit (`Closes: <ID>`) setting its status and moving its rows into the logs — a log that only gains lines, each a blank line, the journal's `## <today> — Run continued` or one of that log's rows or card lines the same commit removes from the story file, both read at the commit and its parent; another story's row, a deleted or rewritten line is refused, the log named (AUD-025) —, and a change that only pads or unpads a shared file's Markdown tables, so that the formatter can rewrite them; a changed cell, row or other line is still refused, and a checklist in any commit. The other branches — `develop`, the feature branches, the stories the main thread writes (`SP`, `CHK`, `AUD`, `VAL`, `REV`) — pass.
- Its limits are accepted (AUD-020): it runs on the pushing machine only, so a commit pushed first under another branch passes, and GitHub checks nothing. It keeps workers from editing shared files by mistake, not on purpose.
- A worker writes the rows meant for a log in its story file, under `## For the main thread`, beneath `### Journal`, `### Improvements`, `### Feedback` or `### Incidents`. Before the `done` commit the main thread runs `target/release/move-rows <branch>` in the story's worktree: it moves them into the logs, drops those headings and prints the files it wrote, which the `done` commit holds with the statuses, the pre-commit hook formatting them. `story:finish` commits nothing on the base: a story file still holding rows, or lines `move-rows` left, at `HEAD` — read by `git show`, not in the working tree, a file it cannot read a stop too (AUD-025) — stops it before its pull request, naming the recovery: the `done` commit made again, `git reset --soft HEAD~1`, `move-rows`, the lines left fixed by hand, then `target/release/git-sequences commit -F <message file> --push-with-lease <files>`, never an amend.

| Log | Rows moved | Dropped | Left in the story file, named on standard error |
| --- | --- | --- | --- |
| `journal.md` | a bullet opening with its tag (``- `#run` …``), at the end of the newest section, or under a new section `## <today> — Run continued` when that one is older | a placeholder (`- None.`) | any other line |
| `improvements.md`, `feedback.md` | a table row whose first cell is a date, after the table's last row | a placeholder, a header and separator | any other line |
| `tool-incidents.md` | a card: its heading `#### <date> — <title>` and the bullets and indented lines (a continuation, a nested bullet) under it, at the log's end under `## <date> — <title>` | a placeholder | a card whose heading has no date, its bullets; a card at another level (a dated `## ` or `### ` heading) or a heading deeper than `#### ` inside a card, with the lines after it, `### Incidents` kept above them (AUD-025); any other line |

`tests/logs.test.ts` fails on a row written twice — in the incidents log a whole card, heading and bullets, since two cards may share a bullet (AUD-025) —, a line that breaks a log's table, an untagged journal bullet, or a card without its date, its use (`- Used:`), its failure (`- Failed:`, `- Fails:`) or its workaround (`- Instead:`), a `- Seen:` standing for both the use and the failure; a label may hold a parenthesis before its colon (`- Used (found by AUD-024):`).

`rule-docs` gathers the files of the story's commits — those whose `Refs:` or `Closes:` names the story or one of its tasks — and the files staged; a file changed counts, not the rows inside it. It runs at commit, not at push, so that a refused `done` commit is made again rather than amended after a push.
