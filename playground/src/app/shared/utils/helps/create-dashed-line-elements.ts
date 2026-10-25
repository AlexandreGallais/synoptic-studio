import { createPathElement, formatSvgNumber, screenToUser } from "editor";

import { readView } from "../view";

import type { DashedLine } from "./dashed-line";

/** The length of a dash and of the gap after it, in screen pixels, as Inkscape's (4 on, 4 off). */
const DASH = 4;

/**
 * Creates a help of the layer above the drawing as one line of white and black dashes
 * (ADR-0099): the path it follows stroked white, of class `<name>-light`, then stroked black over
 * it, dashed 4 and 4 screen pixels from the path's start, of class `<name>`, so that its dashes
 * alternate black and white, as Inkscape draws a control's dashes again in a second color in the
 * gaps of the first (`REF-INKSCAPE-CANVAS-ITEM-RECT`); each line's width and dashes divided by the
 * view's scale (`screenToUser`), so that they keep their size on screen; `fill: none` and the
 * colors in the page's style.
 *
 * @kind procedure
 * @param document - page document
 * @param line - the path it follows, its class and its width in screen pixels
 * @returns the white line's `<path>`, then the dashes'
 */
export function createDashedLineElements(
  document: Document,
  line: DashedLine,
): readonly SVGPathElement[] {
  const { scale } = readView(document);
  const width = formatSvgNumber(screenToUser(line.width, scale));
  const dash = formatSvgNumber(screenToUser(DASH, scale));
  const light = createPathElement(document, line.data);
  const dashes = createPathElement(document, line.data);

  light.setAttribute("stroke-width", width);
  light.classList.add(`${line.name}-light`);
  dashes.setAttribute("stroke-width", width);
  dashes.setAttribute("stroke-dasharray", `${dash} ${dash}`);
  dashes.classList.add(line.name);

  return [light, dashes];
}
