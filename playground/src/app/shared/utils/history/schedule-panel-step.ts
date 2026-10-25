import { PENDING_REDRAWS } from "../redraw";

import { DUE_HISTORY_STEPS } from "./due-history-steps";
import { updateHistory } from "./update-history";

/**
 * Asks a step for a change of the panel that just finished, without a redraw of its own:
 * the field's or the button's listener redrew the page, or asked it, from what it changed, and a
 * value it refused is left as typed, a redraw writing it back otherwise; the step is taken by the
 * redraw asked (`PENDING_REDRAWS`), or at once from the page as last drawn (`updateHistory`).
 *
 * @kind procedure
 * @param document - page document
 */
export function schedulePanelStep(document: Document): void {
  DUE_HISTORY_STEPS.add(document);

  if (!PENDING_REDRAWS.has(document)) {
    updateHistory(document);
  }
}
