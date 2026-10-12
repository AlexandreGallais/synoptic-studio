import { NO_VERTEX_SELECTION } from "../src";

import { selectElement } from "./select-element";
import { writeAnchorValue } from "./write-anchor-value";
import { writeInputValue } from "./write-input-value";
import { writeSelection } from "./write-selection";
import { writeStretchedValue } from "./write-stretched-value";

import type { GuidedStep } from "./guided-step";

/**
 * Sets the shape selector, without firing any event.
 *
 * @kind procedure
 * @param document - page document
 * @param shape - `rectangle` or `polygon`
 * @see docs/backlog/stories/E01-F02-US-007-shape-selector-playground.md
 */
function writeShapeValue(document: Document, shape: string): void {
  const select = selectElement(document, "shape");

  if (select instanceof HTMLSelectElement) {
    select.value = shape;
  }
}

/**
 * Types the values of a step of the guided test into the inputs, without firing any event.
 *
 * @kind procedure
 * @param document - page document
 * @param step - step whose shape and numbers are typed
 * @see docs/backlog/stories/E01-F02-VAL-002-validate-polygon.md
 */
export function writeStepValues(document: Document, step: GuidedStep): void {
  // A step types one radius for every vertex (F03.AC2), and selects no vertex.
  delete selectElement(document, "radius").dataset["radii"];
  // A step draws a rectangle or a polygon: the free shape kept goes (F08).
  delete selectElement(document, "shape").dataset["freeShape"];

  selectElement(document, "shape")
    .querySelector('option[value="free"]')
    ?.setAttribute("hidden", "");

  writeSelection(document, NO_VERTEX_SELECTION);
  writeShapeValue(document, step.values.shape);
  writeAnchorValue(document, step.values.anchor);
  // The polygons of F02 to F08 are not stretched (F06).
  writeStretchedValue(document, step.values.stretched === true);
  writeInputValue(document, "corners", step.values.corners);
  writeInputValue(document, "width", step.values.width);
  writeInputValue(document, "height", step.values.height);
  writeInputValue(document, "radius", step.values.radius);
  // A step of F01 to F09 draws a straight shape at (0, 0) (F04).
  writeInputValue(document, "angle", 0);
  delete selectElement(document, "shape").dataset["x"];
  delete selectElement(document, "shape").dataset["y"];
}
