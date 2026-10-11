import { NO_VERTEX_SELECTION } from "../src";

import { GUIDED_STEPS } from "./guided-steps";
import { selectElement } from "./select-element";
import { writeAnchorValue } from "./write-anchor-value";
import { writeInputValue } from "./write-input-value";
import { writeSelection } from "./write-selection";
import { writeStepToAddress } from "./write-step-to-address";
import { writeText } from "./write-text";

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
function writeStepValues(document: Document, step: GuidedStep): void {
  // A step types one radius for every vertex (F03.AC2), and selects no vertex.
  delete selectElement(document, "radius").dataset["radii"];
  writeSelection(document, NO_VERTEX_SELECTION);
  writeShapeValue(document, step.values.shape);
  writeAnchorValue(document, step.values.anchor);
  writeInputValue(document, "corners", step.values.corners);
  writeInputValue(document, "width", step.values.width);
  writeInputValue(document, "height", step.values.height);
  writeInputValue(document, "radius", step.values.radius);
}

/**
 * Shows one step of the guided test, types its values, so that the drawing follows (VAL-001,
 * VAL-002), and writes its number into the page address (VAL-003).
 *
 * @kind procedure
 * @param document - page document
 * @param index - step to show, kept within the first and the last step
 * @param update - redraws the page from the values typed
 * @see docs/backlog/stories/E01-F01-VAL-001-validate-rectangle.md
 */
export function showGuidedStep(document: Document, index: number, update: () => void): void {
  const position = Math.min(Math.max(index, 0), GUIDED_STEPS.length - 1);
  const step = GUIDED_STEPS.at(position) ?? GUIDED_STEPS[0];

  if (step === undefined) {
    return;
  }

  selectElement(document, "guide").dataset["step"] = String(position);
  writeText(document, "guide-number", String(position + 1));
  writeText(document, "guide-count", String(GUIDED_STEPS.length));
  writeText(document, "guide-title", step.title);
  writeText(document, "guide-explanation", step.explanation);
  writeText(document, "guide-look", step.look);
  writeStepToAddress(document, position);

  writeStepValues(document, step);

  update();
}
