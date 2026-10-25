import { clamp } from "../../src";
import { selectElement, writeText } from "../page";

import { writeStepToAddress } from "./write-step-to-address";
import { writeStepValues } from "./write-step-values";

import type { GuidedStep } from "../guided-steps";

/**
 * Shows one step of the guided test and types its values, so that the drawing follows once the page
 * redraws; writes its number into the page address. A test without
 * steps is not shown.
 *
 * @kind procedure
 * @param document - page document
 * @param steps - steps of the guided test, possibly none
 * @param index - step to show, kept within the first and the last step
 */
export function showGuidedStep(
  document: Document,
  steps: readonly GuidedStep[],
  index: number,
): void {
  const position = clamp(index, 0, steps.length - 1);
  const step = steps.at(position) ?? steps[0];

  selectElement(document, "guide").hidden = step === undefined;

  if (step === undefined) {
    return;
  }

  selectElement(document, "guide").dataset["step"] = String(position);
  writeText(document, "guide-number", String(position + 1));
  writeText(document, "guide-count", String(steps.length));
  writeText(document, "guide-title", step.title);
  writeText(document, "guide-explanation", step.explanation);
  writeText(document, "guide-look", step.look);
  writeStepToAddress(document, position);
  writeStepValues(document, step);
}
