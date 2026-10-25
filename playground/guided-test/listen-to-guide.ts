import { selectElement } from "../page";

import { readGuidedStep } from "./read-guided-step";
import { showGuidedStep } from "./show-guided-step";

import type { GuidedStep } from "../guided-steps";

/**
 * Wires the Previous and Next buttons of the guided test.
 *
 * @kind procedure
 * @param document - page document
 * @param steps - steps of the guided test
 * @param update - redraws the page from the values typed
 * @see VAL-001
 */
export function listenToGuide(
  document: Document,
  steps: readonly GuidedStep[],
  update: () => void,
): void {
  for (const [id, move] of [
    ["guide-previous", -1],
    ["guide-next", 1],
  ] as const) {
    selectElement(document, id).addEventListener("click", () => {
      showGuidedStep(document, steps, readGuidedStep(document) + move);
      update();
    });
  }
}
