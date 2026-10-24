# 0034 — The epics not done, regrouped into small epics of one subject

Spike SP-033, 2026-10-25, decided at REV-008 (ADR-0100, decision 1). It read again every epic not done (E18, E19, E13, E03, E04, E05, E06, E07, E08, E09, E10, E11, E20, E12, and E16 given), the domain files they rest on, `editions.md`, ADR-0030 and ADR-0100, at `develop`'s head; its inventory, each item numbered (E18.1 …), is in the story's Questions section. No new source was read: a size this note could not judge without one is an open point (§5), not a guess. Nothing here is decided: the Product Owner orders the list (§6), and only then are the epic files rewritten.

## 1. How the sizes are read

- **One subject per epic**, a theme named in a few words (ADR-0100, decision 1); an epic of the old list that held two subjects is cut in two, the second after the first.
- **A feature** holds 2 to 4 `US` or `EN` stories, each of at most 3 tasks (ADR-0084); one of 1 story is allowed only where the subject holds no more, and is named so.
- **A duration** is read from the measures by kind (ADR-0100, decision 7), the ranges of E02's review (`docs/process/retrospectives/E02-retrospective.md`: a spike 17 to 32 minutes, a story 14 to 37, an audit's fixes 44 to 86), taken as a spike 20, a story 25, an audit's fixes 45 to 75. An epic is its refinement spike, its stories, and one audit per feature (ADR-0100, decision 4):

  duration = 20 + 25 × stories + (45 to 75) × features, in active minutes.

  Example: 3 features of 2, 3 and 2 stories give 20 + 25 × 7 + 45 × 3 = 330 to 20 + 175 + 75 × 3 = 420 minutes, 5.5 to 7 hours.

- **What the measures do not cover**: they come from E02's interaction stories. No story of robust geometry (booleans, offsets, routing) has been measured by kind yet; the epics this concerns carry the open point of §5. Neither the Product Owner's returns nor a review are counted.

## 2. The proposed epics, in the proposed order

