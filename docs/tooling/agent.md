# Agent configuration

The code is written by a coding agent, Claude Code, under the Product Owner's review. Its configuration is committed (ADR-0021): `AGENTS.md` for any agent, `CLAUDE.md` and `.claude/` for Claude Code. `docs/` stays the source of truth: the configuration only adds procedures, pitfalls, guards and tools, and links the rules it applies rather than restating them.

```mermaid
flowchart LR
  A[AGENTS.md<br/>rules for any agent] --> C[CLAUDE.md<br/>imports AGENTS.md]
  C --> R[.claude/rules<br/>pitfalls per area]
  C --> S[.claude/skills<br/>procedures]
  C --> G[.claude/agents<br/>run's workers, expert reviewers]
  C --> H[tools/hooks<br/>guards, metrics, status line]
  C --> M[.mcp.json<br/>source library]
```

## Files

| Path | Role | Loaded |
| --- | --- | --- |
| `AGENTS.md` | rules for any coding agent (`REF-CC-MEMORY`) | every session, by any tool |
| `CLAUDE.md` | imports `AGENTS.md`, then Claude Code's skills, agents, tools and habits | every session |
| `.claude/rules/library.md`, `docs.md`, `tooling.md` | pitfalls of `src/` and `playground/`, of Markdown, of tooling and tests | when such files are touched |
| `.claude/settings.json` | permissions, hooks, status line, the library server, the subagent cap | every session |
| `.mcp.json` | the source library's MCP server ([sources](./sources.md)) | every session, once approved |

## Skills

| Skill | Does |
| --- | --- |
| `/story` | a story from branch to pull request |
| `/run` | an autonomous run on the feature branch, one story at a time, up to the scope set (ADR-0072, ADR-0086) |
| `/spike` | an epic's refinement from `develop`, or a feature's short spike when questions remain (ADR-0020, ADR-0086) |
| `/derivation` | a `DERIV-*` from read sources |
| `/audit` | the feature audit with the reviewing agents |
| `/review` | a feature's demo page and checklist; the review at each return of the Product Owner (ADR-0023, ADR-0086) |
| `/retro` | the method's part of each review: tool cards, logs, playbook (ADR-0023, ADR-0086) |
| `/triage` | open issues, each with a recommendation (ADR-0026) |
| `project-rules` | the digest of `AGENTS.md` and `CLAUDE.md`, preloaded by every agent type started without them |
| `worker-brief` | a run's worker brief (ADR-0072), preloaded by `story-worker` and `researcher`, read by path by a `general-purpose` worker |

## Run's workers

**Each agent carries only what its task needs.** A subagent's first request is read again at each of its requests, about 200 per worker, so every token it starts with is paid some 200 times ([research note 0024](../research/notes/0024-f20-context-diet.md)). Each task kind therefore has its agent type, started with `omitClaudeMd` (no `CLAUDE.md`, `AGENTS.md` nor memory index), its rules preloaded as skills, and only the tools it uses; a `general-purpose` worker would also carry every tool's definition and the skill listing (`REF-CC-SUBAGENTS`), about three times the tokens.

| Agent type | Task kind | Tools | Preloaded |
| --- | --- | --- | --- |
| `story-worker` | `US`, `EN`, a `REV`'s naming pass, an `AUD`'s fixes, a `VAL`'s preparation | Bash, Read, Edit, Write, Grep, Glob, the library | `project-rules`, `worker-brief` |
| `researcher` | a research spike (`SP`) | the story worker's, `WebFetch`, `WebSearch` | `project-rules`, `worker-brief`, `spike` |
| `clerk` | unused since ADR-0086: the main thread writes the statuses, the journal, the `done` commits and the merges itself; the file stays until a review removes it | Bash, Read, Edit, Write, Grep, Glob | `project-rules` |

- **A fresh worker per story**, never one agent chaining stories: an agent's cost is its growing context re-read at each call, not its start (EN-122's worker ended near 255,000 tokens, a start costs about 10,600; ADR-0086).
- **No agent for small tasks**: the main thread writes the statuses, the journal, the `done` commits and the merges itself (ADR-0086).
- A task no type fits runs on `general-purpose`, given the brief's path inside its worktree.
- A rule a typed worker misses goes into its brief or `project-rules`, not into every agent's context.
- A story's merge and finish stay with the main thread (`story:finish`).
- `target/release/context-size` measures a type's first request; a change to a type is measured before and after.

