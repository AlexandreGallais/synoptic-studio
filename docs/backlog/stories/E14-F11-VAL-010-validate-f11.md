---
id: VAL-010
epic: E14
feature: F11
title: Validate F11
status: in-progress
points: 1
---

# E14 · F11 · VAL-010 — Validate F11

Show the Product Owner, on one demo page, the figures before and after F11: no guided test, since no drawing changes (decision of 2026-10-13).

## Acceptance criteria

- Given the conventions page of the folders, when the Product Owner reads it, then the layout and its reasons are clear to them.
- Given the demo page, when the Product Owner reads it, then it shows before and after: the largest folder (77 modules in `playground/`), the functions without a property test (9), the repeated test helpers, the path writers (2), and `npm run check:all` green with the same tests and values.

## Tasks

One task = one commit, referenced as `VAL-010.Tn`. This story is not merged automatically (ADR-0028).

- [x] T1 — One test per criterion `[F11.AC1]`…`[F11.AC5]`; AC6 is `npm run check:all` (1 h)
- [x] T2 — Demo page of the figures, the layout page linked (1 h)
- [ ] T3 — The Product Owner reads the page and checks the editor; the result recorded

## Product Owner test

No guided test: no drawing changes. Read the demo page (`npm run docs:dev`, then Guide › F11), then in the editor:

1. Open `src/`: each layer holds only folders named after a domain (`math/` holds `vector/`, `angle/`, `bisection/`, `precision/`).
2. Create an empty file `src/math/test.ts`: the editor underlines it, the folder holds domain folders. Delete it.
3. Optional: `npm run dev`, a few steps of `http://localhost:5173/?guide=epic`: the shapes are drawn as before.