The library first, the interface last (the order rule of the epics' index, 2026-10-19): the epics that build the library and its playground, then the settings and the panels, then the interface designed, then the release. Names without ids: an id is given when the Product Owner accepts the list.

| Order | Epic | Subject | Features | Stories | Duration | Needs |
| --- | --- | --- | --- | --- | --- | --- |
| 1 | Resize shapes by their box | the box's handles and its fixed point | 3 | 7 | 5.5 to 7 h | E02 |
| 2 | Turn shapes about a chosen center | the rotation mode and its handles | 3 | 7 | 5.5 to 7 h | 1 |
| 3 | Drag points and handles | the points moved by the mouse | 2 | 5 | 3.9 to 4.9 h | 1 |
| 4 | Number fields that compute | arithmetic typed in a field | 1 | 3 | 2.3 to 2.8 h | E02 |
| 5 | A history that stays fast | the history's cost with many shapes | 1 | 2 | 1.9 to 2.4 h | E02 |
| 6 | Snap, align and distribute | shapes placed against each other | 3 | 8 | 5.9 to 7.4 h | 1, 2, 3 |
| 7 | Groups and the stacking order | the shape tree | 3 | 8 | 5.9 to 7.4 h | E02 |
| 8 | Layout boxes | a box that places its children | 3 | 8 | 5.9 to 7.4 h | 7 |
| 9 | Boolean operations | the six operations, non-destructive | 3 | 8 | 5.9 to 7.4 h, and §5 | 7 |
| 10 | Shape builder and edge operations | shapes rebuilt from their pieces | 2 | 6 | 4.3 to 5.3 h | 9 |
| 11 | Colors and strokes of shapes | a shape's paint | 4 | 10 | 7.5 to 9.5 h, and §5 | F16 |
| 12 | Text, a second pass | the text's lines, options and turn | 3 | 7 | 5.5 to 7 h | 2 |
| 13 | Fonts for synoptic views | the fonts found, shipped and exported | 2 | 6 | 4.3 to 5.3 h | E15 |
| 14 | Symbols and their ports | the symbol as a document | 3 | 7 | 5.5 to 7 h | 7 |
| 15 | Animatable parts | what moves in a symbol | 3 | 7 | 5.5 to 7 h, and §5 | 14, 11 |
| 16 | Business configuration | trees, interfaces, rules | 4 | 11 | 7.9 to 9.9 h, and §5 | 15 |
| 17 | Business symbols and libraries | a symbol's meaning per project | 3 | 6 | 5.1 to 6.6 h | 16 |
| 18 | Synoptic views | instances placed in a view | 3 | 10 | 6.8 to 8.3 h | 17 |
| 19 | Orthogonal pipes | pipes between ports | 4 | 11 | 7.9 to 9.9 h, and §5 | 14, 18 |
| 20 | Static drawings | free drawings reused | 3 | 7 | 5.5 to 7 h | 7, 18 |
| 21 | Files in a working folder | saving and reloading | 3 | 9 | 6.3 to 7.8 h, and §5 | 14 to 20 |
| 22 | SVG imported from elsewhere | another tool's SVG read | 2 | 5 | 3.9 to 4.9 h | 21 |
| 23 | The contrast of texts | a text readable on its background | 3 | 7 | 5.5 to 7 h, and §5 | 11, 15, 18 |
| 24 | Settings and keys of one's own | what the designer tunes | 3 | 6 | 5.1 to 6.6 h | E02 |
| 25 | The layers panel | the tree shown and edited as a list | 1 | 3 | 2.3 to 2.8 h | 7 |
| 26 | The editor's interface designed | every button and option questioned | 2 | 5 | 3.9 to 4.9 h, and §5 | 1 to 25 |
| 27 | Documents for newcomers | the project explained in a few pages | 3 | 7 | 5.5 to 7 h | 26 |
| 28 | Release checks | the suites and budgets of a release | 2 | 5 | 3.9 to 4.9 h | 26 |

In all: 28 epics, 75 features, 191 stories, 8 710 to 10 960 active minutes (145 to 183 hours), before the open points of §5.

## 3. Each epic's features

A feature's stories are named as they would be written; their count is the feature's size. Every name is a proposal for the epic's refinement, which may cut it again (ADR-0100, decision 2).

### Resize shapes by their box

| Feature | Stories |
| --- | --- |
| The helps in the layer above | the box frames drawn as strokes in the layer above, not in the drawing's SVG; the points' and the curves' handles moved there too, with their halo (ADR-0099, decision 2) (2) |
| The box's eight handles | the handles drawn and dragged, the opposite side fixed, on integers; a regular polygon and a round shape stretched as their own rules say; Shift on a corner keeping the proportions (3) |
| The reference point of x and y | the 3 × 3 point shown and chosen; the x and y of the box of several shapes shown at it (2) |

### Turn shapes about a chosen center

| Feature | Stories |
| --- | --- |
| The rotation mode and its center | the mode entered by a button, its center shown, the box's center by default; the center free at an x and y, or on one of the nine anchors; one angle only, a new center moving nothing (3) |
| The turned box's handles | handles on the turned box's corners, as Figma's; the straight box hidden or dashed (2) |
| Centers on points | the center on a point of the shape, following it; on a point or the center of another shape, placed only (2) |

### Drag points and handles

| Feature | Stories |
| --- | --- |
| Points dragged | a point dragged on integers; several selected points moved together; a curve's handles dragged (3) |
| Degenerate shapes flagged | two points at the same place flagged with an offer to delete one; crossing edges flagged (Q29) (2) |

### Number fields that compute

| Feature | Stories |
| --- | --- |
| Arithmetic in every number field | `+`, `−`, `×`, `÷` and parentheses read by one function of the library; every field of the page using it, `+10` moving several points or shapes, `+100` in Width reproduced; a letter refused as it is typed (3) |

A single feature: the subject holds no more.

### A history that stays fast

| Feature | Stories |
| --- | --- |
| A history kept fast | a step, an undo and a redo changing only the shapes they change, a step on 1 000 shapes costing what one on one shape does (9 ms today, about 2 ms aimed, AUD-043); the list of steps scrolled whole, only the rows in sight drawn (US-083's draft), if Question 3 places it here (2) |

### Snap, align and distribute

| Feature | Stories |
| --- | --- |
| Snap to objects | the targets, the boxes' corners, edges, centers and midpoints, and the points; the tolerance in screen pixels; a mark where it snapped (3) |
| Guides and equal gaps | temporary guides when a point lines up; a stop at an existing gap; an object winning over a guide, even farther (3) |
| Align and distribute | the six alignments; equal gaps across and down, on the boxes in the drawing's space (2) |

E19 kept whole: one subject, the placement aids, in 8 stories.

### Groups and the stacking order

| Feature | Stories |
| --- | --- |
| The shape tree in the model | groups and children in the model, the flat drawing migrated; a group drawn as a nested `g`; the stacking order kept in the tree (3) |
| Group and ungroup | grouping and ungrouping by the keys and buttons; a child picked inside its group; a group moved, deleted and resized proportionally (3) |
| Order and duplicates | brought forward, backward, to the front, to the back; a shape duplicated, its first move's offset remembered for the next copies (2) |

### Layout boxes

| Feature | Stories |
| --- | --- |
| A box placing its children by anchor | the box as a rectangle with its radii (Q25); its children placed by the 3 × 3 anchor, F07's reused; the name sourced (3) |
| A child's own anchor | a child overriding the anchor; leaving the stack completely (2) |
| Rows and columns | a stack across or down with its gap; padding; a child hidden leaving its place empty (3) |

### Boolean operations

| Feature | Stories |
| --- | --- |
| Union and difference | the boolean node in the model, its operands kept; the intersections of segments and arcs, robust; union and difference drawn (3) |
| Intersection and exclusion | intersection; exclusion (2) |
| Division and cut path | division into pieces; the outlines cut at their intersections; the operands edited afterwards (3) |

Curved sides (cubics, ADR-0046) as operands: §5.

### Shape builder and edge operations

| Feature | Stories |
| --- | --- |
| The shape builder | the regions built from the operands' edges; a region kept or merged, stored as inside A and outside B; the result edited afterwards (3) |
| Operations on edges | the contour cut between two points; two points joined, an edge deleted; a connection drawn over the shape, the shapes rebuilt from the pieces (3) |

### Colors and strokes of shapes

| Feature | Stories |
| --- | --- |
| A shape's fill color | the fill in the model with ADR-0056's check; F16's fields and picker in the panel (2) |
| The aligned stroke and its color | width and alignment on segments and arcs, by offset (ADR-0002), the contour's band reused or removed (the card of SP-032); its color; its width kept when the shape is stretched (3) |
| Joins and caps | miter, round and bevel with a miter limit; caps on open contours; the cases of `shapes.md` §6 (3) |
| Dashes | a pattern of integers; exact lengths along arcs (2) |

The strokes of E14's curves: §5.

### Text, a second pass

| Feature | Stories |
| --- | --- |
| The lines of a text | trailing empty lines taking no space; one line in a box when it is wide enough, a model field; a text with a negative word spacing picked and its box computed again (3) |
| Fewer text options | the direction removed; the underline's and the strike-through's colors reset by a button, or removed (2) |
| A turned text as a shape | a turned text resized and turned by the tools of epics 1 and 2; its box's handles (2) |

### Fonts for synoptic views

| Feature | Stories |
| --- | --- |
| The font list | a search over the installed fonts; the list saying which fonts can be used and exported; a font of the computer chosen and imported (3) |
| Fonts shipped | free fonts shipped and installed with the software, a curated set; a font that cannot be embedded forced after a confirmation or refused (ADR-0062); the fonts of no use refused (3) |

Font subsetting at export (Q42): §5.

### Symbols and their ports

| Feature | Stories |
| --- | --- |
| A symbol | a symbol as a document, a group with its integer origin; its saved position (Q30) (2) |
| Ports | a port's id, integer position and exit direction; shown, added and moved in the playground; snapping to ports (3) |
| Clickable and status zones | a clickable zone laid over the symbol; a shared status zone (2) |

### Animatable parts

| Feature | Stories |
| --- | --- |
| Parts declared animatable | a part marked animatable and the kinds it accepts; shown in the panel (2) |
| The six animations | color and color blinking; opacity, visibility and their blinking; partial fill and rotation, through the remapping (Q13) (3) |
| The animation preview | values given by hand in the editor; every animation played in the preview (2) |

### Business configuration

| Feature | Stories |
| --- | --- |
| Properties and interfaces | a property's name, type and default; an interface of properties; the Configurator's first screen (3) |
| Trees and their cascade | a configuration tree; a node bringing its ancestors' options (Q14); trees combined (3) |
| Rules driving animations | a rule's form; its evaluation, deterministic; a rule driving a part of a symbol (3) |
| Property groups and defaults | groups of properties; lines hidden by default (2) |

### Business symbols and libraries

| Feature | Stories |
| --- | --- |
| Business symbols | a symbol, its selected configuration and its business name; one symbol in two business symbols (2) |
| Business libraries | a project's library; two projects sharing symbols through two libraries (2) |
| Text styles | named text styles as local templates that forbid nothing; applied to a text (2) |

### Synoptic views

| Feature | Stories |
| --- | --- |
| Instances and quarter turns | an instance placed at an integer position; duplicated; turned by quarter turns about its origin; its ports turned with it (4) |
| Overrides, preview and pop-up lines | a default overridden per instance; the live preview of its animation; the pop-up's lines unchecked and checked again; a line hidden by default (4) |
| Texts in a view | a font embedded once for the view's symbols; a long text scrolled in its box on a touch screen (2) |

### Orthogonal pipes

| Feature | Stories |
| --- | --- |
| A pipe from port to port | a pipe created by two clicks on ports; its first orthogonal path; drawn and stored (3) |
| Routing around symbols | the routing that avoids the symbols; its cost measured on a reference view; re-routed when a symbol moves (3) |
| Manual adjustment | a segment dragged perpendicular to itself; its neighbors adapting; kept through later moves (3) |
| Bends and crossings | rounded bends by the corner radius; a crossing broken on the vertical line (2) |

### Static drawings

| Feature | Stories |
| --- | --- |
| The pen | a Figma-like pen placing points; a point dragged pulling its handles; a drawing's tools kept few (3) |
| A group saved as a drawing | a selected group saved as a drawing; placed in a view (2) |
| The drawing library | the library of drawings; a drawing reused in another view and another project (2) |

### Files in a working folder

| Feature | Stories |
| --- | --- |
| Point 1 alike in every shape | the list of what changes (saved shapes, guided steps, test values); the rectangle from its bottom-right corner; the round shape from its right point (RET-002) (3) |
| One file per kind | a format per kind of document, versioned; the round trip of every kind; the integer model kept exact (3) |
| The working folder | a folder opened; files saved one by one; files reloaded identically (3) |

### SVG imported from elsewhere

| Feature | Stories |
| --- | --- |
| A plain SVG read | paths, rectangles, circles and ellipses read as shapes; a warning past a million units (Q20); refused beyond a level of complexity (3) |
| Simplified and scaled down | the straight lines found and simplified; the drawing scaled down to a sensible size (2) |

### The contrast of texts

| Feature | Stories |
| --- | --- |
| A text's contrast, two methods | the WCAG 2 ratio and its levels; the APCA lightness contrast read by size and weight (2) |
| A verdict in words | the verdict for the text's size and weight; shown beside the text (2) |
| A minimum contrast at all times | the colors a text's animations give; the minimum checked over them; a text refused or flagged below it (3) |

### Settings and keys of one's own

| Feature | Stories |
| --- | --- |
| Settings of the view | the pan's speed by the trackpad and the wheel, the zoom's step and the wheel's factor tuned by hand; the zoom's direction (2) |
| Keys of one's own | a standard place for settings; every shortcut and modifier changed there, Figma's by default (2) |
| The expert menu | every option of the free editor in one menu; kept from one session to the next (2) |

### The layers panel

| Feature | Stories |
| --- | --- |
| The layers panel | the tree shown as a list and reordered by a drag; a layer renamed; hidden and locked (3) |

A single feature: the subject holds no more.

### The editor's interface designed

| Feature | Stories |
| --- | --- |
| A page for any window | the page fitting any window; a toolbar at the bottom as Figma's, the selector first, then each shape, then the multiple selection (3) |
| Messages that explain | a polygon converted to a free shape saying what happens; a **do not show again** box (2) |

The review of every button and option, the epic's subject, makes the rest of its features at its refinement: §5.

### Documents for newcomers

| Feature | Stories |
| --- | --- |
| The overview | the project in a few pages with its architecture summary; a getting-started guide (2) |
| User manuals | the Symbol Editor's; the Configurator's; the View Editor's (3) |
| The traceability page | the matrix of criteria, stories, tests and results generated (ADR-0027); linked from the overview (2) |

### Release checks

| Feature | Stories |
| --- | --- |
| End-to-end and regression suites | the end-to-end suite of each editor; the regression suite; both on each release (3) |
| Budgets and the public API | a size budget; the public API report (2) |

## 4. The map from the old epics

Every item of the inventory, by its number there, and the new epic that takes it. Dropped: none is dropped without a reason, and only one item is.

| Item | Was | Goes to |
| --- | --- | --- |
| E18.1, E18.10, E18.12 (stretch part), E18.13 | resize by the box, the reference point, a shape stretched from a fixed point | Resize shapes by their box; the strokes' width kept when stretched: Colors and strokes of shapes |
| E18.2, E18.7, E18.8 | turn about a chosen center, the rotation mode, one angle only | Turn shapes about a chosen center, The rotation mode and its center |
| E18.3, E18.9, E18.11 | handles on both boxes, centers on points, the straight box hidden | Turn shapes about a chosen center; the straight box's handles: Resize shapes by their box |
| E18.4, E18.12 | drag points and handles, points dragged together | Drag points and handles, Points dragged |
| E18.5, E18.15 | number fields that compute, `+100` in Width | Number fields that compute |
| E18.6 | a history step kept cheap | A history that stays fast |
| E18.14 | the box of several shapes' position shown | Resize shapes by their box, The reference point of x and y |
| E19.1 to E19.3 | snap, guides and gaps, align and distribute | Snap, align and distribute, unchanged |
| E13.1, E13.8 | the shape tree, a group resized proportionally | Groups and the stacking order |
| E13.2 to E13.7, E13.10 | the layout box, a child's anchor, stacks, the name, Q25 | Layout boxes |
| E13.9 | the layers panel | The layers panel |
| E03.1 to E03.3 | union, difference, intersection, exclusion, division, cut path | Boolean operations |
| E03.4, E03.5 | the shape builder, operations on edges | Shape builder and edge operations |
| E04.1 | a shape's fill color | Colors and strokes of shapes, A shape's fill color (Question 5) |
| E04.2, E04.3, E04.5 | the stroke, its color, joins, caps, dashes, the contour's band | Colors and strokes of shapes |
| E04.4 | the strokes of the curves | Colors and strokes of shapes, unsized (§5) |
| E05.1, E05.4 to E05.7 | ports, the clickable and status zones, the saved position, snapping to ports | Symbols and their ports |
| E05.2, E05.3 | animatable parts, the preview | Animatable parts |
| E06.1 to E06.5 | the configuration trees, interfaces, rules, groups, defaults | Business configuration |
| E07.1, E07.2, E07.3 | business symbols and libraries, text styles | Business symbols and libraries |
| E08.1 to E08.6 | instances, quarter turns, overrides, pop-up lines, a font per view, a text scrolled | Synoptic views |
| E09.1 to E09.4 | pipes | Orthogonal pipes |
| E10.1 to E10.3 | the pen, a group saved as a drawing, the drawing library | Static drawings |
| E10.4 | point 1 alike in every shape | Files in a working folder, its first feature (Question 6) |
| E11.1, E11.2, E11.4, E11.5 | the formats, the working folder, local only, Q30 | Files in a working folder; Q30 asked with Symbols and their ports, settled for the files |
| E11.3 | SVG imported from elsewhere | SVG imported from elsewhere |
| E11.6 | Q23, the application shell | not placed: Question 9 |
| E20.1, E20.13, E20.14 | fonts found and shipped, a curated set, a font that cannot be embedded | Fonts for synoptic views |
| E20.2, E20.3, E20.4, E20.11 | fewer text options, the lines of a text, a turned text, the negative word spacing | Text, a second pass (Question 7) |
| E20.5, E20.8 | messages that explain, a page for any window | The editor's interface designed |
| E20.6, E20.9, E20.15 | keys of one's own, the view's settings and the expert menu, the zoom's direction | Settings and keys of one's own |
| E20.7 | the history's list scrolled whole | A history that stays fast, or kept with the interface (Question 3) |
| E20.10 | a letter refused as it is typed | Number fields that compute (Question 4) |
| E20.12, E20.16 | the review of every button and option, the selection rectangle that touches | The editor's interface designed |
| E12.1 to E12.4, E12.9 | the overview, the architecture, the manuals, the getting started, the traceability page | Documents for newcomers |
| E12.6 to E12.8 | the suites, the size budget, the API report | Release checks |
| E12.5 | npm publication | **dropped**: the library is never published on npm (Q40, 2026-10-17, `editions.md` §1); E12 was drafted before |
| E12.10 | a playground asking its own test questions | not placed: it stays a row of `improvements.md`, kept until a feature needs it (RET-004) |
| E16.1 to E16.3 | the contrast of texts | The contrast of texts, if it is V1 (Question 8) |
| D.1 | duplicating a shape | Groups and the stacking order, Order and duplicates |
| D.2 | the flags of degenerate shapes | Drag points and handles, Degenerate shapes flagged |
| D.3 | Q26, the radii of removed corners | not placed: Question 10 |
| D.4 | Q42, font subsetting | Fonts for synoptic views, unsized (§5) |
| D.5 | Q24, a `foreignObject` tool | not placed: an open question of the domain, its edition first (unchanged) |
| D.6 | Q43, free positions | not placed: for later by the domain's own words, once the integers' limits are known |
| D.7 | the helps still in the drawing's SVG | Resize shapes by their box, The helps in the layer above (Question 2) |

## 5. What could not be sized

Each is an open point for the epic's refinement, which reads its sources (ADR-0100, decision 2); no research was done here.

| Epic | What is not known | Why it cannot be judged now |
| --- | --- | --- |
| Boolean operations | booleans on curved sides (cubics, ADR-0046, ADR-0048), and whether the stories of robust geometry fit 25 minutes | the domain names an extension of the sweep to arcs (`shapes.md` §5) and nothing for cubics; the intersection of two cubics has no exact closed form, so the choice (subdivision, an approximation, curves refused as operands) is a source question and perhaps an ADR |
| Colors and strokes of shapes | the strokes of curves (Q4 open) | the offset of an arc is an arc (`shapes.md` §6), the offset of a cubic is not a cubic: an approximation and its tolerance need a source, perhaps an ADR beside ADR-0001 |
| Animatable parts | how a synoptic plays its animations outside the editor | the domain says what a part does, not what runs it in a view at run time (a player of the library, CSS, or the host's code); its size depends on the answer (Question 11) |
| Business configuration | the form of a rule | `configuration.md` §1 gives one example, a condition and two colors; a form of conditions and an expression language differ by a feature or more (Question 11) |
| Orthogonal pipes | the routing's size | `REF-WYBROW-2009`, `REF-MARRIOTT-2014` and `REF-LIBAVOID` are read only at its refinement; an incremental router may outgrow 3 stories |
| Files in a working folder | the working folder's reach | a browser opens a folder only in Chromium-based browsers (to source); a desktop shell (Q23) would be an epic of its own, with an ADR (Question 9) |
| The contrast of texts | its sources and its edition | WCAG 2.2's criteria 1.4.3 and 1.4.6 and APCA are not read yet (E16, Notes for later) |
| Fonts for synoptic views | font subsetting (Q42) | which texts are fixed at export is a business question still open |
| The editor's interface designed | its features beyond the two known | they come from the review of every button and option, its own subject, made at its refinement |

## 6. Questions for the Product Owner

Each with its options and an example; the agent's proposal is the first option, never a choice made. The order of the epics comes last.

1. **How small an epic is.** (a) The 28 epics of §2, the smallest one subject allows, from 1 to 4 features. (b) Pairs that share a subject's family merged: resize and turn, booleans and the shape builder, symbols and animations, configuration and business libraries, the two release epics, the settings and the layers panel: 22 epics. (c) Only the epics of over 9 stories cut, the rest as the old list. Example: (a) gives Resize shapes by their box, 7 stories, 5.5 to 7 hours, its questions asked at one spike; (b) gives Resize and turn, 6 features of 14 stories, 640 to 820 minutes (10.7 to 13.7 hours), every question of both at one spike, one return later.
2. **The helps still in the drawing's SVG.** (a) The first feature of Resize shapes by their box, since every later handle is drawn in the layer above. (b) An epic of their own, first, of one feature. (c) With the interface designed, the helps staying fills until then (ADR-0099, decision 2). Example: 2 stories, 95 to 125 minutes as a feature in (a), 115 to 145 with its own spike in (b).
3. **The history's list scrolled whole.** It was placed with the interface designed last on 2026-10-23 (SP-031's answer 1). (a) Kept there, A history that stays fast holding its one step story only. (b) Moved beside the cheap step, both about a history that grows. Example: a history of 1 000 000 steps, every row reached by the scroll, 15 rows drawn; (b) adds 25 minutes to epic 5 and takes them from epic 26.
4. **A letter refused as it is typed in a number field.** (a) With Number fields that compute, which rewrites how every field reads its text. (b) With the interface designed, as it was given. Example: typing `1a` in Width shows `1` in (a) from epic 4, and is refused once applied until epic 26 in (b).
5. **The shapes' fill color.** (a) The first feature of Colors and strokes of shapes, epic 11. (b) An epic of its own, after the groups, so that the shapes take their colors sooner. Example: (b) is 2 stories, 115 to 145 minutes with its spike; every example of the playground until then stays the page's blue `#4a7bd0`.
6. **Point 1 alike in every shape.** (a) The first feature of Files in a working folder, before the format is fixed, as RET-002 placed it. (b) Now, before epic 1, its cost growing with each epic's test values and guided steps. Example: the rectangle's point 1 goes from its top-left corner to its bottom-right one; in (a) every test written by epics 1 to 20 that names a rectangle's point is changed with it.
7. **The text's second pass and the fonts.** (a) Epics 12 and 13, among the library's epics, after the turn. (b) Just before the interface designed, as their ideas were placed in E20. Example: the direction removed and trailing empty lines taking no space arrive at epic 12 in (a), at about epic 25 in (b).
8. **The contrast of texts.** (a) In V1, epic 23, after the views and the animations whose colors it checks. (b) In the pro edition, kept in `editions.md`. (c) In V1 without the minimum at all times, which goes pro. Example: a bold 16 px text `#9a9a9a` on a white `#ffffff` background, blinking to `#ffffff` by an animation: its WCAG 2 ratio, its APCA reading and a verdict shown in the free editor in (a) and (c), the blinking's minimum checked in (a) only; all of it in the pro editor alone in (b).
9. **The application shell (Q23) and the working folder.** (a) The browser's own access to a folder in Files in a working folder, the shell at the release. (b) A desktop shell as an epic of its own before the files, with its ADR. (c) The shell left out of V1, the editor a page. Example: in (a) a folder opens in Chrome and Edge, not in Firefox nor Safari (to source); in (b) everywhere, one more epic and a new tool in the build.
10. **The radii of removed corners (Q26).** (a) A story of Drag points and handles, the radii kept while the session lasts. (b) The same, and kept in the saved file, with the files. (c) Not built, the corners coming back sharp as F03 decided. Example: a polygon of 12 corners of radii 1 to 12 made a triangle, then 12 again: corners 4 to 12 get radii 4 to 12 back in (a) and (b), radius 0 in (c).
11. **How a rule and an animation run.** For the sizes of Animatable parts and Business configuration. (a) A rule as a form of conditions, a property compared to a value giving a part's state, played by the library in the view. (b) An expression language, written as text. (c) Settled at the epics' refinements, from their sources. Example: `isRunning` true makes the body green, otherwise grey: one line of a form in (a), the text `isRunning ? green : grey` in (b).
12. **The order of the epics.** (a) The order of §2: the shapes edited by hand (1 to 6), the tree and the operations (7 to 11), the text (12, 13), the symbols to the drawings (14 to 20), the files (21, 22), the contrast (23), the settings and panels (24, 25), the interface (26), the release (27, 28). (b) As (a), the colors and strokes moved before the booleans (epic 11 after epic 8), the shapes' paint being seen in every later try. (c) As (a), the files moved right after the symbols (21 and 22 after 14), so that a symbol is saved early, the format growing with each later kind of document. Example: in (c), a symbol is saved at epic 16 of 28 instead of 21; in (b) a stroke is seen from epic 9 instead of 11.

### The answers

Given by the Product Owner on 2026-10-25; questions 2, 3, 4, 5, 7 and 11 placed by the agent the same day, within those answers.

1. (a) The 28 small epics.
2. (a) The helps of the drawing's SVG are the first feature of Resize shapes by their box (the agent's).
3. (a) The history's list scrolled whole stays with the interface designed, as decided on 2026-10-23 (the agent's): A history that stays fast holds its one step story.
4. (a) A letter refused as it is typed goes with Number fields that compute (the agent's).
5. (a) The fill color is the first feature of Colors and strokes of shapes (the agent's).
6. (a) Point 1 alike in every shape is the first feature of the files' epic.
7. (a) The text's second pass and the fonts are epics 12 and 13 (the agent's).
8. (b) The contrast of texts belongs entirely to the pro edition: The contrast of texts leaves the list, its three features written in `editions.md` §2.
9. Neither option as written: the editor works first as a web page, in every browser. A file is opened from the disk and saved by Save as, a download, which stands in for the working folder; Ctrl + S saves in place where the browser allows it (the File System Access API, to source at that epic's refinement), otherwise it opens Save as; a file is saved only when the designer asks, never continuously. The desktop application is an epic of its own, placed just before the release's epics, likely a Rust program hosting a web view, how being researched at its refinement; the working folder moves to it. Files in a working folder becomes Open and save files from the web page. Q23 is settled so.
10. (a) The radii of removed corners are kept while the session lasts, a story of Drag points and handles. Q26 is settled so.
11. (c) Settled at the refinements of Animatable parts and Business configuration, from their sources (the agent's).
12. (a) The order of §2, the contrast out and the desktop application in: still 28 epics.

## 7. The list as decided

The epics of §2 as the answers leave them, each given its id, E21 to E48 in order, and a title of a verb and its result as every epic's (the subject's name of §2 kept in its file); each epic's file in `docs/backlog/epics/` holds its features as drafts, the items of §4 it takes and its open points of §5. The changes from §2: A history that stays fast holds 1 story, the list scrolled whole going to the interface designed as a feature of 1 story; Drag points and handles takes the radii of removed corners into its second feature; the files' third feature opens and saves from the web page; The contrast of texts leaves; The desktop application comes in before the release.

| Order | ID | Epic | Features | Stories | Duration |
| --- | --- | --- | --- | --- | --- |
| 1 | E21 | Resize shapes by their box | 3 | 7 | 5.5 to 7 h |
| 2 | E22 | Turn shapes about a chosen center | 3 | 7 | 5.5 to 7 h |
| 3 | E23 | Drag points and handles | 2 | 6 | 4.3 to 5.3 h |
| 4 | E24 | Compute in number fields | 1 | 3 | 2.3 to 2.8 h |
| 5 | E25 | Keep the history fast | 1 | 1 | 1.5 to 2 h |
| 6 | E26 | Snap, align and distribute shapes | 3 | 8 | 5.9 to 7.4 h |
| 7 | E27 | Group shapes and order their stacking | 3 | 8 | 5.9 to 7.4 h |
| 8 | E28 | Lay out shapes in boxes | 3 | 8 | 5.9 to 7.4 h |
| 9 | E29 | Combine shapes by boolean operations | 3 | 8 | 5.9 to 7.4 h, and §5 |
| 10 | E30 | Rebuild shapes from their pieces | 2 | 6 | 4.3 to 5.3 h |
| 11 | E31 | Give shapes colors and strokes | 4 | 10 | 7.5 to 9.5 h, and §5 |
| 12 | E32 | Refine the text's lines and options | 3 | 7 | 5.5 to 7 h |
| 13 | E33 | Choose fonts for synoptic views | 2 | 6 | 4.3 to 5.3 h |
| 14 | E34 | Build symbols with ports | 3 | 7 | 5.5 to 7 h |
| 15 | E35 | Animate parts of symbols | 3 | 7 | 5.5 to 7 h, and §5 |
| 16 | E36 | Configure business types | 4 | 11 | 7.9 to 9.9 h, and §5 |
| 17 | E37 | Publish business symbols and libraries | 3 | 6 | 5.1 to 6.6 h |
| 18 | E38 | Assemble synoptic views | 3 | 10 | 6.8 to 8.3 h |
| 19 | E39 | Connect symbols with orthogonal pipes | 4 | 11 | 7.9 to 9.9 h, and §5 |
| 20 | E40 | Draw and reuse static drawings | 3 | 7 | 5.5 to 7 h |
| 21 | E41 | Open and save files from the web page | 3 | 9 | 6.3 to 7.8 h, and §5 |
| 22 | E42 | Import SVG from other tools | 2 | 5 | 3.9 to 4.9 h |
| 23 | E43 | Tune settings and keys | 3 | 6 | 5.1 to 6.6 h |
| 24 | E44 | Edit the shape tree in a layers panel | 1 | 3 | 2.3 to 2.8 h |
| 25 | E45 | Design the editor's interface | 3 | 6 | 5.1 to 6.6 h, and §5 |
| 26 | E46 | Run the editor as a desktop application | 2 | 5 | 3.9 to 4.9 h, and §5 |
| 27 | E47 | Explain the project to newcomers | 3 | 7 | 5.5 to 7 h |
| 28 | E48 | Check each release | 2 | 5 | 3.9 to 4.9 h |

In all: 28 epics, 75 features, 190 stories, 8 685 to 10 935 active minutes (145 to 182 hours), before the open points of §5. Example, E25: 20 + 25 × 1 + (45 to 75) × 1 = 90 to 120 minutes; E46: 20 + 25 × 5 + (45 to 75) × 2 = 235 to 295 minutes.

The epics not done before (E03 to E13, E16, E18 to E20) are `dropped`, each naming the new epics that took its content.
