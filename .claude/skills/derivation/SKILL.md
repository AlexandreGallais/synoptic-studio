---
name: derivation
description: Write a DERIV-* derivation when no single readable source states a formula. Use when a math or geometry function lacks a verifiable reference.
argument-hint: "[topic, e.g. fillet-setback]"
---

# Write a derivation

Topic: $ARGUMENTS. Model: `knowledge show DERIV-fillet-setback`.

1. **State** the result and its domain (inputs, units, sign conventions of the project: y down, clockwise contours).
2. **Steps** (ask the `math-expert` agent for a second derivation when a step is not obvious): each step cites a read source (a `REF-*` of the records) or follows from the previous steps by named algebra. No step "by intuition".
3. **Cross-check** with an independent source or field, marked as a cross-check, not as the proof.
4. **Check table**: inputs and expected outputs computed by hand, including degenerate cases (0, aligned, extreme). The tests reuse this table literally.
5. **Record**: `DERIV-<topic>` in the records (`knowledge add derivation <file>`); the code cites it with `@see DERIV-<topic>`.
