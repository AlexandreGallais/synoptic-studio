import { fieldValuesOfHandle, isAngleUnknown } from "../../src";
import { readMode, selectElement, writeInputValue } from "../page";

import { readHandleFrames } from "./read-handle-frames";
import { readSelectedCorner } from "./read-selected-corner";
import { readTypingWay } from "./read-typing-way";

import type { CurveCorner, HandleWay } from "../../src";
import type { ShapeReading } from "../shape";
import type { SelectedCorner } from "./read-selected-corner";

/** The two values of a handle to show, and whether its first field keeps what it shows. */
type ShownValues = {
  /** Whether the first field keeps its text: an angle typed before a length. */
  readonly isKept: boolean;
  /** The two values in the way chosen. */
  readonly values: readonly [number, number];
};

/** Names of the two values of a handle, by the way it is typed. */
const VALUE_NAMES: Readonly<Record<HandleWay, readonly [string, string]>> = {
  axis: ["angle", "length"],
  page: ["x", "y"],
  point: ["dx", "dy"],
  side: ["angle", "length"],
};

/**
 * Writes the name of each handle field's value: x and y, dx and dy, or angle and length.
 *
 * @kind procedure
 * @param document - page document
 * @param way - the way the handles are typed
 * @see docs/backlog/stories/E14-F12-US-024-handles-as-angle-and-length.md
 */
function writeAxisLabels(document: Document, way: HandleWay): void {
  for (const [index, label] of document.querySelectorAll("#handle-fields .handle-axis").entries()) {
    label.textContent = VALUE_NAMES[way][index % 2] ?? "";
  }
}

/**
 * Writes the two values of one handle; a handle of length 0 typed as an angle keeps the angle
 * typed before its length in its field (`isAngleUnknown`).
 *
 * @kind procedure
 * @param document - page document
 * @param end - which handle
 * @param shown - its two values in the way chosen, and whether the first field keeps what it shows
 * @see docs/backlog/stories/E14-F12-US-024-handles-as-angle-and-length.md
 */
function writeEndValues(document: Document, end: "after" | "before", shown: ShownValues): void {
  if (!shown.isKept) {
    writeInputValue(document, `handle-${end}-x`, shown.values[0]);
  }

  writeInputValue(document, `handle-${end}-y`, shown.values[1]);
}

/**
 * Writes the mode and the two values of each handle in the way chosen (`fieldValuesOfHandle`).
 *
 * @kind procedure
 * @param document - page document
 * @param reading - the shape read from the inputs, its corners as drawn
 * @param selected - the point selected, in handles mode
 * @see docs/backlog/stories/E14-F12-US-024-handles-as-angle-and-length.md
 */
function writeHandleValues(
  document: Document,
  reading: ShapeReading,
  selected: SelectedCorner,
): void {
  const mode = selectElement(document, "handle-mode");
  const way = readTypingWay(document);
  const frames = readHandleFrames(reading, selected);
  const { handles } = selected.corner;

  if (mode instanceof HTMLSelectElement) {
    mode.value = handles.mode;
  }

  writeAxisLabels(document, way);

  for (const end of ["before", "after"] as const) {
    writeEndValues(document, end, {
      isKept: isAngleUnknown(way, handles[end]),
      values: fieldValuesOfHandle(way, handles[end], frames[end]),
    });
  }
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
 * handles, then the mode and each handle's two values in the way chosen to type them (F12,
 * research note 0013; domain: radius and handles on one point).
 *
 * @kind procedure
 * @param document - page document
 * @param reading - the shape read from the inputs, its corners as drawn
 * @see docs/backlog/stories/E14-F12-US-022-handles-in-points-mode.md
 */
export function showHandleFields(document: Document, reading: ShapeReading): void {
  const selected =
    readMode(document) === "points" ? readSelectedCorner(document, reading) : undefined;

  showHandleSwitch(document, selected?.corner);

  if (selected?.corner.handles.isOn === true) {
    writeHandleValues(document, reading, selected);
  }
}
