import { formatByteSize, movesWithinSize } from "../../src";
import { readPageHistoryBound } from "../history-bound";
import { showHistoryList } from "../history-list";
import { selectElement, writeText } from "../page";

import { PAGE_HISTORIES } from "./page-histories";

/**
 * Shows the page's history in the toolbar: the Undo button disabled while nothing is past,
 * the Redo button while nothing is to come, the size the history holds, kept with it (ADR-0097),
 * beside the size set, `History 1.2 kB of 100 MB` (`formatByteSize`, SP-029's answer 8), and about
 * how many moves of one shape the size set holds, `about 80,380 moves` (`movesWithinSize`,
 * SP-029's answer 6); none yet, as an empty history; then the list of its steps
 * (`showHistoryList`).
 *
 * @kind procedure
 * @param document - page document
 * @see RULE-interaction-7-navigation-and-history
 */
export function showHistory(document: Document): void {
  const history = PAGE_HISTORIES.get(document);
  const {
    future,
    past,
    size: held,
  } = history ?? {
    future: [],
    past: [],
    size: 0,
  };
  const { size } = readPageHistoryBound(document);
  const moves = movesWithinSize(size).toLocaleString("en-US");

  selectElement(document, "history-undo").toggleAttribute("disabled", past.length === 0);
  selectElement(document, "history-redo").toggleAttribute("disabled", future.length === 0);
  writeText(document, "history-size", `History ${formatByteSize(held)} of ${formatByteSize(size)}`);
  writeText(document, "history-moves", `about ${moves} moves`);
  showHistoryList(document, history);
}
