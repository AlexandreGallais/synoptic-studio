import { readGuidedStep, readStepFromAddress, showGuidedStep } from "../guided-test";
import { listenToHover } from "../hover";
import { selectElement } from "../page";
import { listenToColorPicker } from "../picker-controls";
import { scheduleRedraw } from "../redraw";
import { listenToDrawing } from "../shape-drawing";
import { listenToTextFields } from "../text";

import { listenToInputs } from "./listen-to-inputs";
import { updatePage } from "./update-page";

import type { GuidedStep } from "../guided-steps";

/**
 * Wires the Previous and Next buttons of the guided test.
 *
 * @kind procedure
 * @param document - page document
 * @param steps - steps of the guided test
 * @param update - redraws the page from the values typed
 * @see docs/backlog/stories/E01-F01-VAL-001-validate-rectangle.md
 */
function listenToGuide(document: Document, steps: readonly GuidedStep[], update: () => void): void {
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

/**
 * Mounts the playground: redraws on every input and window resize, at most once per animation
 * frame, the inputs before it gathered (`scheduleRedraw`, US-048), wires the toolbar and the guided
 * test, then shows the step asked by the page address, or the first one.
 *
 * @kind procedure
 * @param document - page document
 * @param steps - steps of the guided test, possibly none
 * @see docs/backlog/stories/E01-F01-US-002-sharp-rectangle-in-playground.md
 */
export function mountPlayground(document: Document, steps: readonly GuidedStep[]): void {
  /** Asks a redraw at the next animation frame, the requests before it gathered (US-048). */
  function update(): void {
    scheduleRedraw(document, () => {
      updatePage(document);
    });
  }

  listenToInputs(document, update);
  listenToTextFields(document, update);
  listenToColorPicker(document, update);
  listenToDrawing(document, update);
  listenToHover(document);

  listenToGuide(document, steps, update);

  showGuidedStep(document, steps, readStepFromAddress(document));

  updatePage(document);
}