## Expert agents

Read-only reviewers in a fresh context, each with the documents it reads first (`REF-CC-SUBAGENTS`): the skills start them in the background, one at a time (ADR-0086), and only where their independence pays: the `auditor`, and the `verifier` when the tests need a second look, at a feature's audit; the `math-expert` and the `functional-analyst` only for mathematics or business rules to check, never by default; they report and never edit, so that a review cannot change what it reviews. Those that read sources (`auditor`, `math-expert`, `functional-analyst`) have the library's tools and follow its protocol ([sources](./sources.md)): library first, the web only for a page it cannot read. Each starts like a worker, with `omitClaudeMd` and `project-rules` preloaded.

| Agent | Expert in | Started by |
| --- | --- | --- |
| `auditor` | refuting a feature: formulas, values, sources, copies | `/audit` (ADR-0064: no review per story) |
| `math-expert` | plane geometry and floating-point numerics | only for a story or an audit holding mathematics to check: `/spike`, `/derivation`, `/run`, `/audit` |
| `verifier` | tests that would catch a wrong implementation | `/audit`, when the tests need a second look |
| `functional-analyst` | business rules, domain objects, refinement questions | only for a refinement or a story holding business rules to check |
| `doc-editor` | pages that scan: structure, words, no quotes | `/audit`, when its pages need it |

Agents to add when their epic comes:

| Agent | Would bring | When |
| --- | --- | --- |
| `interaction-designer` | handles, dragging, selection, keyboard: how editors behave | E02, the first editor application |
| `svg-expert` | the SVG specification: paths, strokes, other tools' files | E31 (strokes), E41 and E42 (files) |
| `performance` | budgets, profiling, large views | E38 (views), or a first performance budget |

## Models

**Each agent type runs on the cheapest model that does its task as well**, named in its file with the reason in the file's body (EN-066; F20's second round, 2026-10-17). Sonnet 5.5 costs half of Opus 5.5 (`REF-CLAUDE-PROMPT-CACHING`); a task that derives, judges or computes values stays on Opus until a measured trial shows Sonnet does as well.

| Agent type | Model | Why |
| --- | --- | --- |
| `clerk` | `sonnet` | unused since ADR-0086; copies given facts into set formats and follows the fixed steps of a merge or a rebase: nothing to derive or judge |
| `story-worker` | `opus` | code, hand-computed tests and the docs of its rules; until one story on Sonnet, measured beside one on Opus, shows Sonnet does as well |
| `researcher` | `opus` | judges what each source states; the feature is built from it |
| `auditor` | `opus` | re-derives formulas and recomputes values; a weaker reviewer than the worker misses more |
| `verifier` | `opus` | mutations by thought and recomputed values |
| `math-expert` | `opus` | formulas, derivations and precision, where the tests' expected values come from |
| `functional-analyst` | `opus` | finds the two readings of a rule and prepares the Product Owner's decisions |
| `doc-editor` | `opus` | a rewrite keeps every fact of the page; read-only and short, so its share of the tokens is small |

The model is taken from the first of these that names one (`REF-CC-SUBAGENTS`): the `Agent` call's `model`, the agent file's, `CLAUDE_CODE_SUBAGENT_MODEL`, the main conversation's. A file naming `opus` stays on Opus whatever the main conversation runs on. The session report gives the tokens per model ([metrics](./metrics.md#session-report)).

## Hooks

