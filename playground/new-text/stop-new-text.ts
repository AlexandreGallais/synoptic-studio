import { NEW_TEXTS } from "./new-texts";

/**
 * Ends the typing of the page's new text (`NEW_TEXTS`, F40): a text still empty is removed
 * (`readNewText`), as a refused shape is not kept, so that the drawing is back as it was before
 * it was placed; a text typed stays, its step taken at the next one the page records.
 *
 * @kind procedure
 * @param document - page document
 * @see docs/domain/interaction.md#_1-tools
 */
export function stopNewText(document: Document): void {
  NEW_TEXTS.add(document);
}
