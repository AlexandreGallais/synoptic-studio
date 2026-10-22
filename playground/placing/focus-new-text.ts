import { selectElement } from "../page";
import { writeTextSections } from "../text";

/**
 * Shows the Text tool's fields of a new text and selects its content in the Content field, so
 * that typing replaces it (SP-026).
 *
 * @kind procedure
 * @param document - page document
 * @see docs/domain/interaction.md#_1-tools
 */
export function focusNewText(document: Document): void {
  const content = selectElement(document, "text-content");

  // Shown before the redraw, so that the content can take the focus.
  writeTextSections(document, true);

  if (!(content instanceof HTMLTextAreaElement)) {
    return;
  }

  content.focus();
  content.select();
}
