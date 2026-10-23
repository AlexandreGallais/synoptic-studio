import type { PanDrag } from "./pan-drag";

/** The drag panning the view, by the page's document, from its press to the click after its release. */
export const PAN_DRAGS = new WeakMap<Document, PanDrag>();
