---
id: US-091
epic: E02
feature: F47
title: Space never left held after Cmd
status: ready
points: 2
---

# E02 · F47 · US-091 — Space never left held after Cmd

As a symbol designer, I want the view to pan only while Space is held, so that a drag after every key is released always selects.

Seen by the Product Owner on 2026-10-24 on F46's branch, on their Mac: Space pressed, Cmd pressed, Space released, Cmd released; then a drag on the empty drawing pans the view, as if Space were still held, until Space is pressed and released again. US-090 lets Space go at Cmd's `keyup` (`playground/selection-keys/listen-to-space.ts`, `removeSpaceUnderCommand`), which does not hold there.

A likely cause, to check: Cmd + Space is a system shortcut on macOS (Spotlight, the input source, or another application's, such as the Product Owner's dictation tool), which takes the keys from the page without the window losing the focus, so that neither Space's nor Cmd's `keyup` reaches it; the page's `HELD_SPACES` then stays set. The fix cannot rest on a `keyup` the page may never hear.

## Acceptance criteria

- Given Space held (`HELD_SPACES`), when a press reaches the drawing, then the view pans only if Space is still held: the page knows it from what it hears at the press itself (for instance a Space `keydown` repeated within the system's repeat time, or the press's own state), never from a `keyup` that may not come; the chosen way and why written here, with its source.
- Given the sequence of the Product Owner (Space down, Cmd down, Space up, Cmd up), with or without the `keyup`s reaching the page, when the empty drawing is dragged, then a selection rectangle is drawn.
- Given Space really held, then a drag pans, as today (US-078).

## Product Owner test

`npm run dev`, `http://localhost:5173/`.

| Step | Do | You should see |
| --- | --- | --- |

## Tasks

- [ ] T1 — The cause checked (a headless Chromium from the machine's caches for the events a page receives with Cmd + Space; MDN or the UI Events specification for key repeat and `KeyboardEvent.repeat`), the way chosen and written; tests first, failing (45 min)
- [ ] T2 — The fix, the tests passing; the Product Owner test card (45 min)

One task = one commit, referenced as `US-091.Tn` in the commit footer.
