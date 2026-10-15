import { isHtmlLayerDrawn } from "../model";

import { createHtmlLayerTextElement } from "./create-html-layer-text-element";
import { createTextElement } from "./create-text-element";

import type { DocumentOptions, TextWithHtmlLayer } from "../model";

/**
 * Creates a text as its document writes it (ADR-0053, decision 1): with its HTML layer when the
 * document's option is on and the layer is drawn, wider and taller than 0 (`isHtmlLayerDrawn`,
 * `createHtmlLayerTextElement`); otherwise exactly as before, its group alone
 * (`createTextElement`), the layer unused.
 *
 * @kind procedure
 * @param document - document owning the elements
 * @param options - the document's options, `hasHtmlLayer` among them
 * @param drawing - the text, its drawing and its HTML layer
 * @returns the detached `<g>`
 * @see REF-SVG2-STRUCT
 */
export function createDocumentTextElement(
  document: Document,
  options: DocumentOptions,
  drawing: TextWithHtmlLayer,
): SVGGElement {
  return options.hasHtmlLayer && isHtmlLayerDrawn(drawing.layer)
    ? createHtmlLayerTextElement(document, drawing)
    : createTextElement(document, drawing.text, drawing.drawn);
}
