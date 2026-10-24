import type { ShapeDrawing } from "./shape-drawing";

/** The drag with a tool on, by the page's document (F36), from the press to the release. */
export const SHAPE_DRAWINGS = new WeakMap<Document, ShapeDrawing>();
