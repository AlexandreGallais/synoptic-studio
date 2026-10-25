import { NEW_TEXTS } from "../new-text";
import { selectElement } from "../page";
import { writeTextSections } from "../text";

/**
 * Shows the Text tool's fields of a new text and gives its Content field the focus, so that the
 * designer clicks, then types (US-075); the text is then being typed (`NEW_TEXTS`), its placing
 * and what is typed in it one step (SP-029's answer 5).
 *
 * @kind procedure
 * @param document - page document
 * @see RULE-interaction-1-tools
 */
export function focusNewText(document: Document): void {
  const content = selectElement(document, "text-content");

  NEW_TEXTS.add(document);
  // Shown before the redraw, so that the content can take the focus.
  writeTextSections(document, true);
  content.focus();
}
