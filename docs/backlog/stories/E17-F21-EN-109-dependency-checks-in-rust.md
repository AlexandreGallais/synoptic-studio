---
id: EN-109
epic: E17
feature: F21
title: The dependency checks in Rust, without npm-check-updates nor knip
status: draft
points: 5
---

# E17 · F21 · EN-109 — The dependency checks in Rust, without npm-check-updates nor knip

Two dev packages and one script rewritten in Rust (F21.AC4, the feature's table of dev packages):

- `npm-check-updates` (`deps:outdated`): every package on its latest version published at least 7 days ago (`.ncurc.cjs`, the cooldown of ADR-0038, `REF-NCU-COOLDOWN`); `tool-versions.ts` (`deps:tools`), which checks the GitHub Actions of the workflows and Node.js's LTS, joins it, and with it the crates of `[workspace.dependencies]`, so that one command checks every version of the repository.
- `knip` (`deps:unused`): unused files, exports and dev packages, rewritten on Oxc's parser (the `oxc` crates of the workspace) and `oxc_resolver` (`REF-OXC-RESOLVER`), for this repository's layout: one export per file (ADR-0019), barrels, the entry points of `package.json`, `.mcp.json`, `.claude/settings.json` and the workflows.

`npm audit` stays: it is part of npm. Story EN-113's clean-up relies on the unused-file check made here.

## Acceptance criteria

- Given `package.json`, the workflows, `.nvmrc` and `[workspace.dependencies]`, when `npm run deps:outdated` runs, then it lists each package, action, Node.js major and crate behind its latest version at least 7 days old, with that version and its date, and exits 1 when one is listed; on the current tree it lists the same npm packages as `ncu` with `.ncurc.cjs`, and the same actions and Node.js major as `tool-versions.ts`.
- Given a registry or GitHub that does not answer, when the check runs, then it says which and exits 2, never 0.
- Given the tree, when `npm run deps:unused` runs, then it reports the same unused files, exports and dev packages as `knip` with `knip.json`, on the tree and on planted cases (an unused file, an export imported nowhere, a dev package no file nor script names); `knip`, `knip.json`, `npm-check-updates`, `.ncurc.cjs` and `tool-versions.ts` with its test are gone.
- Given the checks, when they are timed, then each one's time before and after is stated, the network's share apart.

## Files

`tools/deps/` (new crate: `outdated` and `unused`), `Cargo.toml` (`oxc_resolver` if chosen, its row and age checked), `Cargo.lock`, `package.json`, `package-lock.json`, `.ncurc.cjs`, `knip.json`, `scripts/tool-versions.ts` and its test (removed), the `check-all` lanes (EN-103), `docs/tooling/versions-and-security.md`, `docs/tooling/dependencies.md`, `docs/tooling/commands.md`, `NOTICE` and `THIRD-PARTY-LICENSES.md` if knip's code or cases are ported (ADR-0076).

## Measures (ADR-0069)

Before and after on this Mac, 10 runs each: `deps:outdated` and `deps:tools` together against the new command, the network's time apart; `deps:unused` against `knip`.

## Tasks

One task = one commit, referenced as `EN-109.Tn`.

- [ ] T1 — Latest versions at least 7 days old for packages, actions, Node.js and crates, in one command; tested on recorded registry answers (5 h)
- [ ] T2 — The unused files, exports and dev packages on Oxc, on knip's verdicts and planted cases (6 h)
- [ ] T3 — The packages, configurations and script removed; `check:all` and the docs; times measured (2 h)
