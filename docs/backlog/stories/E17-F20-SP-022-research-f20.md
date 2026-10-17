---
id: SP-022
epic: E17
feature: F20
title: Research for F20, the context diet
status: draft
points: 1
---

# E17 · F20 · SP-022 — Research for F20, the context diet

Find, read and record the sources of every story of F20 before its implementation (ADR-0020): how an agent's input is measured in tokens, and how a digest is kept in step with its source.

Widened by the Product Owner on 2026-10-17: measure whether the agent works faster and on fewer tokens through its own tools than through shell commands — local MCP servers in Rust for Git, file reading, web fetching with a local store of pages read, and a local memory — and propose a tool for each case where the measure says so; the goal is an agent that works simply, quickly and on few tokens, its tools reviewed at each retrospective as an engineering-tools team reviews how its tools are used. Each tool of its own also records its use — the call, its time, its outcome — into the project's metrics, so that the retrospectives see how the project was really built and change the tools or the method from that evidence (Product Owner, 2026-10-17).

## Acceptance criteria

- Given each story of F20, when the spike ends, then its open questions are answered with a source or a measure, or turned into a research request.
- Given the agent's most frequent calls in the metrics (Git, reading files, fetching pages, memory), when each is timed and counted in tokens through the shell and through a tool of its own (an MCP server or a Rust command), then the spike lists the gain of each and drafts a story for each worth building.

## Tasks

One task = one commit, referenced as `SP-022.Tn`.

- [ ] T1 — Read, measure and record the sources (1 h)
