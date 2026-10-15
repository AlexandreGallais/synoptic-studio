import type { HtmlLayer } from "./html-layer";

/**
 * Whether a text's HTML layer is drawn at all: "A value of zero for either width or height
 * disables rendering of the element" (`REF-SVG2-EMBEDDED`, §12.2), so a layer 0 wide or 0 tall —
 * a box 80 × 0 or 0 × 40, an empty text — is not written and the text's group stands alone, as
 * with the layer off.
 *
 * @kind domain
 * @param layer - its frame
 * @returns true when its frame is wider and taller than 0
 * @see REF-SVG2-EMBEDDED
 */
export function isHtmlLayerDrawn(layer: HtmlLayer): boolean {
  return layer.frame.width > 0 && layer.frame.height > 0;
}
