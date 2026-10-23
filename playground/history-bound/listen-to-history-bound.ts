import { readNumber, selectElement } from "../page";

import { applyHistoryBound } from "./apply-history-bound";
import { HISTORY_BOUND_FIELDS } from "./history-bound-fields";
import { PAGE_HISTORY_BOUNDS } from "./page-history-bounds";
import { readBoundPart } from "./read-bound-part";
import { readPageHistoryBound } from "./read-page-history-bound";
import { readStoredHistoryBound } from "./read-stored-history-bound";
import { writeBoundFields } from "./write-bound-fields";

import type { HistoryBound } from "../../src";

/**
 * Applies the value of a field of the bound (`applyHistoryBound`) when its field accepts it
 * (`readBoundPart`), then bounds the history by it; otherwise the fields show again the bound in
 * force.
 *
 * @kind procedure
 * @param document - page document
 * @param part - `size` or `steps`, the part the field sets
 * @param onBound - bounds the page's history by the bound just set
 * @see docs/domain/interaction.md#_7-navigation-and-history
 */
function applyTypedPart(document: Document, part: keyof HistoryBound, onBound: () => void): void {
  const field = HISTORY_BOUND_FIELDS[part];
  const typed = readBoundPart(readNumber(document, field.id) * field.unit, field);
  const bound = readPageHistoryBound(document);

  if (typed === undefined) {
    writeBoundFields(document, bound);

    return;
  }

  applyHistoryBound(document, { ...bound, [part]: typed });
  onBound();
}

/**
 * Listens to the fields of the history's bound (F40, `HISTORY_BOUND_FIELDS`), the bound kept in
 * the browser read first (`readStoredHistoryBound`) and shown (`writeBoundFields`): a value
 * applied, by Enter or by leaving its field (`change`), sets its part of the bound when its field
 * accepts it (`applyTypedPart`), then the history is bounded by it (`onBound`); otherwise the
 * field shows again the value in force (agent's choice).
 *
 * @kind procedure
 * @param document - page document
 * @param onBound - bounds the page's history by the bound just set
 * @see docs/domain/interaction.md#_7-navigation-and-history
 */
export function listenToHistoryBound(document: Document, onBound: () => void): void {
  const stored = readStoredHistoryBound(document);

  PAGE_HISTORY_BOUNDS.set(document, stored);
  writeBoundFields(document, stored);

  for (const part of ["size", "steps"] as const) {
    selectElement(document, HISTORY_BOUND_FIELDS[part].id).addEventListener("change", () => {
      applyTypedPart(document, part, onBound);
    });
  }
}
