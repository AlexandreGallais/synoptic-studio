---
id: EN-058
epic: E17
feature: F18
title: A mutation runner as a tool
status: draft
points: 1
---

# E17 · F18 · EN-058 — A mutation runner as a tool

Make the mutation runner used by the audits a tool, since Stryker is still blocked (stryker-js issue 6210 open on 2026-10-16; row 16 of RET-003; I25, I40, I48; F18.AC15).

## Acceptance criteria

- Given `npm run mutate -- <files>`, when it runs, then it applies one mutation per source file in a copy of the repository, runs the related tests, and reports caught and surviving mutations.

## Tasks

One task = one commit, referenced as `EN-058.Tn`.

- [ ] T1 — The tool, its test and its line in `docs/tooling/agent-tools.md` (2 h)
