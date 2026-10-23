import { DEFAULT_HISTORY_BOUND, formatByteSize, historySize } from "../../src";
import { selectElement, writeText } from "../page";

import { PAGE_HISTORIES } from "./page-histories";

/**
 * Shows the page's history in the toolbar (F40): the Undo button disabled while nothing is past,
 * the Redo button while nothing is to come, and the size the history holds beside the size set,
 * `History 1.2 kB of 100 MB` (`historySize`, `formatByteSize`, SP-029's answer 8); none yet, as
 * an empty history.
 *
 * @kind procedure
 * @param document - page document
 * @see docs/domain/interaction.md#_7-navigation-and-history
 */
export function showHistory(document: Document): void {
  const { future, past, present } = PAGE_HISTORIES.get(document) ?? {
    future: [],
    past: [],
    present: { drawing: [], selection: [] },
  };

  const size = formatByteSize(historySize({ future, past, present }));
  const bound = formatByteSize(DEFAULT_HISTORY_BOUND.size);

  selectElement(document, "history-undo").toggleAttribute("disabled", past.length === 0);
  selectElement(document, "history-redo").toggleAttribute("disabled", future.length === 0);
  writeText(document, "history-size", `History ${size} of ${bound}`);
}
