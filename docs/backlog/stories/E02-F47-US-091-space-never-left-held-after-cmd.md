---
id: US-091
epic: E02
feature: F47
title: Space never left held after Cmd
status: done
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

## The cause and the way chosen

The cause, checked by its sources, not reproduced: Cmd + Space is the system's shortcut, Spotlight or the input source (`REF-APPLE-SHORTCUTS`), and a key released while Cmd is held sends no `keyup` on macOS (`REF-WEBKIT-CMD-KEYUP`, `REF-MOZILLA-CMD-KEYUP`); a tool holding that shortcut may take Cmd's `keyup` too, so that US-090's Cmd release is never heard. A headless Chromium cannot show it: its keys are given to the page by the browser, never through macOS and its shortcuts.

The two ways the criteria name do not hold alone (agent's choice, 2026-10-24):

- a press's own state tells the modifiers only, `ctrlKey`, `shiftKey`, `altKey` and `metaKey`, never Space (`REF-UIEVENTS-KEYS`, §3.5.1.1);
- Space's repeat comes "at a rate determined by the system configuration", after a configuration-dependent delay (`REF-UIEVENTS-KEYS`), and macOS lets it be Off (`REF-APPLE-KEY-REPEAT`): no source bounds the time, so that a window would either keep a pan after a release, or drop Space really held.

The way chosen: **any key or pointer event that carries Cmd (`metaKey`) lets Space go**, a press with Cmd included, and a Space `keydown` with Cmd holds it not. Cmd's own `keydown` carries it, a modifier's state changing as it is pressed down (`REF-UIEVENTS-KEYS`, §4.3.1), and it comes before any key a shortcut may take, an order inferred, not observed: `REF-MOZILLA-CMD-KEYUP` shows Cmd pressed first, then the other key, never Cmd's `keydown` lost to a shortcut (AUD-049); were it lost too, Space's repeat under Cmd, a move of the pointer or the press itself carries Cmd. Nothing rests on a `keyup`, so that US-090's Cmd release goes. Space still down as Cmd is released is held again by its next repeat, Cmd gone from it, as an arrow is; with the repeat Off, Space is pressed again.

## Product Owner test

`npm run dev`, `http://localhost:5173/`.

| Step | Do | You should see |
| --- | --- | --- |
| 1 | Press Space, then Cmd, release Space, then release Cmd; drag on the empty drawing | the dashed selection rectangle, the view not moved; before, the view moved |
| 2 | The same, quickly, several times, then once with Cmd released first; drag on the empty drawing after each | the rectangle each time |
| 3 | Hold Space and drag on the empty drawing, release the button, then Space | the view follows the pointer; a drag after it draws the rectangle again |
| 4 | Hold Space, press and release Cmd, keep Space held a second, then drag | the view follows the pointer once Space repeats; with the key repeat Off, press Space again |

## Tasks

- [x] T1 — The cause checked (a headless Chromium from the machine's caches for the events a page receives with Cmd + Space; MDN or the UI Events specification for key repeat and `KeyboardEvent.repeat`), the way chosen and written; tests first, failing (45 min)
- [x] T2 — The fix, the tests passing; the Product Owner test card (45 min)

One task = one commit, referenced as `US-091.Tn` in the commit footer.
