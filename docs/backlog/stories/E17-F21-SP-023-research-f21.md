---
id: SP-023
epic: E17
feature: F21
title: Research for F21
status: draft
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

[Research note 0025](../../research/notes/0025-f21-agent-tools-in-rust.md). The times are medians and means of 20 calls, of 5 for a tool over a second, and of one call for `metrics:session` without `--all`, whose run in a scratch clone could not be repeated. One question for the Product Owner, written in EN-104: how the Rust tools are called (`cargo run` costs 47 ms per call, `npm run` 107 ms more). EN-104 is left `draft`, not `parked`: that status comes with F20 (EN-086), and this branch's backlog test refuses it until F20 is merged.

## For the main thread

### Journal

- `#tooling` SP-023: F21's crates chosen (`rmcp` 3.5.1, `rusqlite` 0.40.2 bundled except for Windows, `serde_json`, `regex`; `curl` instead of an HTTP crate, no `oxc_resolver`), each rewritten package's behaviour and licence read at its installed version, every tool timed before its port; the launcher (`cargo run`, `npm run`) costs more per call than Node.js, a question for the Product Owner in EN-104.

### Improvements

| 2026-10-18 | agent | The Rust hooks of `.claude/settings.json` run through `cargo run --release -q --manifest-path …`: 49.9 ms per call against 2.7 ms for the binary by its path (SP-023); `library-pages` runs before every Bash call | tooling |
| 2026-10-18 | agent | The library's store is named by `LIBRARY_PATH`, the C toolchain's library search path: Apple's `python3` sets it to `/usr/local/lib`, and a library script it starts opens or creates a store at `<path>:/usr/local/lib` (SP-023; renamed in EN-106) | tool |
| 2026-10-18 | agent | `records:save` copies the whole transcripts' folder, 1.5 GB today, again at each call (SP-023) | tool |
| 2026-10-18 | agent | The `library-pages` hook refuses any Bash command holding a URL, a `printf` of a commit message with the session's link included: the message was written with the Write tool instead (SP-023) | tool |
