import { pointUnderPointer } from "editor";

import { readArmedTool } from "../toolbar";
import { POINTER_OFFSETS, readView } from "../view";

import type { Point } from "editor";

/**
 * Reads where on the canvas the pointer stands for an outline (`POINTER_OFFSETS`): with the selector
 * armed only, and none while the outline may not be drawn.
 *
 * @kind procedure
 * @param document - page document
 * @param isShown - whether the outline may be drawn
 * @returns its offset from the canvas's top-left, in CSS pixels; undefined for none
 */
function readHoverOffset(document: Document, isShown: boolean): Point | undefined {
  return isShown && readArmedTool(document) === "selector"
    ? POINTER_OFFSETS.get(document)
    : undefined;
}

/**
 * Reads the point whose shape is outlined as under the pointer: its offset on the canvas
 * (`POINTER_OFFSETS`) through the view as it is now (`pointUnderPointer`), so that a zoom or a
 * pan outlines the shape under the pointer still; with the selector armed only, a click then
 * picking a shape, and none while a selection rectangle is drawn.
 *
 * @kind procedure
 * @param document - page document
 * @param isShown - whether the outline may be drawn: false while a selection rectangle is drawn or
 * shapes are moved
 * @returns the point; undefined for none
 */
export function readHoverPoint(document: Document, isShown: boolean): Point | undefined {
  const offset = readHoverOffset(document, isShown);

  return offset === undefined ? undefined : pointUnderPointer(readView(document), offset);
}
