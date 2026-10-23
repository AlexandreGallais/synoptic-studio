# Research notes

Summaries of research requests (`../requests/`), of spikes and of the agent's own research, numbered in order; a note answering a request takes the request's number.
Content: summary in our own words (no copy of source text), formulas, references with `REF-*` IDs, impact on code and docs.

| Note | Topic |
| --- | --- |
| [0001](./0001-tooling-practices.md) | Tooling practices: function size, decorators, Prettier, barrels, exports, default parameters, docs site |
| [0002](./0002-agent-and-verification-practices.md) | Working with a coding agent (skills, hooks, subagents, context), property-based and mutation testing, link checking |
| [0003](./0003-f01-remaining-stories.md) | Sources of the remaining F01 stories: radius clamp, fillet arc, SVG arc command |
| [0004](./0004-f02-regular-polygon.md) | Sources of F02: regular polygon fit, starting vertex, maximal rounding, polygons in process symbols (Q19) |
| [0005](./0005-f07-polygon-anchor.md) | Sources of F07: nine alignments of SVG `preserveAspectRatio`, anchored edges, names, anchor picker |
| [0006](./0006-f03-radius-per-vertex.md) | Sources of F03: clamping with unequal radii, Mixed value, node tool, selection in a list and on a drawing |
| [0007](./0007-f08-node-editing.md) | Sources of F08: shape vs path, conversion, midpoint, deleting points, coordinates |
| [0008](./0008-f09-stretchable-free-shape.md) | Sources of F09: a scale about the top-left of the box, without loss, the rectangle tool |
| [0009](./0009-f04-free-rotation.md) | Sources of F04: turning about the center of the box, angle from 0 to 359, exact quarter turns, position, straight box of arcs |
| [0010](./0010-f10-round-shape.md) | Sources of F10: the capsule (stadium), its conversion, the model of the round shape; F10 as built in E01, cancelled and kept for E14 |
| [0011](./0011-f06-stretched-polygon.md) | Sources of F06: a polygon stretched to its box, sides as even as possible, corners kept; rules A, B, C |
| [0012](./0012-f11-folders-and-tests.md) | Sources of F11: tests beside their module, folders by theme, a limit of files per folder, a shape's type in its folder, shared types at the root of a layer |
| [0013](./0013-f12-curves-with-handles.md) | Sources of F12: ways to type a handle, a point with a radius and handles, cubic sides |
| [0014](./0014-f12-fillet-beside-curve.md) | Sources of F12: a corner fillet beside a curved side, a radius too large |
| [0015](./0015-f10-ellipse-from-cubics.md) | Sources of F10 in E14: an ellipse from four cubic sides, not elliptical arcs |
| [0016](./0016-f13-named-calculations.md) | Sources of F13: the calculations of the code that mathematics names, and those left inline |
| [0017](./0017-e15-text.md) | Sources of E15: text in SVG readers, font files and their metrics, measuring, scripts and wrapping, embedding, the default font |
| [0018](./0018-f14-text-box.md) | Sources of F14: wrapping at spaces, long words, the lines shown and the ellipsis, line height, cap-height trim, vertical alignment, balanced wrap, background |
| [0019](./0019-f16-colors.md) | Sources of F16: hexadecimal forms and the alpha byte, HSL and HSB and their exact round trip, the picker's plane, sliders and keyboard |
| [0020](./0020-f15-fonts-and-html-layer.md) | Sources of F15: font files read by the page, their names and embedding permissions, fonts as `data:` URLs, the HTML layer in a `switch` and what each reader draws, the vertical metrics browsers use |
| [0021](./0021-f17-latin-script-and-defects.md) | Sources of F17: the scripts of industrial HMIs, installed faces from collections and their `TypeError`, a weight such as 450 drawn, the lag of a held key, a wrapped line's background band |
| [0022](./0022-f18-parallel-run.md) | Sources of F18: counting running subagents, the heavy lock from any worktree, tables without padding, logs merged by union, worktrees, timing tests, the release flow |
| [0023](./0023-f19-oxc-and-rust-tools.md) | Sources of F19: oxlint and oxfmt tried on the repository, the rules left to rewrite, `check:all` by lane, hooks timed, the Rust workspace |
| [0024](./0024-f20-context-diet.md) | Sources of F20: what a call costs, the prompt cache measured, workers from a digest, a model per agent, pushes, the progress page, the stop hook, a run's totals, shell or a tool of our own |
| [0025](./0025-f21-agent-tools-in-rust.md) | Sources of F21: the crates of the Rust tools and the library server, the behaviour and licence of each package rewritten, each tool's time before its port, good release notes |
| [0026](./0026-the-tooling-features-of-2026-10-18.md) | Sources of the tooling features of 2026-10-18: a workflow after each merge into `develop` though the auto-merge's token starts none, each workflow's use, the blocking CI measured |
| [0027](./0027-f35-what-a-new-shape-starts-with.md) | Sources of F35: what Figma, Inkscape, Sketch, Affinity Designer and Penpot give a new shape and a new text, fixed defaults, the last values used or a default set by the designer, and the choice for F35 |
| [0028](./0028-f37-select-shapes.md) | Sources of F37: the keys of Figma, Inkscape, Penpot, Sketch, Affinity Designer and the systems for selecting, deleting, nudging and undoing, what a selection rectangle takes, the panel for several shapes, the code F37 touches and its stories' size |
| [0029](./0029-f39-move-shapes-on-integers.md) | Sources of F39: how Figma, Inkscape, Sketch, Penpot and Affinity Designer move a selection by the mouse and the arrows, a drag's threshold, which point lands on whole units, the band of a large rounded shape, the code F39 touches and its stories' size |
| [0030](./0030-f40-undo-and-redo.md) | Sources of F40: a history of commands or of states in Figma, Inkscape, Penpot, tldraw, GIMP and two text editors, steps grouped, structural sharing, a history's memory measured from its data in a browser, the code F40 touches and its stories' size |
| [0031](./0031-f42-zoom-and-pan.md) | Sources of F42: a view by the `viewBox` or by a CSS transform, the keys, wheels, steps and bounds of Figma, Inkscape, Penpot and Sketch, a fit to the drawing, the helps drawn within the area shown, the code F42 touches and its stories' size |
| [0032](./0032-f43-corrections-from-the-first-try.md) | Sources of F43: where the slow test of the history spends its time, how Inkscape, Penpot and Figma draw their outlines and show what a rectangle takes, the arrows in a field and two arrows held, `user-select`, Shift with the wheel, correction 10 reproduced, a virtual list and the tallest box, the stories' size |
