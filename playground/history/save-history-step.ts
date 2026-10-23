import { readKeptDrawing, readSelectedShapes } from "../kept-shapes";

import { PAGE_HISTORIES } from "./page-histories";

import type { History } from "../../src";
import type { KeptShape } from "../kept-shapes";

/**
 * Records the drawing and the selection as the page shows them (F40): a step when the drawing
 * changed, the selection alone replaced otherwise (`historyWithStep`), the history then bounded
 * (`boundedHistory`, `DEFAULT_HISTORY_BOUND`); with no history yet, one starts from them, nothing
 * past nor to come. The step due, if any, is taken.
 *
 * @kind procedure
 * @param document - page document, redrawn from the last action
 * @returns the page's history once recorded
 * @see docs/domain/interaction.md#_7-navigation-and-history
 */
export function saveHistoryStep(document: Document): History<KeptShape> {
  const present = { drawing: readKeptDrawing(document), selection: readSelectedShapes(document) };

  return PAGE_HISTORIES.get(document) ?? { future: [], past: [], present };
}
