import type { Point } from "../../src";

/**
 * The point of the drawing the pointer is over, by the page's document (F36), in the coordinates
 * of its SVG; none once it leaves the drawing.
 */
export const HOVER_POINTS = new WeakMap<Document, Point>();
