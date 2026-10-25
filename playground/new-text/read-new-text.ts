import { isDrawnText } from "../../src";
import { DRAWING_KEYS, readStoredDrawing } from "../kept-shapes";

import { NEW_TEXTS } from "./new-texts";

import type { Text } from "../../src";

/**
 * Reads the page's new text being typed (`NEW_TEXTS`): placed by the Text tool, its Content
 * field not yet left with a content typed, nor the panel left.
 *
 * @kind procedure
 * @param document - page document
 * @returns the text, as the panel last drew it; undefined when no new text is being typed, or
 * while it is refused
 * @see RULE-interaction-1-tools
 */
export function readNewText(document: Document): Text | undefined {
  const texts = readStoredDrawing(document, DRAWING_KEYS.edited)
    .map((kept) => kept.shape)
    .filter((shape) => isDrawnText(shape));
  const [text] = NEW_TEXTS.has(document) ? texts : [];

  return text;
}
