# Where it stands

The library draws shapes from numbers — rectangles, regular polygons, free shapes, round shapes and curves —, places text with real font metrics, and lets the playground select, move, zoom and undo on one drawing. The editors themselves come later. The [order of the epics](./backlog/) says what comes next; the [playground](https://alexandregallais.github.io/synoptic-studio/playground/) shows the library at work.

## What was done

| Epic | What it delivered |
| --- | --- |
| E01 — Draw symbol shapes from numbers | rectangles and regular polygons with a radius per corner, a polygon anchored in its box or stretched to fill it, free shapes whose points are moved, added and deleted, any shape turned by any angle |
| E14 — Draw curves with handles, ellipses and round shapes | points with handles bending their sides, round shapes and ellipses as four cubic sides, every calculation named after what it computes |
| E15 — Place text in symbols | text measured from the font's own metrics, text in a box with wrapping and ellipsis, colors and their picker, fonts read from files or installed, fonts embedded at export |
| E17 — Fast tooling with Oxc and Rust | the checks on oxlint, TypeScript 7 and Rust, every project tool rewritten in Rust, the release by a button, a CI in parallel jobs |
| E02 — Select and move shapes on one drawing | several shapes on one drawing, picking by the painted area, selection by click and rectangle, a panel for several shapes, moves on integers, undo and redo with the list of steps, zoom and pan |

## Under way

| Epic | What it brings |
| --- | --- |
| E49 — Keep the agent's knowledge in a database, the docs light | the agent's records in a database outside the repository, and these short pages |
