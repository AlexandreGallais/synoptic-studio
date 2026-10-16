---
id: EN-064
epic: E17
feature: F20
title: A one-hour prompt cache on long agents
status: draft
points: 1
---

# E17 · F20 · EN-064 — A one-hour prompt cache on long agents

Use the one-hour prompt cache for long-running agents, as the research of 2026-10-16 recommends; the change is shown to the Product Owner before it is applied. The saving is measured, not assumed.

## Acceptance criteria

- Given the recommended setting shown to the Product Owner and accepted, when it is applied, then the tokens billed for a long agent are measured before and after and recorded.
- Given no measured saving, when the story ends, then the setting is reverted and the finding recorded.

## Tasks

One task = one commit, referenced as `EN-064.Tn`.

- [ ] T1 — Show the setting, apply it, measure before and after (2 h)
