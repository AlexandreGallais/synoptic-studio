---
id: SP-033
epic: E02
title: Regroup the epics not done into small epics of one subject
status: done
points: 2
---

# E02 · SP-033 — Regroup the epics not done into small epics of one subject

Decided at REV-008 (2026-10-24, ADR-0100, decision 1): before the next epic starts, every epic not done (E18, E19, E13, E03, E04, E05, E06, E07, E08, E09, E10, E11, E20, E12, and E16 given) is read again and regrouped into small epics holding one subject each; the Product Owner allows any of them to be dropped and written again. The spike proposes the new list; the Product Owner orders it; the epic files are rewritten only after that.

## Acceptance criteria

- Given the epics not done, when the spike ends, then a note lists the proposed epics, each with one subject, its features and each feature's count of stories (2 to 4, each of at most 3 tasks), and a duration from the measured medians by kind (a spike about 20 minutes, a story about 25, an audit's fixes 45 to 75).
- Given every feature, idea and card of the epics not done, when the spike ends, then the note maps each to the new epic that takes it, or says why it is dropped: nothing is lost silently.
- Given the questions the regrouping raises, when the spike ends, then they are listed for the Product Owner, each with options and a numeric example, the order of the epics among them.

## Questions

The inventory of T1 (2026-10-25), read at `develop`'s head: every epic not done, its features (none of them has a feature file yet), its ideas and the cards placed in it from elsewhere, what it needs before it, and the domain files it rests on. An item is numbered by its epic (E18.1 …) so that the note's map can name it.

**E18 — Transform shapes and points by hand** (`shapes.md` §1 Rotation, §3; `interaction.md` §2, §3; ADR-0003, ADR-0035). Needs E02.

- Features: E18.1 resize by the box's eight handles, the 3 × 3 reference point, polygon and round shape stretched by their own rules, Shift keeping the proportions (3); E18.2 turn about a chosen center, a rotation mode, one angle only (3); E18.3 handles on the turned and the straight boxes, centers on points (3); E18.4 drag points and a curve's handles (2 to 3); E18.5 number fields that compute, `+10` moving several points (2); E18.6 a history step kept cheap with 1 000 shapes (AUD-043) (1).
- Ideas (F04, F08 refinements): E18.7 the rotation mode and its centers; E18.8 one angle only; E18.9 handles on both boxes; E18.10 the reference point of x and y; E18.11 the straight box hidden or dashed; E18.12 points dragged together on integers; E18.13 a shape stretched from a fixed point, strokes keeping their width.
- Cards placed in it from elsewhere: E18.14 the box of several shapes' position shown (SP-032); E18.15 `+100` typed in Width seen not to work (E20's ideas, third try of E02).

**E19 — Snap, align and distribute** (`interaction.md` §4, §5 Align and distribute; `REF-INKSCAPE-SNAP`). Needs E02, E18.

- Features: E19.1 snap to objects (3); E19.2 guides and equal gaps (3); E19.3 align and distribute (2). Out of scope there: snapping to ports (with E05).

**E13 — Arrange shapes in a tree and lay them out** (`shapes.md` §3 Layout box, §7; `interaction.md` §6; Q21, Q25). Needs E02; before E03 (ADR-0006: booleans are nodes of the tree).

- Features: E13.1 the shape tree in the model; E13.2 a layout box with an anchor; E13.3 a child overriding the anchor; E13.4 rows and columns, gap, padding.
- Ideas: E13.5 the name "layout" to be sourced; E13.6 stacks fixed once the symbol is used (settled); E13.7 F07's anchor reused.
- Cards: E13.8 a group resized proportionally (SP-032); E13.9 the layers panel, out of its scope, "priority end of project" (`interaction.md` §6); E13.10 Q25, the layout box's radii.

**E03 — Combine shapes without losing their numbers** (`shapes.md` §5; ADR-0006; Q9; `REF-MARTINEZ-2009`, `REF-GREINER-HORMANN`, `REF-INKSCAPE-SHAPEBUILDER`). Needs E13.

- Features: E03.1 union and difference; E03.2 intersection and exclusion; E03.3 division and cut path; E03.4 the shape builder.
- Ideas: E03.5 operations on edges (cut the contour between two points, join two points, delete an edge, add a connection, rebuild shapes from the pieces).
- Not written there: the operands' cubic sides (ADR-0046, ADR-0048), which came after the epic was drafted for segments and arcs.

