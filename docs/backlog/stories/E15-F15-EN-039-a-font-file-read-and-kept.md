---
id: EN-039
epic: E15
feature: F15
title: A font file read, named and kept, and whether it may be embedded
status: ready
points: 3
---

# E15 · F15 · EN-039 — A font file read, named and kept, and whether it may be embedded

The library reads a TrueType or OpenType file's bytes — chosen by the designer or given by the browser for an installed font — into a face: its family, weight and style from the file, the metrics F05 reads, and whether its `fsType` permits embedding the whole file; a kept collection of families takes each face once (ADR-0053, `DERIV-font-embedding`, research note 0020).

## Acceptance criteria

- Given a font whose name ID 1 is `Arial`, without name ID 16, `usWeightClass` 400 and `fsSelection` 0x0040, when read, then the face's family is `Arial`, its weight 400 and it is upright; given a font whose name ID 16 is `Inter` and name ID 1 `Inter Light`, then its family is `Inter` (`REF-OT-NAME`).
- Given `usWeightClass` 300, 50 and 950, when read, then the weight is 300, 100 and 900; given `fsSelection` 0x0001, then the face is italic, given 0x0040, upright (`REF-OT-OS2`).
- Given `fsType` 8 in an `OS/2` table of version 3 (Arial), 0 (Inter), 4, 0x0100, 1, and 10 in version 2, when asked whether the file may be embedded, then yes; given 2, and 0x0208 in version 4, then no; given 0x0208 in version 1, then yes (`DERIV-font-embedding`, step 3 and its checks).
- Given bytes that are not a TrueType or OpenType font (`ttcf`, `wOFF`, `wOF2`, an empty file), when read, then no face is given, never an error (as F05).
- Given a kept collection holding Inter's families, when the face of Arial 400 upright is added, then Arial is listed with that face and Inter is unchanged; when another file of Arial 400 upright is added, then it replaces the first (family compared without regard to case); when Arial 700 is added, then Arial holds two faces; the collection given is never changed (a new one is returned).
- Given a text in `arial` 700 and the collection above, when measured, then it is measured with the face of Arial 700 read from its file (`REF-CSS-FONTS-4`, §5.1, §5.2, as EN-032).
- Given 1 000 texts of two lines in a face read from a file, when their blocks are measured, then all are measured within 100 ms, the test's Vitest timeout (F15.AC4): about 12 ms are seen alone on the test machine for Inter's data (SP-018, 2026-10-15), and the loaded pre-push run was seen 2.5 times slower (`.claude/rules/tooling.md`).

## Tasks

One task = one commit, referenced as `EN-039.Tn`.

- [ ] T1 — A face's names from the `name` table (name ID 16, else 1; platform 3 strings in UTF-16BE, English US first), its weight from `usWeightClass` brought into 100 to 900 and its style from `fsSelection` bit 0 (`isBitSet`), `@see REF-OT-NAME`, `REF-OT-OS2`; fixtures built in the tests; tests first (2 h)
- [ ] T2 — Whether a file may be embedded, from `fsType` and the `OS/2` version, `@see DERIV-font-embedding` step 3; tests first, the checks of the derivation, then a property test: bit 9 set from version 2 always refuses (1 h)
- [ ] T4 — 1 000 texts measured with a face read from a file, bounded by the test's timeout, no clock read in the test (`.claude/rules/tooling.md`); tests first (1 h)
- [ ] T3 — A face read from a file's bytes, with its metrics, its file and its permission; a kept collection of families adding a face, a face of the same family, weight and style replacing the one kept; `shapes.md` (Fonts from files) updated; tests first (2 h)
