/**
 * Where a state of the list stands against the present (US-077): `past`, a state undo goes back
 * to; `present`, the state the page shows; `future`, a state undone, that redo goes forward to.
 */
export type StepPlace = "future" | "past" | "present";