**E04 — Give shapes colors and exact strokes** (`shapes.md` §6; `interaction.md` §5 Colors; ADR-0002, ADR-0056; Q4). Needs F16 (done).

- Features: E04.1 a shape's fill color, with F16's fields and picker; E04.2 the aligned stroke and its color; E04.3 joins, caps and dashes; E04.4 the strokes of E14's curves (In scope).
- Cards: E04.5 `src/geometry/contour-band/` and `DERIV-band-within-box`, without a caller since US-086, kept for this epic to decide (SP-032, AUD-046 finding 8).

**E05 — Build animatable symbols** (`symbols-and-views.md` §1 to §3; Q13). Needs E13.

- Features: E05.1 ports; E05.2 animatable parts; E05.3 the animation preview.
- Ideas (`symbols-and-views.md` §1, For later): E05.4 a clickable zone; E05.5 a shared status zone; E05.6 the saved position (Q30, with E11). Card: E05.7 snapping to ports (from E19).

**E06 — Configure business types in cascade** (`configuration.md` §1, §2, §5; Q14). Needs E05.

- Features: E06.1 configuration tree; E06.2 interfaces and rules; E06.3 property groups and defaults. In scope, without a feature: E06.4 trees combined; E06.5 lines hidden by default.

**E07 — Publish business symbol libraries** (`configuration.md` §3, §4). Needs E06.

- Features: E07.1 business symbol; E07.2 business library. Card: E07.3 text styles as local templates, "with the libraries' epic" (`shapes.md` Text, Not built).

**E08 — Assemble synoptic views** (`symbols-and-views.md` §4; `configuration.md` §5; ADR-0008). Needs E07.

- Features: E08.1 instances (with duplication); E08.2 rotation by quarter turns; E08.3 default overrides with a live preview; E08.4 pop-up lines.
- Cards: E08.5 a font embedded once per view (F15); E08.6 a long text scrolled in its box on a touch screen (F14).

**E09 — Connect symbols with orthogonal pipes** (`symbols-and-views.md` §5; ADR-0004; `REF-WYBROW-2009`, `REF-MARRIOTT-2014`, `REF-LIBAVOID`). Needs E05's ports, E08.

- Features: E09.1 port-to-port pipe; E09.2 obstacle-avoiding routing; E09.3 manual adjustment; E09.4 bends and crossings.

**E10 — Draw and reuse static drawings** (`shapes.md` §8; ADR-0018; Q12). Needs E13, E08.

- Features: E10.1 drawing tools with a Figma-like pen; E10.2 a group saved as a drawing; E10.3 the drawing library shared between projects.
- Notes: E10.4 point 1 alike in every shape (decided at RET-002, 2026-10-14: the rectangle from its bottom-right corner, the round shape from its right point), to build before E11 fixes the file format.

**E11 — Store and share as files in a working folder** (`configuration.md` §6; `symbols-and-views.md` §6; `editions.md` §1; ADR-0030; Q5, Q20, Q23, Q30). Needs every kind of document it saves.

- Features: E11.1 file formats; E11.2 the working folder.
- Ideas: E11.3 SVG imported from other sources, a warning past a million units, simplified and scaled down when it can be, refused beyond a level of complexity; E11.4 local only (settled, ADR-0030).
- Open questions it carries: E11.5 Q30 (saved position); E11.6 Q23 (the application shell, native code only to save files).

**E20 — Make the editor easy to use** (`shapes.md` Text; `interaction.md` §1; ADR-0062). Placed after every library epic, before E12.

