---
id: US-060
epic: E15
feature: F17
title: The font chosen automatically holds its files
status: done
points: 1
---

# E15 · F17 · US-060 — The font chosen automatically holds its files

As a symbol designer, I want the font chosen for me when Inter is not installed to be embedded like a font I choose, so that every font offered can be exported with Embed fonts.

From the Product Owner's test of 2026-10-16, after US-059: after **Installed fonts**, with **Embed fonts** on, the page said `Academy Engraved LET 400: not embedded, its file not held.`, while IBM Plex Sans chosen by hand was embedded.

**Cause, checked first**: `writeDefaultChoice` set the family measured and the font list, but never read the family's files again, unlike a family chosen by hand (`readInstalledFamily`, US-053): its faces kept their metrics read ahead, without their files. It also ran after the first family approved, not once the reading ended; and every redraw measured the text with the family the list showed first, a family read ahead, while the fonts were still read.

**Decisions** (Product Owner, 2026-10-16; `docs/domain/shapes.md`, Text, Default font available):

1. **Every font offered can be exported with Embed fonts**: a font chosen never ends as `its file not held`, Inter's shipped metrics excepted, Inter being listed only when the browser draws it.
2. **The default font is chosen as a font chosen by hand is**: its faces' files read again and held; a family none of whose files can be read again leaves the list and the next approved family is tried.
3. **The default font is chosen once the reading of the installed fonts ends**, not at the first family approved; no family read ahead is measured with before.

## Acceptance criteria (F17.AC18)

- Given Inter not drawn and installed families Test Sans and Test Serif, when the reading ends, then Test Sans is chosen, its file read a second time, and with Embed fonts on no face is listed as not embedded.
- Given Inter not drawn, Test Sans read and Test Serif still being read, then no font is chosen and no text is drawn; once Test Serif is read, Test Sans is chosen.
- Given Inter not drawn and Test Sans whose file the browser gives once, then refuses, when the reading ends, then Test Sans leaves the list, Test Serif is chosen, and with Embed fonts on no face is listed as not embedded.

## Product Owner test

| Step | Do                                                                   | You should see                                                                |
| ---- | -------------------------------------------------------------------- | ----------------------------------------------------------------------------- |
| 1    | Chrome, Inter not installed: Text tool, Installed fonts, allow, wait | no text while it reads; once `Installed fonts read: …`, the first font chosen |
| 2    | Tick Embed fonts                                                     | no line `…: not embedded, its file not held.` for the font chosen             |

## Tasks

One task = one commit, referenced as `US-060.Tn`.

- [x] T1 — The default font chosen once the reading ends, as a font chosen by hand: its files read again, a family refused then left out and the next tried; no family read ahead measured with while the fonts are read; `shapes.md`, the checklist line, the story registered (1 h)
- [x] T2 — The process log: the font chosen automatically not tried with Embed fonts (0.25 h)
