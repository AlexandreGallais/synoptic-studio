import { NO_VERTEX_SELECTION } from "editor";

import { DRAWING_KEYS, writeStoredDrawing } from "../kept-shapes";
import { clearKindSettings } from "../kind-settings";
import { selectElement, writeInputValue, writeSelection } from "../page";

import { OPENING_TEXT } from "./opening-text";
import { writeAnchorValue } from "./write-anchor-value";
import { writeStretchedValue } from "./write-stretched-value";
import { writeTextValues } from "./write-text-values";

import type { ShapeValues } from "./shape-values";

/**
 * Sets the shape selector, without firing any event.
 *
 * @kind procedure
 * @param document - page document
 * @param shape - `rectangle`, `polygon`, `round` or `text`
 */
function writeShapeValue(document: Document, shape: string): void {
  const select = selectElement(document, "shape");

  if (select instanceof HTMLSelectElement) {
    select.value = shape;
  }
}

/**
 * Clears what the page kept of the shape drawn before, without firing any event: the radius of
 * each vertex and the free shape.
 *
 * @kind procedure
 * @param document - page document
 */
function writeKeptShapeCleared(document: Document): void {
  // One radius for every vertex (F03.AC2), and no vertex selected.
  delete selectElement(document, "radius").dataset["radii"];
  // One shape alone, as the page opens: the shapes placed before it go, below and
  // above, and the settings kept for each kind.
  writeStoredDrawing(document, DRAWING_KEYS.below, []);
  writeStoredDrawing(document, DRAWING_KEYS.above, []);
  clearKindSettings(document);
  // A rectangle or a polygon: the free shape kept goes.
  delete selectElement(document, "shape").dataset["freeShape"];

  selectElement(document, "shape")
    .querySelector('option[value="free"]')
    ?.setAttribute("hidden", "");
}

/**
 * Types the values of one shape into the inputs, without firing any event, the shapes placed and
 * the settings kept forgotten, and the page's text into the Text tool's fields.
 *
 * @kind procedure
 * @param document - page document
 * @param values - the shape and numbers typed
 */
export function writeShapeValues(document: Document, values: ShapeValues): void {
  writeKeptShapeCleared(document);
  writeSelection(document, NO_VERTEX_SELECTION);
  writeShapeValue(document, values.shape);
  writeAnchorValue(document, values.anchor);
  // Not stretched, as the page opens.
  writeStretchedValue(document, false);
  writeInputValue(document, "corners", values.corners);
  writeInputValue(document, "width", values.width);
  writeInputValue(document, "height", values.height);
  writeInputValue(document, "radius", values.radius);
  // A straight shape at (0, 0).
  delete selectElement(document, "shape").dataset["x"];
  delete selectElement(document, "shape").dataset["y"];

  // The page's text, so that the next text placed is the page's and not the last one's, its X,
  // Y and angle fields 0.
  writeTextValues(document, OPENING_TEXT);
}
