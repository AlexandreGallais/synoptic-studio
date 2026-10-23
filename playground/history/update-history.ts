import { NEW_TEXTS } from "../new-text";

import { DUE_HISTORY_STEPS } from "./due-history-steps";
import { PAGE_HISTORIES } from "./page-histories";
import { readHeldAction } from "./read-held-action";
import { saveHistoryStep } from "./save-history-step";
import { showHistory } from "./show-history";

/**
 * Brings the page's history up to the page once it is redrawn (F40): the step due taken, or the
 * history started when there is none (`saveHistoryStep`), unless a press still holds its action
 * (`readHeldAction`), whose release takes the step, or a new text is being typed (`NEW_TEXTS`),
 * its Content field left or the panel leaving it taking the step; then the history shown in the toolbar
 * (`showHistory`).
 *
 * @kind procedure
 * @param document - page document, just redrawn
 * @see docs/domain/interaction.md#_7-navigation-and-history
 */
export function updateHistory(document: Document): void {
  const isDue = [DUE_HISTORY_STEPS.has(document), !PAGE_HISTORIES.has(document)].includes(true);

  const isHeld = [readHeldAction(document) !== undefined, NEW_TEXTS.has(document)].includes(true);

  if (isDue && !isHeld) {
    saveHistoryStep(document);
  }

  showHistory(document);
}
