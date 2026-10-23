---
id: EN-150
epic: E02
feature: F43
title: The playground's tests fast again
status: done
points: 2
---

# E02 · F43 · EN-150 — The playground's tests fast again

First in F43's order: `playground/history/undo-and-redo.test.ts` takes 80 to 140 s under `check:all`, and its test `[F40.AC2] takes a step at each key on the hue slider, the plane's drag and the color typed` goes over Vitest's 5 s, failing pushes again and again. The convention makes a merely slow test faster rather than raising its limit (`docs/conventions/testing.md`, Isolation and time). SP-031 found where the time goes ([note 0032](../../research/notes/0032-f43-corrections-from-the-first-try.md), §2): every mount of the page in a test file leaves its listeners on the document, so that a key is heard once per test before it; and happy-dom's `querySelector`, which `selectElement` calls at each redraw, walks the tree after each change of the document (`REF-HAPPY-DOM-QUERY`).

## Acceptance criteria

The measures are the median of 5 runs of the file alone (`npx vitest run <file>`), as `.claude/rules/tooling.md` asks.

- Given the page mounted, when it is mounted again in the same document, as each test's `beforeEach` does, then the listeners the mount before put on the document and its window are removed (`REF-MDN-ADD-EVENT-LISTENER`, `signal`): after 48 mounts, 7 undos by Ctrl + Z redraw the page 14 times, as after 1 mount, where they redrew it 343 times (49 per key); the page in the browser, mounted once, behaves as before.
- Given `selectElement(document, id)`, then it returns the element of that id as before, read by `getElementById`, and throws the same error for an id the page lacks.
- Given `playground/history/undo-and-redo.test.ts`, then its 62 tests pass in under 5 s alone, measured at 29.2 s before (3.9 s with both cures in SP-031's trial), and the hue slider's test in under 200 ms alone after the 47 tests before it, 2 447 ms before (116 ms in the trial), every limit left at Vitest's 5 s.
- Given the other playground files, then none is slower: `view/zoom-and-pan`, `selection/write-several-values`, `installed-fonts/installed-fonts` and `selection/select-shapes`, 2.3, 2.6, 2.9 and 4.1 s before, 0.8 to 1.5 s in the trial.

## Tasks

- [ ] T1 — The mount's listeners kept with an `AbortController` per document (a `procedure`, `playground/mount/`), each listener of the document and the window given its signal, the controller before aborted at each mount; its test, a key heard once after two mounts, failing first (1 h)
- [ ] T2 — `selectElement` by `getElementById`; the four files and the hue slider's test measured and written in the story's report; no limit raised (1 h)

Files: `playground/mount/mount-playground.ts`, the 9 modules that listen on the document or the window (`shape-drawing/listen-to-shape-drawing.ts`, `picker-controls/listen-to-picker-closing.ts`, `history/listen-to-history-keys.ts`, `history/listen-to-history.ts`, `zoom/listen-to-zoom-keys.ts`, `mount/listen-to-inputs.ts`, `selection-keys/listen-to-space.ts`, `selection-keys/listen-to-nudge-keys.ts`, `selection-keys/listen-to-selection-keys.ts`), `playground/page/select-element.ts`. `@see` `REF-MDN-ADD-EVENT-LISTENER`, `REF-HAPPY-DOM-QUERY`.

One task = one commit, referenced as `EN-150.T1` in the commit footer.
