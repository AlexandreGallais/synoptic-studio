---
id: SP-023
epic: E17
feature: F21
title: Research for F21
status: done
points: 2
---

# E17 · F21 · SP-023 — Research for F21

Find, read and record what F21's stories need before any of them starts (ADR-0020): the crates they add, the behaviour of each package they rewrite, and the time of each tool before its port, measured once for all the stories (ADR-0069). F19's spike ([SP-021](./E17-F19-SP-021-research-f19.md), [research note 0023](../../research/notes/0023-f19-oxc-and-rust-tools.md)) already measured the hooks and three scripts; this spike completes the list and checks each source again.

## Questions

1. Which crates does each story add, at which exact version at least 7 days old, under which licence, with which advisories? At least `rmcp` (`REF-RMCP`) and what it pulls in, and a SQLite crate with FTS5 (`REF-SQLITE-FTS5`) that reads the store `node:sqlite` writes today; for every other need, can the standard library or a command already installed (`git`, `gh`, `npm`) do it without a crate?
2. What does each rewritten package check, read from its own documentation or source at the installed version: commitlint's `config-conventional` rules, lint-staged's steps, `validate-branch-name`'s pattern, secretlint's recommended preset (each pattern and its source), knip's checks on this layout, npm-check-updates' cooldown (`REF-NCU-COOLDOWN`)?
3. Which licence does each package to be ported carry, so that `NOTICE` and `THIRD-PARTY-LICENSES.md` receive it in the story that ports it (ADR-0076)?
4. How long does each tool of `scripts/`, `.claude/hooks/` and `package.json` that F21 rewrites take per call on this Mac today — the "before" of every story?
5. What makes good release notes, read in their sources (`REF-KEEP-A-CHANGELOG`, `REF-COMMON-CHANGELOG`, `REF-GH-RELEASE-NOTES`, `REF-SEMVER`), and how does release-please's changelog of 0.2.0 fall short of them?

## Acceptance criteria

- Given each question above, when the spike ends, then research note 0025 answers it with a verified `REF-*` row, a measure or an open research request.
- Given each crate a story adds, when the spike ends, then the story names it with its version, licence and advisories checked, and `docs/tooling/dependencies.md` is left to the story that adds it.
- Given every tool F21 ports, when the spike ends, then the note holds its mean time per call before the port (20 calls, the release build or `node` as `package.json` runs it, this Mac), the command that measured it, and the story that ports it.
- Given the stories' criteria and tasks, when the spike ends, then they cite the sources found and their open points are written in them.

## Files

`docs/research/notes/0025-f21-agent-tools-in-rust.md` (new), `docs/references.md`, the story files of F21.

## Tasks

One task = one commit, referenced as `SP-023.Tn`.

- [x] T1 — The crates: versions, ages, licences, advisories; what a command already installed can do instead (2 h)
- [x] T2 — The rewritten packages' behaviour and licences, read at their installed versions (3 h)
- [x] T3 — The time per call of every tool before its port, one table (2 h)
- [x] T4 — The research note, the references and the stories updated (1 h)

## Results

[Research note 0025](../../research/notes/0025-f21-agent-tools-in-rust.md). The times are medians and means of 20 calls, of 5 for a tool over a second, and of one call for `metrics:session` without `--all`, whose run in a scratch clone could not be repeated. One decision, written in EN-104 by the agent under ADR-0069 (option C), to tell the Product Owner: how the Rust tools are called (`cargo run` costs 47 ms per call, `npm run` 107 ms more).
