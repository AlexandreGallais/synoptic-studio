import type { SelectionDrag } from "./selection-drag";

/** The press and drag with the Selector on, by the page's document (F37), from the press to the release. */
export const SELECTION_DRAGS = new WeakMap<Document, SelectionDrag>();
