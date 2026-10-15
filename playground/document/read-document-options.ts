import { selectElement } from "../page";

import type { DocumentOptions } from "../../src";

/**
 * Reads the options of the drawing, which stands for a document: its options apply to all its
 * texts (ADR-0053, ADR-0060); Embed fonts, the only one, off as the page opens.
 *
 * @kind procedure
 * @param document - page document
 * @returns whether its fonts are embedded
 * @see docs/backlog/stories/E15-F15-US-036-fonts-embedding-and-html-layer.md
 */
export function readDocumentOptions(document: Document): DocumentOptions {
  const embed = selectElement(document, "embed-fonts");

  return {
    hasEmbeddedFonts: embed instanceof HTMLInputElement && embed.checked,
  };
}
