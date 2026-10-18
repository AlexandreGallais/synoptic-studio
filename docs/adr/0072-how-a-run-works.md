# ADR-0072 — How a run works: the go-ahead, the branches and merges, three agents and one heavy command

**Status**: Accepted (Product Owner, 2026-10-17, review of the ADRs, EN-094). Supersedes ADR-0028, ADR-0038, ADR-0045 and ADR-0063, and ADR-0050's decisions 2 and 3: it gathers what stands of them, as practised in F19's run, with the decisions of 2026-10-17. ADR-0064 (checks by level), ADR-0065 (shared files) and ADR-0067 (the release pull request on `develop`) stand beside it. Its release lines, which restate ADR-0067 (Decision, the release pull request into `develop`; the table's release row), superseded by ADR-0079 (2026-10-18, EN-112): a release branch from `develop`, published by the project's own tool.

## Context

- How a run works was spread over five ADRs, each superseding parts of the one before: feature branches and autonomous runs (ADR-0028), `develop` and local checks (ADR-0038), `/run` as the go-ahead (ADR-0045), parallel checks and validated epic stories (ADR-0050), the limit on agents (ADR-0063, after ADR-0051). Reading one of them alone gave a rule no longer applied, such as the feature branch created from `main`.
- On 2026-10-17 the Product Owner reviewed every accepted ADR with the agent. They kept the way runs work, asked for it in one up-to-date ADR, and decided two changes:
  - a **tooling feature** — one whose criteria concern the agent's tools, checks and hooks, not the library nor its applications — is validated by the agent, who opens its pull request into `develop` without waiting for them;
  - **three subagents and one heavy command at a time** is the rule, no longer a trial: E15's freeze did not come back since the hook and the lock (EN-047, EN-063).

## Decision

### Branches and merges

| Branch | Holds | Starts from | Merges into, by |
| --- | --- | --- | --- |
| `main` | released versions only, no direct push | — | — |
| `develop` | integration: validated features, tooling, documentation, backlog | — | `main` by a merge commit, by the Product Owner, when they ask for a release |
| `feature/f<nn>-<topic>` | one feature while it is built | the up-to-date `develop` | `develop` by a merge commit, from its `VAL` branch, once validated |
| `<type>/<id>-<topic>` | one story, one commit per task | its feature branch on GitHub (`develop` for an epic's story) | its feature by a rebase |

- `git log --first-parent main` lists the releases; each feature is one merge on `develop`.
- Release-please's release pull request goes into `develop`, and the Product Owner merges it just before `develop` → `main` (ADR-0067). After a release, `develop` follows `main` by a fast-forward.
- After every merge the branches are cleaned (`npm run branches:clean`); GitHub deletes a merged branch.

### Checks

1. **Checks of work branches run locally**: `pre-commit`, `commit-msg`, `pre-push` (branch name, `check:all`, the shared files). No workflow runs on a story pull request, a feature pull request or a push to `develop`; the agent never waits for a CI there.
2. **`check:all` runs its checks in parallel lanes**, so that the pre-push hook waits for the slowest lane, not the sum.
3. **One CI, on the pull request into `main`**: quality, commit messages and branch name, dependency review, CodeQL. Nothing is re-checked after the merge; a push to `main` tags, publishes, and deploys the site.
4. **Dependencies**: every package and crate at its latest version at least 7 days old, upgraded by the agent after reading its release notes; Dependabot keeps only its security alerts and updates.

### The run

1. **`/run` is the go-ahead.** The Product Owner starts a run for the next feature in their order; the agent then sets the feature `in-progress` and its stories `ready` itself. It asks only business questions, after a refinement in two rounds.
2. **The main thread only dispatches.** One worker per story, a background subagent in a worktree named after the story (`npm run worktree:new`), on the brief of the run skill. Each comment of the Product Owner is acknowledged at once; a comment about tools or rules goes to another agent, on `develop`, in its own worktree. Business questions come back from the worker; only the main thread asks them.
3. **At most three subagents at once and one heavy command at a time**, the rule: the hook `limit-agents` refuses a fourth subagent, started or resumed; `check:all`, the full test run, `golden:check`, the docs build, `mutate` and a push take the heavy lock themselves.
4. **A story ends by the main thread**: the worker pushes and stops before the `done` commit; the main thread makes it, then `npm run story:finish` opens the pull request labeled `autonomous`, and GitHub merges it into the feature by a rebase (`story-automerge.yml`). The agent never merges a pull request.
5. **A heartbeat**: one chat line when a worker starts and ends, with an estimate; the status line shows each background subagent and the current step.
6. **A user story** carries a Product Owner test card; a **checkpoint** (`CHK`) midway brings docs and methods up to date.
7. **A run stops** at a `VAL`, `REV` or `RET`, a business question, a research request, a stop of `AGENTS.md`, an audit finding that changes a requirement, a check still red after three attempts, or the end of its scope; each stop gets a journal entry and a message in French.

### Validation and release

| What | Validated by | Then |
| --- | --- | --- |
| a feature of the library or its applications | the Product Owner, on the epic's checklist page and the demo page | its pull request from `feat/val-<nnn>-…` into `develop` is labeled `autonomous` once they say the validation is done; GitHub merges it by a merge commit |
| a tooling feature | the agent, on the same checklist and demo page, the Product Owner trying the lines that concern them if they wish | the agent labels its pull request into `develop` `autonomous` without waiting |
| an epic's `NAM`, `REV` or `RET` | the Product Owner | its pull request into `develop` labeled `autonomous`; GitHub merges it by a merge commit |
| a release | the Product Owner asks for it, typically at the end of a day | the agent opens `develop` → `main`; the Product Owner merges release-please's pull request into `develop`, then `develop` → `main` |

## Reasons

| Option | For | Against |
| --- | --- | --- |
| One ADR for the runs (chosen) | one page says how a run works today; superseded rules no longer read as current | the history of each step is read in the superseded ADRs |
| Keep five ADRs, each partly superseded | nothing to write | a reader rebuilds the rule from five pages, and met rules no longer applied |
| The Product Owner validates every feature | one rule for all features | they wait on tooling they do not use, and leave its lines to the agent anyway (VAL-019) |
| The agent limit kept as a trial | room to change it | measured since E15 without a freeze; a trial never closed is a rule without its name |

## Consequences

- `AGENTS.md`, `CLAUDE.md`, the run and story skills and the worker brief cite this ADR where they cited the superseded ones.
- `story-automerge.yml` already accepts `feat/val-` branches into `develop`: a tooling feature uses the same path, its label set by the agent.
- The superseded ADRs stay as the record of each step: ADR-0028 (autonomous runs, 2026-10-08), ADR-0038 (`develop` and local checks, RET-001), ADR-0045 (`/run` as the go-ahead), ADR-0050 (parallel checks, epic stories merged), ADR-0051 and ADR-0063 (the main thread kept free, the limit on agents).

## References

ADR-0028, ADR-0038, ADR-0045, ADR-0050, ADR-0051, ADR-0063, ADR-0064, ADR-0065, ADR-0067, ADR-0069, `REF-GITFLOW`, `REF-GH-MERGE-METHODS`, `REF-GH-REQUIRED-CHECKS`, `REF-GH-AUTO-MERGE`, `REF-NCU-COOLDOWN`, [VAL-019](../backlog/stories/E17-F18-VAL-019-validate-f18.md), [EN-094](../backlog/stories/E17-F19-EN-094-adrs-reviewed-with-the-product-owner.md)
