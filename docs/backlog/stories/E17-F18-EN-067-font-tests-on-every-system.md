---
id: EN-067
epic: E17
feature: F18
title: Font tests on every system
status: ready
points: 2
---

# E17 · F18 · EN-067 — Font tests on every system

Make the font tests still skipped outside macOS run on every system (F18.AC19). EN-062 left the Helvetica.ttc, Apple Color Emoji, Arial and STIXGeneral tests macOS-only; the Product Owner decided on 2026-10-17 that the tests check the code — the reading of a collection, of a large table, the import of a second family — not a given installed font, so they must not depend on one.

## Acceptance criteria

- Given the tests of `src/io/font-tables/`, `src/io/font-face/`, `src/io/font-collection/`, `playground/fonts/` and `playground/guided-steps/epic-15/`, when they run on any system, then none is skipped for want of a system font.
- Given a font collection with shared tables, when the reader reads it, then the case once proven on Helvetica.ttc is proven on a collection built by the test from Inter Regular's tables.
- Given a table over 191 MB, when the collection is written, then the case once proven on Apple Color Emoji is proven on a synthetic table of that size.
- Given a second family needed beside Inter, when the playground's tests import it, then it is built from Inter's file with another family name, or a font under a licence that allows shipping it, with its licence beside it.

## Tasks

One task = one commit, referenced as `EN-067.Tn`.

- [ ] T1 — A collection and a large table built by the tests (2 h)
- [ ] T2 — The playground's second family (1 h)
