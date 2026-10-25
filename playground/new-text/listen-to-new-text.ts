import { selectElement } from "../page";

import { readNewText } from "./read-new-text";
import { stopNewText } from "./stop-new-text";

/**
 * Listens to the Content field of the page's new text (F40, SP-029's answer 5): left with a
 * content typed, the text's typing ends (`stopNewText`) and its step is asked, the text placed
 * and what was typed in it one step; left empty, the text stays being typed, so that its size or
 * its font may be set before its content, until the panel leaves it.
 *
 * @kind procedure
 * @param document - page document
 * @param step - asks the step of the text
 * @see RULE-interaction-1-tools
 */
export function listenToNewText(document: Document, step: () => void): void {
  selectElement(document, "text-content").addEventListener("focusout", () => {
    const text = readNewText(document);

    if (text === undefined || text.content === "") {
      return;
    }

    stopNewText(document);
    step();
  });
}
