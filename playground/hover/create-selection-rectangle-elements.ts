import {
  boxSize,
  createPathElement,
  formatPathData,
  formatSvgNumber,
  screenToUser,
} from "../../src";
import { VIEW_SCALE } from "../kept-shapes";

import { readSelectionBox } from "./read-selection-box";
import { SELECTION_DRAGS } from "./selection-drags";

import type { BoundingBox } from "../../src";

/** Width of the rectangle's outline, in screen pixels: a band of half of it on each side. */
const OUTLINE_WIDTH = 1;

/**
 * Path data of a box's sides moved inward by a distance, outward when it is negative: clockwise
 * on the screen outward, the other way inward, so that the nonzero rule leaves the inner one empty.
 *
 * @kind procedure
 * @param box - the rectangle, in user units
 * @param inset - how far each side moves inward, in user units
 * @returns the closed sub-path
 * @see docs/domain/interaction.md#_2-selector
 */
function readSidesPathData(box: BoundingBox, inset: number): string {
  const left = formatSvgNumber(box.minX + inset);
  const top = formatSvgNumber(box.minY + inset);
  const right = formatSvgNumber(box.maxX - inset);
  const bottom = formatSvgNumber(box.maxY - inset);

  return inset > 0
    ? `M${left} ${top}V${bottom}H${right}V${top}Z`
    : `M${left} ${top}H${right}V${bottom}H${left}Z`;
}

/**
 * Path data of the rectangle's outline, a band centred on its sides: its sides moved outward by
 * half the band, then inward by half the band, a hole unless the rectangle is no wider or no
 * higher than the band.
 *
 * @kind procedure
 * @param box - the rectangle
 * @param half - half the band's width, in user units
 * @returns the band's path data
 * @see docs/domain/interaction.md#_2-selector
 */
function readOutlinePathData(box: BoundingBox, half: number): string {
  const outer = readSidesPathData(box, -half);
  const size = boxSize(box);
  const isHollow = Math.min(size.width, size.height) > 2 * half;

  return isHollow ? formatPathData([outer, readSidesPathData(box, half)]) : outer;
}

/**
 * Creates the selection rectangle in the layer above the drawing (F37), while a drag with the
 * Selector from an empty place has left the press's integer point (`SELECTION_DRAGS`): its area,
 * a light tint of the outline's color, of class `selection-area`, then its outline, a band 1
 * screen pixel wide centred on its sides, of class `selection-rectangle`, a fill with a hole,
 * never a native stroke; a rectangle thinner than the band gets no hole.
 *
 * @kind procedure
 * @param document - page document
 * @returns its area and its outline; nothing without such a drag
 * @see docs/domain/interaction.md#_2-selector
 */
export function createSelectionRectangleElements(document: Document): readonly SVGPathElement[] {
  const drag = SELECTION_DRAGS.get(document);

  if (drag?.isRectangle !== true || !drag.isMoved) {
    return [];
  }

  const box = readSelectionBox(drag);
  const area = createPathElement(document, readSidesPathData(box, 0));
  const outline = createPathElement(
    document,
    readOutlinePathData(box, screenToUser(OUTLINE_WIDTH / 2, VIEW_SCALE)),
  );

  area.classList.add("selection-area");
  outline.classList.add("selection-rectangle");

  return [area, outline];
}
