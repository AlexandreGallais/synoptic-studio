import { parsePanelValue } from "../../src";
import { SHOWN_TEXTS, resetShownText } from "../applied-values";

import { writeOwnValue } from "./write-own-value";

/**
 * Stops a text that is no number typed in a field of its own, the text it showed before put back
 * (`resetShownText`), a Mixed radius's empty one included, so that no listener of the field
 * applies it; a field whose text the page never read, typed by a script without the focus nor a
 * key, lets it go to them, which read it as they read a number field's.
 *
 * @kind procedure
 * @param event - a `change` heard on the document before the field
 * @param field - a field acting as a number field of its own
 * @see RULE-interaction-2-selector
 */
function stopRefusedText(event: Event, field: HTMLInputElement): void {
  if (!SHOWN_TEXTS.has(field)) {
    return;
  }

  resetShownText(field.ownerDocument, field.id);
  event.stopPropagation();
}

/**
 * Applies the text typed in a field of its own (US-087, AUD-046): a number, relative with a sign
 * (`parsePanelValue`), shown as the value it gives within the field's range (`writeOwnValue`),
 * which the field's own listeners then read; a text that is no number refused
 * (`stopRefusedText`).
 *
 * @kind procedure
 * @param event - a `change` heard on the document before the field
 * @param field - a field acting as a number field of its own
 * @see RULE-interaction-2-selector
 */
export function applyOwnText(event: Event, field: HTMLInputElement): void {
  const typed = parsePanelValue(field.value);

  if (typed === undefined) {
    stopRefusedText(event, field);

    return;
  }

  writeOwnValue(field, typed);
}
