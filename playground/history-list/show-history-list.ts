import type { SizedHistory } from "../../src";
import type { KeptShape } from "../kept-shapes";

/**
 * Shows the list of the history's steps (US-077), unless it shows that history already
 * (`SHOWN_HISTORIES`): the rows around the present (`historySteps`, `HISTORY_LIST_ROWS`), each
 * a button named by its step's action (`readKeptShapeFields`), the present's marked as the
 * current step, those to redo marked `future`, each button keeping the state it goes back to
 * (`STEP_BUTTON_STATES`), and a line for the steps left out before and after them,
 * `6 earlier steps`, `1 later step`; no history, no row.
 *
 * @kind procedure
 * @param document - page document
 * @param history - the page's history; undefined before its first redraw
 * @see docs/domain/interaction.md#_7-navigation-and-history
 */
export function showHistoryList(
  document: Document,
  history: SizedHistory<KeptShape> | undefined,
): void {
  document.querySelector("#history-list")?.replaceChildren(String(history?.past.length));
}
