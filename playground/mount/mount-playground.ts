import { listenToHistory, updateHistory } from "../history";
import { listenToHover } from "../hover";
import { removeListeners } from "../listeners";
import { listenToColorPicker } from "../picker-controls";
import { listenToRedrawBeforeClicks, scheduleRedraw } from "../redraw";
import { listenToDrawing } from "../shape-drawing";

import { listenToInputs } from "./listen-to-inputs";
import { updatePage } from "./update-page";

/**
 * Mounts the playground, the listeners a mount before this one put on the document and its window
 * removed first (`removeListeners`), so that each event is heard once: redraws on every
 * input and change of the canvas's size, at most once per animation frame, the inputs before it gathered
 * (`scheduleRedraw`), each redraw bringing the history of the drawing up to the page
 * (`updateHistory`), a redraw asked drawn before a release or a click on the drawing acts,
 * heard first, the redraw the mount before asked and did not draw forgotten
 * (`listenToRedrawBeforeClicks`); wires the toolbar and, after it, the history
 * (`listenToHistory`).
 *
 * @kind procedure
 * @param document - page document
 */
export function mountPlayground(document: Document): void {
  /** Asks a redraw at the next animation frame, the requests before it gathered. */
  function update(): void {
    scheduleRedraw(document, () => {
      updatePage(document);
      updateHistory(document);
    });
  }

  removeListeners(document);
  listenToRedrawBeforeClicks(document);
  listenToInputs(document, update);
  listenToColorPicker(document, update);
  listenToDrawing(document, update);
  listenToHover(document);
  listenToHistory(document, update);

  updatePage(document);
  updateHistory(document);
}
