---
id: EN-108
epic: E17
feature: F21
title: The Git hooks in Rust, without husky, lint-staged, commitlint nor validate-branch-name
status: done
points: 5
---

# E17 · F21 · EN-108 — The Git hooks in Rust, without husky, lint-staged, commitlint nor validate-branch-name

Four dev packages rewritten as one Rust crate (F21.AC4, the feature's table of dev packages): `husky`, which installs the hooks at `npm install`; `lint-staged`, which runs the formatter, the linters and the secret scan on the staged files; `@commitlint/cli` with `config-conventional` and `types`, which checks each commit message; `validate-branch-name`, which checks the branch name. Each took 0.7 to 2.6 s beside the test lane ([research note 0023](../../research/notes/0023-f19-oxc-and-rust-tools.md), Everything around the library); a commit runs two of them. Carried from EN-093's note (measured in `docs/conventions/rules.md`, Checks, hooks and conventions): the staged-file runner without ESLint's step; commitlint's rules as they are, with the 14 scopes (`eslint` dropped); the branch-name pattern as one regular expression; and a check added, that a commit on a story branch carries `Refs: <ID>.Tn` or `Closes: <ID>` with the branch's own id.

Friction of F20's run folded in (2026-10-17): the formatter, run at each commit on the staged files, prints "Tables without padding: 353 of 537 Markdown files changed" when it changed nothing.

## Acceptance criteria

- Given a fresh clone, when `npm install` runs, then a Rust command sets the hooks folder once (`core.hooksPath`), the hooks are short shell files calling the binaries, and `husky` is gone from `package.json`; a worktree made by `worktree:new` still gets its own hooks (EN-052), and `guard-bash`'s guard on `core.hooksPath` stays.
- Given staged files, when the pre-commit hook runs, then each is formatted, linted and scanned for secrets by the same commands, on the same patterns, as `.lintstagedrc.json` runs them today, the fixes staged again, and a partly staged file keeps its unstaged lines; `lint-staged` and `.lintstagedrc.json` are gone.
- Given a commit message, when the commit-msg hook checks it, then it gives the same verdict as commitlint with `config-conventional` and the project's rules on every commit of `develop`'s history and on the refused cases of commitlint's own tests, read at 21.2.3; the 14 scopes live in one place that the docs name, and commitlint's packages and `commitlint.config.ts` are gone.
- Given a commit on a story branch, when its message carries no `Refs: <ID>.Tn` nor `Closes: <ID>` with the branch's own id, then the hook refuses it, naming the footer expected; a feature, `develop` or `docs/` branch is not asked for one.
- Given a branch name, when the pre-push hook checks it, then one regular expression gives the same verdict as `.validate-branch-namerc.json` on every branch name of the repository's history, and `validate-branch-name` is gone.
- Given a pull request into `main`, when the CI's `history` job checks it, then it runs the same Rust checks on its branch name and on each of its commits, in place of `npx validate-branch-name` and `npx commitlint`.
- Given the formatter with nothing to change, when it runs, then it prints nothing; when it changes files, it names how many.
- Given a commit of one file and a push, when they are timed, then the pre-commit and commit-msg hooks' times and the branch check's are stated before and after.

## Files

`tools/git-hooks/` (new crate), `tools/format/src/main.rs`, `.husky/` (moved or renamed, decided in T1), `tools/worktree/`, `.github/workflows/ci.yml` (`history`), `package.json` (`prepare`, devDependencies), `package-lock.json`, `.lintstagedrc.json`, `commitlint.config.ts`, `.validate-branch-namerc.json` (removed), `tests/` cases that read them, `docs/tooling/git-workflow.md`, `docs/tooling/commands.md`, `docs/tooling/dependencies.md`, `docs/conventions/rules.md`, `AGENTS.md` and `.claude/skills/project-rules/SKILL.md` (where the scopes are named), `NOTICE` and `THIRD-PARTY-LICENSES.md` if a rule's code or cases are ported (ADR-0076).

## Measures (ADR-0069)

Before and after, 20 rounds each on this Mac: a commit of one Markdown file and of one TypeScript file, hooks included; the commit-msg hook alone; the branch check alone; `npm install`'s `prepare`.

Before, measured by SP-023 ([research note 0025](../../research/notes/0025-f21-agent-tools-in-rust.md), Each tool's time before its port), medians: a commit of one Markdown file 842 ms and of one TypeScript file 1,023 ms, hooks included (20 rounds, load 8 to 13); `commitlint --edit` 222 ms; `validate-branch-name` 44 ms; `husky` 48 ms.

Before and after, measured by EN-108 on 2026-10-18 at 02:38, load 3.1 to 4.7, in two scratch clones of the repository: the feature branch before the story (husky, lint-staged, commitlint, validate-branch-name, its release binaries built) and the story's branch (`.githooks/`, the release binaries by their path); one process started and waited for, its wall time taken by a timer script of the scratch folder (Python's `perf_counter`), the before and after series run one after the other. Medians, the mean in parentheses, in milliseconds:

| Measure | Rounds | Before | After |
| --- | --- | --- | --- |
| `git commit` of one changed Markdown file, every hook run | 10 | 716.5 (719.2) | 356.2 (359.7) |
| `git commit` of one changed TypeScript file, every hook run | 10 | 989.1 (989.1) | 616.5 (620.7) |
| the commit-msg check alone: `commitlint --edit`, then `git-hooks commit-msg` | 20 | 217.9 (217.8) | 23.7 (23.9) |
| the branch check alone: `validate-branch-name`, then `git-hooks branch` | 20 | 43.9 (44.6) | 12.8 (13.2) |
| `prepare`'s hooks step: `husky`, then `git-hooks install` | 20 | 47.6 (48.1) | 21.4 (21.7) |

The commits gain about 360 ms each, the message's check 194 ms of it; what is left of a commit is the commands the hook runs, oxfmt through `node_modules/.bin` and oxlint's type check on a TypeScript file. `git-hooks install` starts `git` two or three times: it asks whether the folder is a work tree, reads `core.hooksPath`, and writes it when it names another folder.

What each package does here, behaviour by behaviour, read at the installed versions (0025, The rewritten packages; `REF-HUSKY-INSTALL`, `REF-LINT-STAGED`, `REF-COMMITLINT`, `REF-VALIDATE-BRANCH-NAME`): husky's stubs and its `PATH` with `node_modules/.bin`, which the hooks need for `oxlint`; lint-staged's `--diff-filter=ACMR`, its base-name globs with dot files, its backup stash and its patch for partly staged files; commitlint's parser (the footer token at a line's start, `BREAKING CHANGE`), its rules and levels, its ignored merges and fixups, the URL exemption, `subject-case` through `es-toolkit`'s case functions. Licences: all MIT but `conventional-changelog-conventionalcommits` (ISC); a rule's code or cases ported brings its `NOTICE` row (0025, Licences). The branch pattern and the message rules need no crate beyond `regex` 1.13.1, shared with EN-110.

