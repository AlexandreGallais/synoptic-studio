import type { Point } from "../../src";

/**
 * Where the pointer is over the drawing, by the page's document (F36): its offset from the canvas's
 * top-left, in CSS pixels, so that the shape under it is read again through the view once the view
 * changes, the pointer still (F42); none once it leaves the drawing.
 */
export const HOVER_OFFSETS = new WeakMap<Document, Point>();
