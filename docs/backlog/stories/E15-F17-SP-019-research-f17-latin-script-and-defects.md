---
id: SP-019
epic: E15
feature: F17
title: "Research for F17: the Latin script in industrial HMIs, and the causes of the defects"
status: ready
points: 3
---

# E15 · F17 · SP-019 — Research for F17: the Latin script in industrial HMIs, and the causes of the defects

Find, read and record the sources of F17's stories before their implementation (ADR-0020, research protocol §8). From the Product Owner's validation of E15 (2026-10-15): the editor targets the Latin script (ADR-0060), and each defect needs its cause found before any fix.

## Acceptance criteria

- Given ADR-0060's reason that industrial synoptic views are written in Latin script, when the spike ends, then a research note records what the sources read say (HMI style guides, ISA-101, vendors' language support), each with a verified `REF-*`, or says that none was found.
- Given each defect below, when the spike ends, then its cause is found from a reproduction with numbers, written in the research note, and the story that fixes it cites it.
- Given the stories of F17, when the spike ends, then each formula or behavior they need has a verified `REF-*`, a `DERIV-*` or an open research request.

## Questions

- Do industrial HMIs and synoptic views mostly use the Latin script? Which other scripts appear, and where?
- A font collection (`.ttc`): how is one face read from it (`ttcf` header, `REF-OT-OTFF`), and how does the Local Font Access API give a face held in a collection (`REF-WICG-LOCAL-FONT-ACCESS`)? Is an imported collection read as well?
- Which installed faces fail with a `TypeError` when their file is asked for, and why?
- Weight 450 of a family such as IBM Plex: measured with the 450 face, why does the browser seem to draw another face (CSS font matching, `REF-CSS-FONTS-4` §5.2; the `FontFace` descriptors given)?
- A value changed fast (an arrow key held on the size): where does the time go, measured, and what can be kept between two redraws?
- The background band of a wrapped text: which line's band runs past the line's end, to the right, and why?

## Tasks

One task = one commit, referenced as `SP-019.Tn`.

- [x] T1 — Research note on the scripts of industrial HMIs, sources read and recorded (2 h)
- [x] T2 — Causes of the font defects: collections, `TypeError`, weight 450, each reproduced (3 h)
- [x] T3 — Causes of the lag and of the background band, measured; note finished and stories updated (2 h)
