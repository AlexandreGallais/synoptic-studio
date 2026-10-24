import { listenToNumberField } from "../number-fields";

import { applyEditedValue } from "./apply-edited-value";
import { EDITED_FIELDS } from "./edited-fields";

/**
 * Makes the fields of the shape the panel edits, alone, act as number fields (US-085), as several
 * shapes' (`listenToNumberField`, `EDITED_FIELDS`): a value typed applied on Enter or when the
 * field is left, Up and Down stepping it by 1, or 10 with Shift, each set by the computation of
 * several shapes' (`applyEditedValue`); the page is then redrawn.
 *
 * @kind procedure
 * @param document - page document
 * @param update - redraws the page
 * @see docs/domain/interaction.md#_2-selector
 */
export function listenToEditedFields(document: Document, update: () => void): void {
  for (const [key, id] of EDITED_FIELDS) {
    listenToNumberField(document, id, (typed) => {
      applyEditedValue(document, key, typed);
      update();
    });
  }
}
