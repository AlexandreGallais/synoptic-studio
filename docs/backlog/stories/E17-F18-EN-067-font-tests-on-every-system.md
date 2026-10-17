---
id: EN-067
epic: E17
feature: F18
title: Font tests on every system
status: done
points: 2
---

# E17 · F18 · EN-067 — Font tests on every system

Make the font tests still skipped outside macOS run on every system (F18.AC19). EN-062 left the Helvetica.ttc, Apple Color Emoji, Arial and STIXGeneral tests macOS-only; the Product Owner decided on 2026-10-17 that the tests check the code — the reading of a collection, of a very large file, the import of a second family — not a given installed font, so they must not depend on one. No test names a font the editor refuses: Apple Color Emoji holds no basic Latin letters, so it is never offered (ADR-0061, ADR-0062); its tests only proved that a very large file met while the installed fonts are read is read as a view, without copying its bytes, before it is refused.

The font tests are meant for macOS, Windows and Linux alike (Product Owner, 2026-10-17): paths built with `node:path`, fixtures read beside the test files, no shell command, no folder of installed fonts, and the case of a table over 191 MB built without holding its bytes twice, so that a Windows runner holds it.

## Acceptance criteria

- Given the tests of `src/io/font-tables/`, `src/io/font-face/`, `src/io/font-collection/`, `playground/fonts/` and `playground/guided-steps/epic-15/`, when they run on any system, then none is skipped for want of a system font.
- Given a font collection with shared tables, when the reader reads it, then the case once proven on Helvetica.ttc is proven on a collection built by the test from Inter Regular's tables.
- Given a file with a table over 191 MB, built by the test, when it is read and written, then its bytes are read as a view and never copied, and a file without the basic Latin letters is refused; no test names Apple Color Emoji.
- Given a second family needed beside Inter, when the playground's tests import it, then it is built from Inter's file with another family name, or a font under a licence that allows shipping it, with its licence beside it.

## Tasks

One task = one commit, referenced as `EN-067.Tn`.

- [ ] T1 — A collection and a large table built by the tests (2 h)
- [ ] T2 — The playground's second family (1 h)
