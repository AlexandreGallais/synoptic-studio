---
id: US-059
epic: E15
feature: F17
title: Only fonts read and approved, imported ones too
status: ready
points: 2
---

# E15 · F17 · US-059 — Only fonts read and approved, imported ones too

As a symbol designer, I want the font list to hold only fonts read and approved, and a font I import to follow the same rules, so that I never choose a font the editor cannot measure or export.

From the Product Owner's decisions of 2026-10-16, asked by questionnaire after US-056: while testing, the font list showed every installed family as soon as **Installed fonts** listed them, before any was read, so that fonts later left out could be chosen for a moment.

**Cause, checked first**: `writeFontOptions` listed the families kept and, after them, the installed families listed by the browser and not read yet (US-036), which US-056 left as they were. A family left out appeared in the list until the reading reached it.

**Decisions** (Product Owner, 2026-10-16; `docs/domain/shapes.md`, Text, Only usable fonts listed; ADR-0062):

1. **No font choice while the installed fonts are read.** While **Installed fonts** reads, the font list cannot be used (disabled), and **Installed fonts** neither. The list holds only fonts read and approved — every basic Latin letter mapped and embeddable —; a family not read yet, or refused, never appears in it, not even for a moment. Once the reading ends, the first approved family is chosen when the text's font has none, as US-056 did; a family chosen meanwhile can no longer happen, so the guard kept for it (US-056, T3) goes. No text can be written before a font is available: while the list is empty, the Content field is disabled and the page says why; a reading that approves no font says so.
2. **A font imported by hand follows the same rules** (ADR-0062, refining ADR-0061, decision 2): a face lacking any of the 52 letters of the ISO basic Latin alphabet, or whose `fsType` forbids embedding it, is refused at import, said why (Apple Color Emoji: refused, no Latin letters). This reverses US-056's "an imported file is not judged".
3. **Ideas for the later epic on ease of use** (E02, ideas): free, exportable fonts shipped with the software; importing a font from the computer by selecting it, the software saying whether it is exportable; whether a font that is not exportable may be exported once the designer forces it with a confirmation, or is refused outright, to decide then; a font list showing which fonts are usable and which exportable; fonts installed automatically.

## Acceptance criteria (F17.AC17)

- Given Inter not drawn and installed families Test Sans, Test Emoji (no Latin letter) and Test Serif, the second still being read, when the font list is looked at, then it is disabled, holds Test Sans alone, and Installed fonts is disabled; once all are read, the list is enabled, holds Test Sans and Test Serif, and Test Sans is chosen.
- Given Inter not drawn and no font read, then the font list is empty and disabled, the Content field disabled, and no text is drawn.
- Given Inter not drawn and installed families that are all refused, when they are read, then the list stays empty, the Content field disabled, and the page says no installed font can be used.
- Given a file imported whose face lacks a basic Latin letter, then it is refused: `TestEmoji.ttf: Test Emoji 400 refused, it lacks letters of the basic Latin alphabet (A to Z, a to z).`; given one whose `fsType` is 2, then it is refused because its `fsType` forbids embedding it in an exported SVG.

## Product Owner test

| Step | Do                                                                  | You should see                                                                                     |
| ---- | ------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------- |
| 1    | Chrome, Inter not installed: open the page, Text tool               | an empty, greyed font list; the Content field greyed; no text                                      |
| 2    | Click Installed fonts, allow, and open the font list while it reads | the list and Installed fonts greyed; only families already read and approved, no Apple Color Emoji |
| 3    | Wait for `Installed fonts read: …`                                  | the list usable, the first approved family chosen, the text drawn in it                            |
| 4    | Import a font: Apple Color Emoji (/System/Library/Fonts on macOS)   | refused, the line saying it lacks the basic Latin letters                                          |

## Tasks

One task = one commit, referenced as `US-059.Tn`.

- [x] T1 — The font list holds only families read and approved, disabled with Installed fonts while they are read; the Content field disabled while no font is available; the first approved family chosen at the end of the reading; the guard for a family chosen meanwhile removed; `shapes.md`, the checklist lines, the story registered (1.5 h)
- [x] T2 — A face imported by hand without the basic Latin letters, or whose `fsType` forbids embedding it, refused and said why; ADR-0062; `shapes.md` (1 h)
- [x] T3 — The ideas for the later epic on ease of use in E02 (0.25 h)
- [ ] T4 — The process logs: the explanation of US-056 given without an example, and `story:finish` from a worktree (0.25 h)
