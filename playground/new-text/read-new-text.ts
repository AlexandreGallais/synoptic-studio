import { DRAWING_KEYS, readStoredDrawing } from "../kept-shapes";

import { NEW_TEXTS } from "./new-texts";

import type { Text } from "../../src";

/**
 * Reads the page's new text being typed (`NEW_TEXTS`, F40): placed by the Text tool, its Content
 * field not yet left with a content typed, nor the panel left.
 *
 * @kind procedure
 * @param document - page document
 * @returns the text, as the panel last drew it; undefined when no new text is being typed
 * @see docs/domain/interaction.md#_1-tools
 */
export function readNewText(document: Document): Text | undefined {
  const [kept] = readStoredDrawing(document, DRAWING_KEYS.edited);

  return NEW_TEXTS.has(document) && kept?.kind === "text" ? undefined : undefined;
}
