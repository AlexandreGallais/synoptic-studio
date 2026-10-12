---
name: derivation
description: Write a DERIV-* derivation when no single readable source states a formula. Use when a math or geometry function lacks a verifiable reference.
argument-hint: "[topic, e.g. fillet-setback]"
---

# Write a derivation

Topic: $ARGUMENTS. Model: `docs/derivations/fillet-setback.md`; index: `docs/derivations/README.md`.

1. **State** the result and its domain (inputs, units, sign conventions of the project: y down, clockwise contours, Q11).
2. **Steps** (ask the `math-expert` agent for a second derivation when a step is not obvious): each step cites a read source (`REF-*` already in `docs/references.md`) or follows from the previous steps by named algebra. No step "by intuition".
3. **Cross-check** with an independent source or field (a different derivation, an engineering formula), marked as a cross-check, not as the proof.
4. **Check table**: inputs and expected outputs computed by hand, including degenerate cases (0, aligned, extreme). The tests reuse this table literally.
5. **Record**: `DERIV-<topic>` in the derivations index; the code cites it with `@see DERIV-<topic>`; an `[unverified]` reference it replaces stays uncited, with a note.
6. Commit scope `docs(geometry)`.
