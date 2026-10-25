import type { WheelZoom } from "./wheel-zoom";

/**
 * The zoom the wheel and the pinch asked since the last frame, by the page's document: their
 * deltas gathered into one change of the view a frame (`REF-PENPOT-VIEWPORT-ACTIONS`).
 */
export const WHEEL_ZOOMS = new WeakMap<Document, WheelZoom>();
