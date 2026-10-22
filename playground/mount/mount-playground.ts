import { readGuidedStep, readStepFromAddress, showGuidedStep } from "../guided-test";
import { readShapeKind, selectElement } from "../page";
import { listenToColorPicker } from "../picker-controls";
import { listenToPlacing } from "../placing";
import { scheduleRedraw } from "../redraw";
import { hidePointPanels, listenToTextFields, updateTextTool, writeTextSections } from "../text";
import { writeShapeKind } from "../toolbar";

import { listenToInputs } from "./listen-to-inputs";
import { updatePlayground } from "./update-playground";

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
 * Redraws the page from its inputs: the Text tool, or the shapes.
 *
 * @kind procedure
 * @param document - the playground's document, its fields read
 * @see docs/backlog/stories/E01-F01-US-002-sharp-rectangle-in-playground.md
 */
function updatePage(document: Document): void {
  const isText = readShapeKind(document) === "text";

  writeTextSections(document, isText);

  if (isText) {
    writeShapeKind(document);
    hidePointPanels(document);
    updateTextTool(document);

    return;
  }

  updatePlayground(document);
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
  listenToPlacing(document, update);

  listenToGuide(document, steps, update);

  showGuidedStep(document, steps, readStepFromAddress(document));

  updatePage(document);
}
