/**
 * Where a state of the list stands against the present: `past`, a state undo goes back
 * to; `present`, the state the page shows; `future`, a state undone, that redo goes forward to.
 */
export type StepPlace = "future" | "past" | "present";