## Tasks

One task = one commit, referenced as `EN-108.Tn`.

- [ ] T1 — The hooks installed by a Rust command, without husky; worktrees keep their own (3 h)
- [ ] T2 — The staged-file runner, a partly staged file kept; the formatter silent when nothing changed (4 h)
- [ ] T3 — The commit message: commitlint's rules and the 14 scopes, the story branch's footer; tested on the history and commitlint's cases (5 h)
- [ ] T4 — The branch name as one expression; the packages and configurations removed; docs; times measured (2 h)

## For the main thread

- **The switch of `core.hooksPath` is shared by every checkout.** Once `prepare` or `worktree:new` runs `git-hooks install` from a checkout of this story's code, the main checkout and every worktree run `.githooks/`: a checkout on a branch without that folder, `develop` until F21 is merged, runs no Git hook at all. The other way, an `npm install` on `develop` runs husky, which sets `.husky/_` again, and a worktree of the feature branch then runs husky's stubs, which find no `.husky/<hook>` and run nothing. Until F21 reaches `develop`, run `target/release/git-hooks install` once in the checkout that makes the next worktree, and no `npm install` on `develop`. Each checkout keeps its ignored `.husky/_` folder until it is deleted by hand.
- **This worktree's T4 commit and push ran no hook**, for that reason: its `core.hooksPath` is husky's `.husky/_`, whose stubs no longer find a hook. T1 to T3 ran the old hooks, then from T2 and T3 the new runner and message check through `.husky/` forwarding to `.githooks/`. For T4 the same checks were run by hand: `git-hooks pre-commit` on the staged files, `git-hooks commit-msg` on the message, `refuse-shared-files`, `git-hooks branch` and `npm run check:all` before the push.
- ADR-0078 (EN-104) names `.husky/` as the Git hooks that call the binaries by their path; they are `.githooks/` now, calling `target/release/<binary>` through `.githooks/tool.sh`, with `cargo run --release -q -p <crate>` only when the binary is missing. The feature asks the install to be named in an ADR: whether ADR-0078 gets a successor, or a new ADR records `git-hooks install` and the folder, is the main thread's call; no ADR is written here.
- EN-110 adds `regex` 1.13.1 too: `Cargo.toml`'s `[workspace.dependencies]` line and the row of `docs/tooling/dependencies.md` (Crates) are written here and will conflict at its merge, the same text expected.
- The guard of `tools/hooks` (EN-104) still names lint-staged and commitlint in its refusal of a hook switched off, and `tools/lint`'s comments name lint-staged's absolute paths (the new runner passes paths from the root): wording only, left for their next edit or EN-113.
- commitlint's own tests are cited by `REF-COMMITLINT-TESTS`, added to `docs/references.md`; the same verdicts were checked on the 1,011 commits of every branch (956 of `develop`, its root aside, which neither refuses): 8 refused for `scope-enum` (`eslint`), 19 warned for `footer-leading-blank`, the same commits and rules from commitlint and from the port. Two stash commits of `--all` are left out, on no branch. The port lists `start-case` beside `sentence-case` for a capitalised subject with an accented letter, where commitlint's `deburr` strips the accent first; the verdict is the same (`case.rs`).
