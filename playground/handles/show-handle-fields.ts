import { add } from "../../src";
import { readMode, selectElement, writeInputValue } from "../page";

import { readFieldOrigin } from "./read-field-origin";
import { readSelectedCorner } from "./read-selected-corner";

import type { CurveCorner } from "../../src";
import type { ShapeReading } from "../shape";

/**
 * Writes the axis of each handle field: x and y on the page, dx and dy from the point.
 *
 * @kind procedure
 * @param document - page document
 * @param prefix - "d" for a step from the point, "" for a place on the page
 * @see docs/backlog/stories/E14-F12-US-023-handles-relative-to-point.md
 */
function writeAxisLabels(document: Document, prefix: "" | "d"): void {
  for (const label of document.querySelectorAll("#handle-fields .handle-axis")) {
    label.textContent = `${prefix}${label.textContent.replace("d", "")}`;
  }
}

/**
 * Writes the mode and each handle, as its end on the page (x, y) or as its vector from the point
 * (dx, dy), by the way chosen to type them.
 *
 * @kind procedure
 * @param document - page document
 * @param corner - the corner selected, as drawn, in handles mode
 * @see docs/backlog/stories/E14-F12-US-022-handles-in-points-mode.md
 */
function writeHandleValues(document: Document, corner: CurveCorner): void {
  const mode = selectElement(document, "handle-mode");
  const origin = readFieldOrigin(document, corner.point);
  const before = add(origin, corner.handles.before);
  const after = add(origin, corner.handles.after);

  if (mode instanceof HTMLSelectElement) {
    mode.value = corner.handles.mode;
  }

  writeAxisLabels(document, origin === corner.point ? "" : "d");
  writeInputValue(document, "handle-before-x", before.x);
  writeInputValue(document, "handle-before-y", before.y);
  writeInputValue(document, "handle-after-x", after.x);
  writeInputValue(document, "handle-after-y", after.y);
}

/**
 * Shows the switch between radius and handles, pressed in handles mode, and the fields of the
 * mode shown: the handles' fields, or the radius field (hidden, its value kept).
 *
 * @kind procedure
 * @param document - page document
 * @param corner - the corner selected, as drawn, or `undefined` for no switch
 * @see docs/backlog/stories/E14-F12-US-022-handles-in-points-mode.md
 */
function showHandleSwitch(document: Document, corner: CurveCorner | undefined): void {
  const isOn = corner?.handles.isOn === true;
  const toggle = selectElement(document, "handles-switch");

  toggle.hidden = corner === undefined;
  toggle.setAttribute("aria-pressed", String(isOn));
  selectElement(document, "handle-fields").hidden = !isOn;
  selectElement(document, "point-radius-field").hidden = isOn;
}

/**
 * Shows the handles of the one point selected in Points mode: the switch between radius and
 * handles, then the mode and the end of each handle as x and y on the page (F12, domain: radius
 * and handles on one point).
 *
 * @kind procedure
 * @param document - page document
 * @param reading - the shape read from the inputs, its corners as drawn
 * @see docs/backlog/stories/E14-F12-US-022-handles-in-points-mode.md
 */
export function showHandleFields(document: Document, reading: ShapeReading): void {
  const corner =
    readMode(document) === "points" ? readSelectedCorner(document, reading)?.corner : undefined;

  showHandleSwitch(document, corner);

  if (corner?.handles.isOn === true) {
    writeHandleValues(document, corner);
  }
}
