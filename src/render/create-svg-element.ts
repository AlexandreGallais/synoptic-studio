import { formatSvgNumber } from "../io";

import { SVG_NAMESPACE } from "./svg-namespace";

import type { ViewBox } from "../model";

/**
 * Creates an empty `<svg>` canvas showing the given region.
 *
 * @kind procedure
 * @param document - document owning the element
 * @param viewBox - region shown, written with `formatSvgNumber`
 * @returns the detached `<svg>` element
 */
export function createSvgElement(document: Document, viewBox: ViewBox): SVGSVGElement {
  const svg = document.createElementNS(SVG_NAMESPACE, "svg");
  const values = [viewBox.x, viewBox.y, viewBox.width, viewBox.height].map((value) =>
    formatSvgNumber(value),
  );

  svg.setAttribute("viewBox", values.join(" "));

  return svg;
}