- Features: E20.1 fonts found and shipped (2 to 4); E20.2 fewer text options (2); E20.3 lines of a text (2); E20.4 a turned text as a shape (2 to 3); E20.5 messages that explain (1 to 2); E20.6 keys of one's own (2); E20.7 the history's list scrolled whole, US-083's draft (1).
- Ideas not yet in a feature: E20.8 a responsive playground, a toolbar at the bottom as Figma's; E20.9 settings to tune the pan and the zoom by hand, then an expert menu; E20.10 a number field blocking a letter as it is typed; E20.11 a text's box with a negative word spacing that can no longer be picked; E20.12 a review of every button, option and message (the epic's own subject); E20.13 fonts installed automatically, a curated set (Comic Sans, Braille and emoji fonts refused); E20.14 a font that cannot be embedded, forced after a confirmation or refused (ADR-0062).
- E02's remarks given to it: E20.15 Ctrl + wheel's zoom direction as an option; E20.16 the selection rectangle taking a shape it touches by one pixel, "to discuss".

**E12 — Release a product people can trust** (`editions.md` §1; ADR-0027, ADR-0030; Q40). Last.

- No feature. In scope: E12.1 project overview; E12.2 architecture summary; E12.3 user manuals; E12.4 getting-started guide; E12.5 npm publication; E12.6 regression and end-to-end tests; E12.7 size budget; E12.8 public API report.
- Cards: E12.9 the traceability matrix as a generated page (ADR-0027); E12.10 a playground asking its own test questions and taking spoken answers (`improvements.md`, 2026-10-13, kept until a feature needs it).

**E16 — Check the contrast of texts** (Ideas for later; edition not decided; no source read).

- Features: E16.1 the contrast of a text, WCAG 2 and APCA side by side; E16.2 a verdict in words; E16.3 a minimum contrast at all times through animations.

**Not in any epic, met on the way**: D.1 duplicating a shape, its first move's offset remembered (`interaction.md` §1, For later); D.2 the flags of degenerate free shapes, "with the editor applications" (Q29, `interaction.md` §3); D.3 Q26, the radii of removed corners remembered; D.4 Q42, font subsetting at export; D.5 Q24, a `foreignObject` tool, V1 or pro; D.6 Q43, free positions, for later; D.7 the helps still drawn in the drawing's SVG, the box frames and the points' handles (ADR-0099, decision 2; `interaction.md` §7, The helps keep their size on screen), to move to one of the new epics (REV-008).

**Durations**: the measured ranges of E02's review (`docs/process/retrospectives/E02-retrospective.md`, ADR-0100 decision 7): a spike 17 to 32 minutes, a story 14 to 37, an audit's fixes 44 to 86; read as the medians a spike 20, a story 25, an audit's fixes 45 to 75.

## Tasks

One task = one commit, referenced as `SP-033.Tn`.

- [x] T1 — Inventory: each epic not done, its features, ideas and cards, its dependencies, the domain files it rests on
- [x] T2 — The new list: subjects, epics, features and their sizes, the map from the old epics
- [x] T3 — The research note and the questions for the Product Owner

## Answers of the Product Owner

Given on 2026-10-25, to the twelve questions of note 0034, §6:

1. (a) The 28 small epics as proposed, each holding one subject.
2. (a) The helps still drawn in the drawing's SVG are the first feature of Resize shapes by their box (E21), every later handle being drawn in the layer above (the agent's placement, option (a)).
3. The history's list scrolled whole stays with the interface designed (E45), as decided on 2026-10-23 at SP-029 (the agent's placement): A history that stays fast (E25) holds its one step story.
4. A letter refused as it is typed goes with Number fields that compute (E24), which rewrites how every field reads its text (the agent's placement).
5. The fill color is the first feature of Colors and strokes of shapes (E31) (the agent's placement).
6. (a) Point 1 alike in every shape is the first feature of the files' epic (E41), before the format is fixed, as RET-002 placed it.
7. The text's second pass and the fonts are epics 12 and 13 as proposed, E32 and E33 (the agent's placement).
8. (b) The contrast of texts belongs entirely to the pro edition: its epic is dropped from the list and its content kept in `editions.md` §2, the Product Owner's choice of edition (ADR-0030).
9. The editor works first as a web page, in every browser: files are opened from the disk and saved by Save as, a download, which stands in for the working folder; Ctrl + S saves in place where the browser allows it (the File System Access API, to source at that epic's refinement), otherwise opens Save as. A file is saved only when the designer asks, never continuously. The desktop application is an epic of its own (E46), placed just before the release's epics, likely a Rust program hosting a web view, how being researched then; the working folder moves to it. The reason: an editor that runs everywhere from the start, the desktop shell coming once its need is known (Q23 settled).
10. (a) The radii of removed corners are kept while the session lasts, not in the saved file: a story of Drag points and handles (E23) (Q26 settled).
11. How a rule and an animation run is settled at the refinements of Animatable parts (E35) and Business configuration (E36), from their sources (option (c), the agent's placement).
12. (a) The order of §2, with the two changes above: the contrast out, the desktop application in before the release; still 28 epics, E21 to E48 (note 0034, §7).
