import { formatByteSize, historySize, movesWithinSize } from "../../src";
import { readPageHistoryBound } from "../history-bound";
import { selectElement, writeText } from "../page";

import { PAGE_HISTORIES } from "./page-histories";

import type { History } from "../../src";
import type { KeptShape } from "../kept-shapes";

/**
 * The size of each history shown, by the history itself, so that a redraw that changed no step,
 * a move of the pointer, counts none again (`historySize` reads every shape held only by the past
 * and the states to come).
 */
const SHOWN_SIZES = new WeakMap<History<KeptShape>, number>();

/**
 * Reads the size of a history, counted once for each history (`SHOWN_SIZES`, `historySize`).
 *
 * @kind procedure
 * @param history - the page's history
 * @returns its size in bytes
 * @see docs/domain/interaction.md#_7-navigation-and-history
 */
function readShownSize(history: History<KeptShape>): number {
  const known = SHOWN_SIZES.get(history);

  if (known !== undefined) {
    return known;
  }

  const size = historySize(history);

  SHOWN_SIZES.set(history, size);

  return size;
}

/**
 * Shows the page's history in the toolbar (F40): the Undo button disabled while nothing is past,
 * the Redo button while nothing is to come, the size the history holds beside the size set,
 * `History 1.2 kB of 100 MB` (`readShownSize`, `formatByteSize`, SP-029's answer 8), and about how
 * many moves of one shape the size set holds, `about 80,380 moves` (`movesWithinSize`, SP-029's
 * answer 6); none yet, as an empty history.
 *
 * @kind procedure
 * @param document - page document
 * @see docs/domain/interaction.md#_7-navigation-and-history
 */
export function showHistory(document: Document): void {
  const history = PAGE_HISTORIES.get(document) ?? {
    future: [],
    past: [],
    present: { drawing: [], selection: [] },
  };
  const { size } = readPageHistoryBound(document);
  const used = formatByteSize(readShownSize(history));
  const moves = movesWithinSize(size).toLocaleString("en-US");

  selectElement(document, "history-undo").toggleAttribute("disabled", history.past.length === 0);
  selectElement(document, "history-redo").toggleAttribute("disabled", history.future.length === 0);
  writeText(document, "history-size", `History ${used} of ${formatByteSize(size)}`);
  writeText(document, "history-moves", `about ${moves} moves`);
}
