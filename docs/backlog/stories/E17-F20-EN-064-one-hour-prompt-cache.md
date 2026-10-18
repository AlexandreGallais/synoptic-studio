---
id: EN-064
epic: E17
feature: F20
title: A one-hour prompt cache on long agents
status: dropped
points: 1
---

# E17 · F20 · EN-064 — A one-hour prompt cache on long agents

Dropped on 2026-10-17, closed by SP-022's measure with the Product Owner's agreement: nothing is left to set ([research note 0024](../../research/notes/0024-f20-context-diet.md), EN-064).

- The main thread already writes only to the one-hour cache: a subscription within its included usage gets it there automatically (`REF-CC-ENV-VARS`), and the transcripts of 2026-10-17 show 2,397,275 tokens written to it and none to the five-minute cache.
- For subagents, a one-hour cache would cost 47 % more on cache writes to save the 8.6 % of writes made after a pause of more than 5 minutes; it pays only above 38 % (`REF-CLAUDE-PROMPT-CACHING`).

## Acceptance criteria

- Given the measure of SP-022, when F20 starts, then no cache setting is changed and the finding stays in research note 0024.

## Tasks

None: closed by the spike.
