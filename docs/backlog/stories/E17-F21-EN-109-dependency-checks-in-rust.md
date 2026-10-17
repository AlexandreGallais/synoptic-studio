---
id: EN-109
epic: E17
feature: F21
title: The dependency checks in Rust, without npm-check-updates nor knip
status: ready
points: 5
---

# E17 · F21 · EN-109 — The dependency checks in Rust, without npm-check-updates nor knip

Two dev packages and one script rewritten in Rust (F21.AC4, the feature's table of dev packages):

- `npm-check-updates` (`deps:outdated`): every package on its latest version published at least 7 days ago (`.ncurc.cjs`, the cooldown of ADR-0038, `REF-NCU-COOLDOWN`); `tool-versions.ts` (`deps:tools`), which checks the GitHub Actions of the workflows and Node.js's LTS, joins it, and with it the crates of `[workspace.dependencies]`, so that one command checks every version of the repository.
- `knip` (`deps:unused`): unused files, exports and dev packages, rewritten on Oxc's parser (the `oxc` crates of the workspace), for this repository's layout: one export per file (ADR-0019), barrels, the entry points of `package.json`, `.mcp.json`, `.claude/settings.json` and the workflows. Without `oxc_resolver` (`REF-OXC-RESOLVER`): it would add 59 crates, and the imports here are relative paths (3,933) or bare package names (SP-023, [research note 0025](../../research/notes/0025-f21-agent-tools-in-rust.md)).

What the two packages do here, read at their installed versions (0025, The rewritten packages; `REF-NCU-COOLDOWN`, `REF-KNIP-ISSUE-TYPES`): `ncu` falls back, when the latest version is inside the cooldown, to the greatest version outside it; it includes prereleases when the current version is one (`vitepress` 2.0.0-alpha.20); `errorLevel 2` exits 1 when a package can be upgraded. Knip runs its default issue types with 11 plugins enabled here (commitlint, GitHub Actions, husky, lint-staged, Node.js, Oxfmt, Oxlint, TypeScript, Vite, VitePress, Vitest), each adding its tool's entry files and binaries. The registry, GitHub and Node.js's index are read by `curl` and `gh api`, the answers by `serde_json`, without an HTTP crate (0025, The crates). The crates' check also reads `Cargo.lock`: on 2026-10-18 F20's lock held `syn` 3.0.9 and `unicode-ident` 1.0.27, under 7 days, which a check of `[workspace.dependencies]` alone misses.

`npm audit` stays: it is part of npm. Story EN-113's clean-up relies on the unused-file check made here.

## Acceptance criteria

- Given `package.json`, the workflows, `.nvmrc` and `[workspace.dependencies]`, when `npm run deps:outdated` runs, then it lists each package, action, Node.js major and crate behind its latest version at least 7 days old, with that version and its date, and exits 1 when one is listed; on the current tree it lists the same npm packages as `ncu` with `.ncurc.cjs`, and the same actions and Node.js major as `tool-versions.ts`.
- Given a registry or GitHub that does not answer, when the check runs, then it says which and exits 2, never 0.
- Given the tree, when `npm run deps:unused` runs, then it reports the same unused files, exports and dev packages as `knip` with `knip.json`, on the tree and on planted cases (an unused file, an export imported nowhere, a dev package no file nor script names); `knip`, `knip.json`, `npm-check-updates`, `.ncurc.cjs` and `tool-versions.ts` with its test are gone.
- Given the checks, when they are timed, then each one's time before and after is stated, the network's share apart.

## Files

`tools/deps/` (new crate: `outdated` and `unused`), `Cargo.toml` (`serde_json` if EN-106 has not brought it), `Cargo.lock`, `package.json`, `package-lock.json`, `.ncurc.cjs`, `knip.json`, `scripts/tool-versions.ts` and its test (removed), the `check-all` lanes (EN-103), `docs/tooling/versions-and-security.md`, `docs/tooling/dependencies.md`, `docs/tooling/commands.md`, `NOTICE` and `THIRD-PARTY-LICENSES.md` if knip's code or cases are ported (ADR-0076).

## Measures (ADR-0069)

Before and after on this Mac, 10 runs each: `deps:outdated` and `deps:tools` together against the new command, the network's time apart; `deps:unused` against `knip`.

Before, measured by SP-023 (0025, Each tool's time before its port), medians of 5 runs: `ncu` 2.14 s and `tool-versions.ts` 0.78 s, network included; `knip` 0.60 s.

## Tasks

One task = one commit, referenced as `EN-109.Tn`.

- [ ] T1 — Latest versions at least 7 days old for packages, actions, Node.js and crates, in one command; tested on recorded registry answers (5 h)
- [ ] T2 — The unused files, exports and dev packages on Oxc, on knip's verdicts and planted cases (6 h)
- [ ] T3 — The packages, configurations and script removed; `check:all` and the docs; times measured (2 h)
