import { formatRotation } from "../io";

import { createHtmlLayerElement } from "./create-html-layer-element";
import { createTextElement } from "./create-text-element";
import { SVG_NAMESPACE } from "./svg-namespace";

import type { TextWithHtmlLayer } from "../model";

/**
 * Creates a text with its HTML layer (ADR-0053, decision 1): an outer `<g>` carrying the text's
 * turn, as `createTextElement`'s group does, and holding one `<switch>`, which draws its first
 * child whose conditions hold (`REF-SVG2-STRUCT`, §5.6.3): first the HTML layer, which only a
 * browser draws (`createHtmlLayerElement`), then the text's group as `createTextElement` writes
 * it, with the same content and no `transform`, which every other reader draws. Both children
 * turn alike with the outer group.
 *
 * @kind procedure
 * @param document - document owning the elements
 * @param drawing - the text, its drawing and its HTML layer
 * @returns the detached outer `<g>`
 * @see REF-SVG2-STRUCT
 */
export function createHtmlLayerTextElement(
  document: Document,
  drawing: TextWithHtmlLayer,
): SVGGElement {
  const { drawn, layer, text } = drawing;
  const outer = document.createElementNS(SVG_NAMESPACE, "g");
  const choice = document.createElementNS(SVG_NAMESPACE, "switch");

  if (text.angle !== 0) {
    outer.setAttribute("transform", formatRotation(text.angle, drawn.center));
  }

  choice.append(
    createHtmlLayerElement(document, text, layer),
    createTextElement(document, { ...text, angle: 0 }, drawn),
  );

  outer.append(choice);

  return outer;
}
