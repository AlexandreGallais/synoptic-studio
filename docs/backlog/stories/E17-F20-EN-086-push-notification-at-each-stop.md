---
id: EN-086
epic: E17
feature: F20
title: A push notification at each stop, question or validation ready
status: draft
points: 1
---

# E17 · F20 · EN-086 — A push notification at each stop, question or validation ready

Send the Product Owner a push notification, on the desktop and on the phone, when a run stops, asks a question or has a validation ready, so that they need not watch the terminal. Asked by the Product Owner on 2026-10-17, during SP-021 ([research note 0023](../../research/notes/0023-f19-oxc-and-rust-tools.md), "The Product Owner's second round"); Claude Code's push notifications through Remote Control are sourced by SP-022 before the story is set `ready`.

## Acceptance criteria

- Given a run that stops, asks a question or has a validation ready, when it happens, then a push notification reaches the Product Owner on the desktop and on the phone, one line saying what is waited for.
- Given a run that goes on without the Product Owner, when a story ends, then no notification is sent.

## Tasks

One task = one commit, referenced as `EN-086.Tn`.

- [ ] T1 — The notification set up for a stop, a question and a validation ready, tried once on each device (2 h)
- [ ] T2 — The setting documented in `docs/tooling/agent.md` and the run skill (1 h)
