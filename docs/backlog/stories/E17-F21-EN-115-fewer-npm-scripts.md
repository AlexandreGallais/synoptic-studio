---
id: EN-115
epic: E17
feature: F21
title: The npm scripts kept to what a person runs and what calls them
status: in-progress
points: 2
---

# E17 · F21 · EN-115 — The npm scripts kept to what a person runs and what calls them

Decided on 2026-10-18: `package.json` held 52 scripts, of which the Product Owner runs a few; the agent already calls every Rust tool by its binary, `target/release/<binary>` (ADR-0078), so the scripts kept only for it are clutter. `package.json` keeps the scripts a person runs (`dev`, `build`, `preview`, `docs:dev`, `fix`, `check:all`, `test`) and those that something calls by their name — npm's lifecycle, a Git hook, the CI, a lane of `check:all`, a tool, or another script kept — each found by a search, never assumed. Every other script goes, and the pages and rules name the binary instead. It joins F21, done, since it changes how F21's tools are called after its validation.

## Acceptance criteria

- Given `package.json`, when the story ends, then each script left is one a person runs or one with a caller named in the inventory below, and every other script is gone.
- Given a page, a rule, a skill, a tool's message or doc comment, or a test that is not a historical record, when it names a removed script, then it names the binary or the command that replaces it instead; the journal, the retrospectives, the done stories and the ADRs' bodies stay as they were written.
- Given `docs/tooling/agent-tools.md`, when the story ends, then it lists the binaries the agent calls, without a column of scripts.
- Given an agent typing `npm run <removed script>`, when `guard-bash` refuses it, then its reason says that a tool is called by its binary, not only that a rebase may bring the script.
- Given `golden:write` removed, when a worktree runs its Vitest command (`--mode golden-write`), then `guard-bash` still refuses it.
- Given the change, when `check:all` runs, then it is green.

## Tasks

One task = one commit, referenced as `EN-115.Tn`.

- [x] T1 — The story file (15 min)
- [ ] T2 — The inventory of the 52 scripts and their callers (30 min)
- [ ] T3 — The scripts without a caller removed; the code, the tests, the pages and the rules on the binaries; ADR-0082 (2 h)
