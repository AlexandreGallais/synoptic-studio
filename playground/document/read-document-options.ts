import { selectElement } from "../page";

import type { DocumentOptions } from "../../src";

/**
 * Reads the options of the drawing, which stands for a document: its options apply to all its
 * texts (ADR-0053); both off as the page opens.
 *
 * @kind procedure
 * @param document - page document
 * @returns whether its fonts are embedded and whether its texts have their HTML layer
 * @see docs/backlog/stories/E15-F15-US-036-fonts-embedding-and-html-layer.md
 */
export function readDocumentOptions(document: Document): DocumentOptions {
  const embed = selectElement(document, "embed-fonts");
  const layer = selectElement(document, "html-layer");

  return {
    hasEmbeddedFonts: embed instanceof HTMLInputElement && embed.checked,
    hasHtmlLayer: layer instanceof HTMLInputElement && layer.checked,
  };
}
