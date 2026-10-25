import { formatSvgNumber, isFreeShape } from "../../src";
import { readMode, selectElement } from "../page";

import { showPointButtons } from "./show-point-buttons";

import type { Point, VertexSelection } from "../../src";
import type { ShapeReading } from "../shape";

/** What a field shows and whether it can be typed in. */
type FieldState = {
  /** Whether the field is greyed. */
  readonly disabled: boolean;
  /** Text shown in the field. */
  readonly text: string;
};

/**
 * Writes the text and the state of an input of the page.
 *
 * @kind procedure
 * @param document - page document
 * @param id - id of the input
 * @param state - text shown and whether it is greyed
 */
function writeFieldState(document: Document, id: string, state: FieldState): void {
  const field = selectElement(document, id);

  if (!(field instanceof HTMLInputElement)) {
    return;
  }

  field.value = state.text;
  field.disabled = state.disabled;
}

/**
 * Reads the point selected when exactly one is.
 *
 * @kind procedure
 * @param reading - valid shape
 * @param selection - vertices selected
 * @returns the point as drawn, `undefined` when none or several are selected
 */
function readSelectedPoint(reading: ShapeReading, selection: VertexSelection): Point | undefined {
  const [only] = selection.selected;

  return selection.selected.length === 1
    ? reading.corners.find((corner, vertex) => vertex === only)?.point
    : undefined;
}

/**
 * Writes the x and y fields for the point selected: its coordinates, typed for a free shape,
 * greyed for a regular polygon; empty and greyed when not exactly one point is selected.
 *
 * @kind procedure
 * @param document - page document
 * @param point - point selected, `undefined` when not exactly one is
 * @param isTyped - whether its coordinates can be typed
 */
function writePointFields(document: Document, point: Point | undefined, isTyped: boolean): void {
  if (point === undefined) {
    writeFieldState(document, "point-x", { disabled: true, text: "" });
    writeFieldState(document, "point-y", { disabled: true, text: "" });

    return;
  }

  writeFieldState(document, "point-x", { disabled: !isTyped, text: formatSvgNumber(point.x) });
  writeFieldState(document, "point-y", { disabled: !isTyped, text: formatSvgNumber(point.y) });
}

/**
 * Shows the x and y of the one point selected, typed for a free shape (the rectangle included)
 * and greyed for a regular polygon, which is converted first, with the buttons acting on the
 * points (F08.AC1, F08.AC2, F08.AC3).
 *
 * @kind procedure
 * @param document - page document
 * @param reading - valid shape
 * @param selection - vertices selected
 */
export function showPointFields(
  document: Document,
  reading: ShapeReading,
  selection: VertexSelection,
): void {
  selectElement(document, "point-fields").hidden = readMode(document) !== "points";
  writePointFields(document, readSelectedPoint(reading, selection), isFreeShape(reading.model));
  showPointButtons(document, reading, selection);
}
