import { SHOWN_TEXTS } from "./shown-texts";
import { writeFieldText } from "./write-field-text";

/**
 * Puts back in a field acting as a number field the text it showed before a text that is no
 * number was typed in it (`SHOWN_TEXTS`, AUD-046), a Mixed radius's empty one included; a field
 * whose text the page never read keeps its own.
 *
 * @kind procedure
 * @param document - page document
 * @param id - the field's, without `#`
 * @see RULE-interaction-2-selector
 */
export function resetShownText(document: Document, id: string): void {
  const input = document.querySelector(`#${id}`);

  if (input instanceof HTMLInputElement) {
    writeFieldText(document, id, SHOWN_TEXTS.get(input) ?? input.value);
  }
}