| Hook | Event | Does |
| --- | --- | --- |
| `guard-bash` (Rust) | `PreToolUse` (shell; `Write`, `Edit`, `NotebookEdit`) | refuses what would skip a check or lose work: `gh pr merge` (and the API's merge), `--no-verify` (`commit -n`, a merge's too), the hooks' folder changed (`core.hooksPath` set, unset, its section removed, by `-c`, `--config-env` or `GIT_CONFIG_KEY_<n>`; read, it goes through), a force push without lease (`-f`, `--force`, `--mirror`, a `+` refspec), a push to `main` or `develop` (a refspec's destination, `--all`, or the current branch when none is named, read in every repository the push may run against: the folders the `cd` before it may have entered, only the brief's `cd "<folder>" \|\| exit 9` at the top of the line entering one for certain, then its `-C`, `--git-dir` and `GIT_DIR`; a place the guard cannot know, a `cd` in a subshell or a condition, `pushd`, `env -C`, a variable, `~`, a `GIT_DIR` set apart, `GIT_WORK_TREE`, or a branch it cannot read, refuses such a push, AUD-033), the heavy lock taken by hand (`mkdir …/heavy.lock` or `install -d`, or a folder under `.git` named by a variable or a pattern, which a `git push` inside it waits on forever, EN-119, AUD-033); a full Vitest run outside `npm run test`, which takes the heavy lock; in a story's worktree, a Git command chained with another, answered with the form to run, Vitest's mode `golden-write`, which writes the shared golden file, and a Python program writing a file, through any wrapper (`env`, `timeout`, `bash -c`) and its options read as Python reads them (`-Ic`; by `-c`, a here-document, a here-string or `/dev/stdin`: `.write(` on anything but `sys.stdout` and `sys.stderr`, `write_text(`, `os.replace(`, `shutil`'s writing functions, an `open(` whose mode writes, a path's `.open(mode)`, an `os.open` to write, a program run by `os.system`, `subprocess` or `exec(`, its output redirected to a file or piped into a `tee` of one), answered with the Edit tool and `replace` (EN-119, AUD-033); an `npm run` script missing from the `package.json` the command runs against, naming the branch, the rebase that brings a newer script, and the binary that replaces a removed one (EN-115); here-document bodies skipped as the shell reads them: opened by an unquoted `<<` outside a comment, a backquote or an arithmetic expansion, their word the shell's with its quotes removed, their body after the end of the logical line, one opened in a `$(…)` closing in it (AUD-033); the text a shell reads from its input (`bash`, `sh`, `zsh` without `-c` nor script, `-s`, `/dev/stdin`, `source /dev/stdin`) is judged as commands, its here-documents, here-strings and what the line's other commands print into it, and so are the substitutions of an unquoted here-document's body, which the shell expands. Each command is read word by word as the shell reads it, unquoted (`"-f"` is `-f`), its substitutions (`$(…)`, backquotes), subshells and the command each wrapper runs judged too (`VAR=…`, `env`, `command`, `sudo`, `nice`, `timeout`, `xargs`, `heavy-lock`, `bench` after its `--runs <count> --` (EN-141), `cargo run … --` and `cargo r … --` after cargo's global options (`+stable`, `-q`, `--config k=v`, `-Zx`, AUD-035), `bash -c`, `eval`, `find -exec`, a `-c alias.<name>`), and Git's options as Git reads them: bundled (`-uf`), a long one by any unique prefix (`--no-veri`), configuration keys without case (AUD-023); a forbidden command written inside a quoted text that no shell runs goes through (`git commit -m "fix -n"`, `grep "gh pr merge"`), and so does a message of several lines or holding a URL. The command a program runs of its own (`node -e`, `python -c`), a script file or an alias set earlier by `git config` or in a Cargo configuration is not read. A command opening with `cd <folder>` is judged in that folder, its word read as the shell reads it, and one entering a worktree by a later `cd` is judged there too; the brief's `cd "<worktree>" \|\| exit 9` then a chain of Git reads goes through, joined by `;`, `&&`, `\|\|` or a line feed (`status`, `log`, `diff`, `show`, `rev-parse`, `rev-list`, `ls-files`, `ls-tree`, `grep`, `blame`, `cat-file`, `describe`, `for-each-ref`, `merge-base`, `branch` listing only; no option before the subcommand, no `--output`, `--ext-diff` nor `grep -O`; EN-119), as does one Git command of any kind, each alone or piped into read-only filters (`head`, `tail`, `grep`, `wc`, `cut`; `sort` without `-o`, `--output` nor `--compress-program`; `uniq` with one file at most; `sed -n` whose scripts only choose and print lines, without `-i`; `awk` without `system`, `getline`, a pipe, a redirection nor `-f`; no substitution, no redirection to a file, `2>&1` allowed), while a Git command that writes or another command chained with a Git command, or a job sent to the background by `&`, is refused (EN-104). On `Write`, `Edit` and `NotebookEdit`, a worker whose prompt names a story's worktree still on disk, the folder of its brief's `cd "<worktree>" \|\| exit 9`, is refused a file of the main checkout or of another worktree, its path's links and case resolved by the file system, the same file in its own worktree named (I12, EN-119, AUD-033); the shared `.git` folder, a path outside the checkout, the main thread and the read-only types are left alone |
| `limit-agents` (Rust) | `PreToolUse` (`Agent`, `SendMessage`) | refuses a fourth subagent, started or resumed (ADR-0072), naming the three running; a message that starts none goes through; backed by Claude Code's own cap `CLAUDE_CODE_MAX_CONCURRENT_SUBAGENTS=3` (`REF-CC-ENV-VARS`), which names no subagent and which a resume escapes |
| `library-pages` (Rust) | `PreToolUse` (`WebFetch`, shell) | refuses a `WebFetch`, or a `curl` or `wget` printing a page, with the `library_get` call to make, unless the library stored that page with fewer than 300 characters or its site refused it; a local host, an API or a registry, a file to download or saved under a name that is not a page's, a link check (`-o /dev/null`), a script or an archive piped into `sh`, `bash`, `zsh` or `tar`, headers only or data sent go through; quotes and here-documents are read as the shell does; an agent whose file gives it no library tools is told to name the page to its caller ([sources](./sources.md)) |
| `check-handback` (Rust) | `PreToolUse` (`SubagentHandback`) | refuses once a worker's hand-back whose story branch is not on `origin` at its worktree's head, or whose report leaves a commit unnamed, the reason its next instruction; a second one passes with what is missing above the report; complete, adds a line naming the story, its branch and `story:finish`; leaves alone the main thread, read-only types and agents without a worktree. The story is its branch's, else its folder's; its commits are counted from its feature branch, or `develop` for an epic's own story, and without either the naming is not checked; ten unnamed commits are listed, the others counted. A `general-purpose` agent is checked, so a reviewer naming a worktree is given a read-only type. `SubagentHandback` exists only in auto mode (`REF-CC-HOOKS`): outside it, nothing is checked |
| `session-state` (Rust) | `SessionStart` | prints the branch, uncommitted files and the started features' progress; removes the files of calls left running for a day |
| `track-tool-call` (Rust) | `PreToolUse`, in the background | writes the call starting, for the status line ([metrics](./metrics.md)) |
| `log-tool-call` (Rust) | `PostToolUse`, `PostToolUseFailure`, `SubagentStop`, in the background | logs each call and its duration; on a subagent's stop, removes the files of its calls still shown running ([metrics](./metrics.md#the-calls-running), EN-120) |
| `status-line` (Rust) | status line, every second | place in the backlog, step, session, call running ([metrics](./metrics.md#status-line)) |

`limit-agents` and `check-handback` let the call through when they cannot read their input, so that a hook's own failure never blocks work. `guard-bash` alone fails closed, both its handlers', the shell's and the writes', `onFailure` set to `block` (`REF-CC-HOOKS`, AUD-023, AUD-033): a binary missing (a clone before `npm install`, a `cargo clean`) or crashing would otherwise let every shell command and every write through unjudged; it blocks them instead, the Bash calls rebuilding it included, so a person runs `npm run rust:build` once. `library-pages` runs on every `WebFetch` and only on a shell command whose text names `curl` or `wget`, the handlers' `if`, `Bash(*curl*)` and `Bash(*wget*)` (`REF-CC-HOOKS`, `REF-CC-PERMISSIONS`): a command it refuses names one, `/usr/bin/curl` or `sh -c 'curl …'` too, so the filter skips none, and the other commands do not start it. It and `check-handback` stop after 30 seconds and let the call through, in place of the 600 of the default, set when they ran through `cargo run`, which could wait on Cargo's build lock during a `check:all`. Every hook and the status line run their release binary by its path, `${CLAUDE_PROJECT_DIR}/target/release/<hook>` in exec form (ADR-0078, `REF-CC-HOOKS`): about 3 ms a hook, against 47 ms through `cargo run` and 59 to 106 ms for the TypeScript hooks they replace; the status line 10 ms a refresh against 180 ms, 1.9 % of a core against 24.5 % (EN-104). The binaries are the main checkout's, rebuilt at `prepare`, after `git-sequences sync`, after `story:finish` and after a pull that changed a Rust input (the `post-merge` and `post-rewrite` hooks, EN-131); `session-state` names one older than its sources, and `npm run rust:fresh` fails on it. The Rust hooks live in `tools/hooks`, each with its tests.

## Pushes

During a run, the main thread pushes a notification only for a question that parks a story or a blocker it cannot get around, and only when either prevents finishing the scope asked; never for a merge nor a stop (run skill). A push interrupts the Product Owner, so it is kept for what needs them. `PushNotification` raises a desktop notification, and a phone push when Remote Control is connected (`REF-CC-TOOLS-REFERENCE`). A project's settings can turn Remote Control off, never on (`REF-CC-REMOTE-CONTROL`), so the Product Owner sets it once on their side:

| Setting | Where |
| --- | --- |
| Remote Control for every session | `remoteControlAtStartup: true` in the user settings (`~/.claude/settings.json`), or `/remote-control` in one session |
| phone pushes | **Push when Claude decides** in `/config`, the Claude app signed in with the same account |

A push is skipped while the Product Owner types in or focuses the terminal. Without it, the question is still asked in the chat and written in its parked story.

## Parallel sessions

The Product Owner may talk to a second session while a run works. Both share the main checkout, which belongs to the run (its branch, its uncommitted files), and the repository is the only state both read. So the second session:

1. **Never touches the run's folder** (no `git switch`, `stash` nor commit there): it works in its own worktree, `target/release/worktree-new <ID>` for a story, or `git worktree add .claude/worktrees/<topic> -b <type>/<story-id>-<topic> origin/develop` for a tooling change without a story of its own, which takes the id of the story it serves: the pre-push hook refuses a branch without one, as the auto-merge does ([Git workflow](./git-workflow.md#branches-and-pull-requests)); each Bash call starts with `cd "<worktree>" || exit 9`.
2. **Writes each decision in the repository** — domain, ADR, backlog, `AGENTS.md`, skills — on one branch into `develop`; a durable fact also goes to the agent's memory.
3. **Tells the run**: `ListAgents`, then `SendMessage` with what changed, where and what it changes for the run. The run acts on the change once it reaches `develop`, never on the message alone.
4. **Removes its worktree** once merged (`target/release/branches-clean`).

Before each story the run merges `origin/develop` into the feature branch when `develop` changed `AGENTS.md`, `CLAUDE.md`, `.claude/`, `docs/domain/`, `docs/adr/`, `docs/backlog/` or `docs/conventions/`, in a worktree of that branch (never a force push on a feature branch), and follows the new versions from that story on; a change to the run's scope or stories is a stop.

## Cloud agents and scheduled tasks

Product Owner, 2026-10-17: every agent works on the Product Owner's machine, in its repository; cloud agents are not used. A scheduled task (`CronCreate`) is made only when the Product Owner asks for one.

## Searching past conversations

The session journal ([journal](../process/journal.md), ADR-0024) is read first; the transcripts, `~/.claude/projects/<project-path>/*.jsonl` (kept in the private records, ADR-0081), are the full archive:

```sh
cd ~/.claude/projects/<project-path>
grep -l "radius" *.jsonl                                     # which conversation mentions a word
jq -r 'select(.type == "user") | .message.content | strings' <id>.jsonl | grep -i "radius"
```

## Rules for this folder

- Every hook and the status line are Rust binaries of `tools/hooks` (ADR-0073, EN-104), each tested in `tools/hooks/tests`; a ported hook writes the same bytes as the TypeScript one did, its JSON as `JSON.stringify` writes it.
- A rule a hook enforces is not repeated as an instruction: the hook's refusal says it when it matters.
- A guard that blocks legitimate work is changed by a new ADR, never bypassed; `.claude/settings.local.json` (personal, not committed) may add permissions, never remove a guard.
- Changes go to `develop` as tooling, with `chore(tooling): …` commits.
