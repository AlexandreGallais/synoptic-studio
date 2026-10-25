# ADR-0101 — The ADRs that no longer hold, removed into one record

**Status**: Accepted (Product Owner, 2026-10-24, improvement log; confirmed 2026-10-25, REV-009). Amends the rule "an accepted ADR is superseded, never rewritten" (`AGENTS.md`, Golden rules): superseding still applies while both ADRs hold; an ADR that no longer holds is removed and recorded here. Removes ADR-0009, ADR-0010, ADR-0013, ADR-0015, ADR-0028, ADR-0038, ADR-0041, ADR-0042, ADR-0044, ADR-0045, ADR-0051, ADR-0063, ADR-0067, ADR-0079 and ADR-0083.

## Context

- By 2026-10-25 the index held 99 ADRs, 47 of them superseded or amended, 14 wholly. A reader looking for how the project works read decisions that had given way, and followed chains (ADR-0041, then ADR-0044, then ADR-0081) to find the one in force.
- The documentation is meant to say where the project stands now (improvement log, 2026-10-24): what was dropped, cancelled or forbidden is worth one place, with its reason, rather than pages that read as current.
- Git keeps every removed file: `git log --follow -- docs/adr/<file>` or the history of `develop` before REV-009 gives its text back.

## Decision

1. **An ADR that no longer holds is removed**, its row moved from the index into the table below: wholly superseded, or describing tooling, a process or a choice that is gone. It names what it decided, why it no longer holds and what replaces it.
2. **An ADR partly superseded stays** while its other decisions hold; its status says which decision gave way to which ADR, and its row in the index says so.
3. **Superseding still applies between ADRs that both hold**: a new ADR changes an accepted one, never a rewrite of its decisions. Once the older one holds nothing, it comes here.
4. **A link or a mention of a removed ADR points to the ADR that replaced it**, or to this record; a history record (a backlog story, the journal, a log, a retrospective, a research note, `history.md`) keeps its text, a link to a removed file pointing here.
5. **A rule whose reason lived only in a removed ADR** is listed below with its reason, so that the reason is not lost with the file.

### The ADRs removed

| ADR | Title | What it decided | Why it no longer holds | Replaced by |
| --- | --- | --- | --- | --- |
| 0009 | Toolchain: TypeScript 6, Vite, Vitest, ESLint, Prettier, VitePress | the first toolchain: TypeScript 6 for typescript-eslint's typed rules, VitePress 2, ESLint and Prettier; every package a development one, pinned, justified in `dependencies.md` | the toolchain moved to Oxc and TypeScript 7 on 2026-10-17; its rules on dependencies are restated there | ADR-0075 |
| 0010 | Strictness carried by ESLint first, TypeScript second | a rule a linter can check lives in ESLint; every rule `error`; every rule of every plugin decided once | ESLint was removed on 2026-10-17; the linter first stays, the linters being oxlint and the project's Rust linter, the type errors given with the lint | ADR-0075, ADR-0071 |
| 0013 | Prettier next to ESLint, not inside it | Prettier run beside ESLint, never as its plugin; `.editorconfig` first; one `npm run fix` | Prettier and ESLint were removed on 2026-10-17; oxfmt formats beside the linters | ADR-0075 |
| 0015 | One export per file, named like the file | a module exports one value, its file named exactly like it (`filletSetback.ts`) | its file name gave way to the kebab-case of the export; its other rules, kept by ADR-0019, are listed below | ADR-0019 |
| 0028 | Feature branches and autonomous runs: stories merge into their feature, the Product Owner merges the feature | a feature branch from `main` with required checks, stories rebased into it, a run chaining stories, an auditor per story, a `CHK` and a `VAL` per feature | the branches from `develop` came with ADR-0038, then the whole was gathered into ADR-0072 on 2026-10-17; the reviews per story gave way to the audit per feature, the `VAL` per feature went at ADR-0100 | ADR-0072, ADR-0086, ADR-0100 |
| 0038 | A `develop` branch, checks run locally, one CI on the pull request into `main` | `main` for releases, `develop` for integration, the merge method per pull request, the checks local, release-please's pull request merging itself | gathered into ADR-0072 on 2026-10-17; release-please was removed on 2026-10-18, a story's rebase gave way to a merge commit (ADR-0086) | ADR-0072, ADR-0086 |
| 0041 | Private records in a sibling repository, saved by `npm run records:save` | the records in a private repository beside the project, pushed to a private remote | the records moved inside the project (ADR-0044), then to a local folder shared by every project, never sent | ADR-0081 |
| 0042 | Folders by theme, 15 modules as a signal, a shape's type in its folder | folders named by theme, 15 modules a signal | folders by business domain since 2026-10-13; its tests beside their module and a shape's type in its folder are kept by ADR-0043 | ADR-0043 |
| 0044 | The private records inside the project folder, ignored by Git | `records/` at the project's root, its own repository with a private remote | the records are in a local folder shared by every project, never sent | ADR-0081 |
| 0045 | `/run` is the go-ahead; validated features merge by themselves; releases when the Product Owner asks | the agent sets a run's stories `ready`; a validated feature merged by GitHub; `develop` → `main` on request | gathered into ADR-0072 on 2026-10-17; `main` receives each review and the release is a button since ADR-0086 | ADR-0072, ADR-0086 |
| 0051 | The main thread kept free: stories worked by subagents | the main thread dispatches, a worker per story in its worktree, reviews per story, tool comments to parallel agents | a trial, amended by ADR-0063 (2026-10-16); one agent at a time since ADR-0086, the reviews per story replaced by the audit per feature (ADR-0064) | ADR-0086, ADR-0064 |
| 0063 | Runs with a limited number of agents, enforced by the tools | at most three subagents at once, one heavy command at a time (`.git/heavy.lock`), the worker brief in the run skill | gathered into ADR-0072 on 2026-10-17; one story and one worker at a time since ADR-0086; the heavy lock stays, ADR-0072's | ADR-0072, ADR-0086 |
| 0067 | The release pull request on `develop`, only the release on `main` | release-please's pull request into `develop`, merged before `develop` → `main`; `main` only tags; `develop` fast-forwarded after a release | release-please was removed on 2026-10-18 (EN-112); the release is a button since ADR-0086 | ADR-0086 |
| 0079 | The release prepared on a release branch from `develop`, published by our own tool | `release/x.y.z` from `develop`, a version commit, the release published on the push to `main` | the release branch and the version commit were dropped on 2026-10-18 (ADR-0083, then ADR-0086) | ADR-0086 |
| 0083 | The release made by the merge into `main`: the version from the tags, no release branch nor version commit | the merge into `main` tags and publishes; `CHANGELOG.md` kept for 0.1.0 and 0.2.0 | the same day, the merge into `main` deploys the site only and the release is a button (2026-10-18); `CHANGELOG.md` deleted | ADR-0086 |

