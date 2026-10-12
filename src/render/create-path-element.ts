import { SVG_NAMESPACE } from "./svg-namespace";

/**
 * Creates a `<path>` element drawing the given path data.
 *
 * @kind procedure
 * @param document - document owning the element
 * @param pathData - value of the `d` attribute
 * @returns the detached `<path>` element
 * @see REF-MDN-CREATEELEMENTNS
 */
export function createPathElement(document: Document, pathData: string): SVGPathElement {
  const path = document.createElementNS(SVG_NAMESPACE, "path");

  path.setAttribute("d", pathData);

  return path;
}
