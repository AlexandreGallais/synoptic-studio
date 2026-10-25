import { commonPanelValues, formatSvgNumber } from "../../src";
import { readKeptDrawing, readSelectedShapes } from "../kept-shapes";
import { selectElement } from "../page";
import { readPanelValues } from "../picking";

import { SEVERAL_FIELDS } from "./several-fields";

import type { PanelValues } from "../../src";

/** A value of several shapes as its field shows it. */
type ShownValue = {
  /** The field's placeholder: `Mixed` where the shapes differ, nothing otherwise. */
  readonly placeholder: string;
  /** The field's text: the common value, or nothing where the shapes differ. */
  readonly text: string;
};

/**
 * Reads the values of each shape selected (`readPanelValues`), in the order drawn.
 *
 * @kind procedure
 * @param document - page document
 * @returns their values, a text never measured left out
 * @see RULE-interaction-2-selector
 */
function readSelectedValues(document: Document): readonly PanelValues[] {
  const drawing = readKeptDrawing(document);

  return readSelectedShapes(document).flatMap((index) => {
    const kept = drawing[index];

    return kept === undefined ? [] : readPanelValues(document, kept.shape);
  });
}

/**
 * Creates what the field of a value of several shapes shows: the value with the SVG's decimals
 * (`formatSvgNumber`), as one shape's shows; or an empty field and its placeholder.
 *
 * @kind procedure
 * @param value - the value common to the shapes, `undefined` where they differ
 * @param mixed - the placeholder where they differ: `Mixed`, or nothing with no value to show
 * @returns the field's text and placeholder
 * @see RULE-interaction-2-selector
 */
function createShownValue(value: number | undefined, mixed: string): ShownValue {
  return value === undefined
    ? { placeholder: mixed, text: "" }
    : { placeholder: "", text: formatSvgNumber(value) };
}

/**
 * Writes a value of several shapes in its field.
 *
 * @kind procedure
 * @param document - page document
 * @param id - `several-x`, `several-y`, `several-width` or `several-height`
 * @param shown - its text and placeholder (`createShownValue`)
 * @see RULE-interaction-2-selector
 */
function writeSeveralField(document: Document, id: string, shown: ShownValue): void {
  const field = selectElement(document, id);

  if (!(field instanceof HTMLInputElement)) {
    return;
  }

  field.value = shown.text;
  field.placeholder = shown.placeholder;
}

/**
 * Shows the position and size of several shapes selected in the panel's four fields of several
 * shapes: each shape's values (`readSelectedValues`), a text never measured, which is not
 * drawn, left out; a value common to them as one shape's shows it, an empty field reading Mixed
 * where they differ (`commonPanelValues`, `createShownValue`), nor any placeholder with no value left; typed in,
 * they set the shapes (`listenToSeveralFields`).
 *
 * @kind procedure
 * @param document - page document
 * @see RULE-interaction-2-selector
 */
export function writeSeveralValues(document: Document): void {
  const values = readSelectedValues(document);
  const common = commonPanelValues(values);
  const mixed = values.length > 0 ? "Mixed" : "";

  for (const [key, id] of SEVERAL_FIELDS) {
    writeSeveralField(document, id, createShownValue(common[key], mixed));
  }
}