### What stays forbidden, and why

Rules whose reason was written only in a removed ADR:

- **No `export { … }` list and no re-export outside a barrel** (ADR-0015, kept by ADR-0019): a module exports one value with `export <declaration>`, so that its file's name says what it holds and a value has one place.
- **No helper shared from inside another module's file** (ADR-0015): a non-exported helper serves its file's one export only; a helper two modules use becomes its own module, so that it is reused and never copied.
- **No type beside a value unless the value's declaration uses it** (ADR-0015): such a type is the function's contract and stays with its only user; a type another signature uses is a concept of its own, in its own file.
- **No Dependabot version updates** (ADR-0038, restated by ADR-0072 without its reason): `check:all` already fails on an outdated package, so a version pull request would only repeat it; Dependabot keeps its security alerts and updates.
- **No release-please, no release branch, no version commit, no version typed** (ADR-0067, ADR-0079, ADR-0083): a version in a file drifts from the tags, and a number typed by hand can be mistyped; the version is computed from the tags at the button (ADR-0086).

## Reasons

| Option | For | Against |
| --- | --- | --- |
| Keep every superseded ADR, its status saying so (the rule until now) | nothing moves; the history reads in place | a reader follows chains to find the decision in force; a page reads as current when it is not |
| **Remove them into one record** | the index holds what is in force; one page says what went and why | a removed text is read from Git, not the site |
| Mark them in the index only | no file removed | the pages stay, read as current by search and by the agent |

## Consequences

- The index (`docs/adr/README.md`) lists the ADRs in force; its preamble, `AGENTS.md` (Golden rules), the `project-rules` skill, `docs/conventions/writing.md` and the playbook's principles say an ADR that no longer holds comes here.
- Every link to a removed file points to its replacement or here; the docs build's dead-link check keeps it so.
- A future ADR that leaves nothing of an older one moves its row into the table above in the same commit.

## References

- ADR-0019, ADR-0043, ADR-0064, ADR-0071, ADR-0072, ADR-0075, ADR-0081, ADR-0086, ADR-0100.
- `docs/process/improvements.md` (2026-10-24), REV-009.
