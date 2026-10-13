import { NO_VERTEX_SELECTION } from "../../src";
import { selectElement, writeSelection } from "../page";

import { listenToToolbar } from "./listen-to-toolbar";

/** Buttons of the tool bar, one per mode. */
const MODES = ["mode-shape", "mode-points"];

/**
 * Presses the button of one mode and releases the others.
 *
 * @kind procedure
 * @param document - page document
 * @param pressed - id of the button pressed
 * @see docs/backlog/stories/E01-F03-US-012-points-mode.md
 */
function writePressedMode(document: Document, pressed: string): void {
  for (const id of MODES) {
    selectElement(document, id).setAttribute("aria-pressed", String(id === pressed));
  }
}

/**
 * Presses the mode button clicked and releases the other, then calls back (F03.AC4); wires the
 * shape buttons of the same toolbar.
 *
 * @kind procedure
 * @param document - page document
 * @param update - redraws the playground
 * @see docs/backlog/stories/E01-F03-US-012-points-mode.md
 */
export function listenToModes(document: Document, update: () => void): void {
  for (const id of MODES) {
    selectElement(document, id).addEventListener("click", () => {
      writePressedMode(document, id);
      // Back to Shape mode: no vertex stays selected (F03.AC5).
      writeSelection(document, NO_VERTEX_SELECTION);
      update();
    });
  }

  listenToToolbar(document);
}
