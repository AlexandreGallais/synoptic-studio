import { listenToGuide, readStepFromAddress, showGuidedStep } from "../guided-test";
import { listenToHistory, updateHistory } from "../history";
import { listenToHover } from "../hover";
import { removeListeners } from "../listeners";
import { listenToColorPicker } from "../picker-controls";
import { scheduleRedraw } from "../redraw";
import { listenToDrawing } from "../shape-drawing";

import { listenToInputs } from "./listen-to-inputs";
import { updatePage } from "./update-page";

import type { GuidedStep } from "../guided-steps";

/**
 * Mounts the playground, the listeners a mount before this one put on the document and its window
 * removed first (`removeListeners`, EN-150), so that each event is heard once: redraws on every
 * input and window resize, at most once per animation frame, the inputs before it gathered
 * (`scheduleRedraw`, US-048), each redraw bringing the history of the drawing up to the page
 * (`updateHistory`, F40); wires the toolbar, the guided test and, after them, the history
 * (`listenToHistory`), then shows the step asked by the page address, or the first one.
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
      updateHistory(document);
    });
  }

  removeListeners(document);
  listenToInputs(document, update);
  listenToColorPicker(document, update);
  listenToDrawing(document, update);
  listenToHover(document);
  listenToGuide(document, steps, update);
  listenToHistory(document, update);

  showGuidedStep(document, steps, readStepFromAddress(document));

  updatePage(document);
  updateHistory(document);
}
