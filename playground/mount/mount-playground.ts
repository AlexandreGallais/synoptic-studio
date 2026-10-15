import { readGuidedStep, readStepFromAddress, showGuidedStep } from "../guided-test";
import { readShapeKind, selectElement } from "../page";
import { listenToColorPicker } from "../picker-controls";
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
 * Mounts the playground: redraws on every input and window resize, wires the toolbar and the
 * guided test, then shows the step asked by the page address, or the first one.
 *
 * @kind procedure
 * @param document - page document
 * @param steps - steps of the guided test, possibly none
 * @see docs/backlog/stories/E01-F01-US-002-sharp-rectangle-in-playground.md
 */
export function mountPlayground(document: Document, steps: readonly GuidedStep[]): void {
  /** Redraws the page from the inputs. */
  function update(): void {
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

  listenToInputs(document, update);
  listenToTextFields(document, update);
  listenToColorPicker(document, update);

  listenToGuide(document, steps, update);

  showGuidedStep(document, steps, readStepFromAddress(document));

  update();
}
