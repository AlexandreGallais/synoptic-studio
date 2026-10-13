import { createPathElement, formatSvgNumber } from "../../src";

import type { BoundingBox } from "../../src";

/** Length of a dash, in user units. */
const DASH = 4;

/** Distance from one dash to the next, in user units. */
const PERIOD = 7;

/** Thickness of the dashes, in user units. */
const LINE = 1;

/** One side of the box: along x at a given y, or along y at a given x. */
type Side = {
  /** Whether the side runs along x. */
  readonly isHorizontal: boolean;
  /** Its coordinate across: the top of a horizontal dash, the left of a vertical one. */
  readonly at: number;
  /** Its first coordinate along. */
  readonly start: number;
  /** Its last coordinate along. */
  readonly end: number;
};

/**
 * Path data of one dash, a thin filled rectangle from `from` to `to` along its side.
 *
 * @kind procedure
 * @param side - the side it lies on
 * @param from - its first coordinate along the side
 * @param to - its last coordinate along the side
 * @returns the rectangle's path data
 * @see docs/backlog/stories/E01-F04-US-018-turn-a-shape.md
 */
function readDash(side: Side, from: number, to: number): string {
  const rectangle = side.isHorizontal
    ? { bottom: side.at + LINE, left: from, right: to, top: side.at }
    : { bottom: to, left: side.at, right: side.at + LINE, top: from };
  const left = formatSvgNumber(rectangle.left);

  return `M${left} ${formatSvgNumber(rectangle.top)}H${formatSvgNumber(rectangle.right)}V${formatSvgNumber(rectangle.bottom)}H${left}Z`;
}

/**
 * Path data of the dashes along one side.
 *
 * @kind procedure
 * @param side - one of the four sides of the box
 * @returns the dashes' path data
 * @see docs/backlog/stories/E01-F04-US-018-turn-a-shape.md
 */
function readSideDashes(side: Side): string {
  const count = Math.max(Math.ceil((side.end - side.start) / PERIOD), 0);

  return Array.from({ length: count })
    .keys()
    .map((index) => side.start + index * PERIOD)
    .map((from) => readDash(side, from, Math.min(from + DASH, side.end)))
    .toArray()
    .join(" ");
}

/**
 * Draws the straight box of a turned shape, dashed (F04, as Inkscape's dashed bounding box): the
 * dashes are thin filled rectangles inside the box, never a native stroke.
 *
 * @kind procedure
 * @param document - page document
 * @param box - straight box of the drawn shape
 * @returns the dashed frame's `<path>`
 * @see docs/backlog/stories/E01-F04-US-018-turn-a-shape.md
 */
export function createStraightBoxElement(document: Document, box: BoundingBox): SVGPathElement {
  // In a box thinner than two lines, the far dashes stay inside it.
  const across = Math.min(LINE, (box.maxY - box.minY) / 2);
  const along = Math.min(LINE, (box.maxX - box.minX) / 2);
  const sides: readonly Side[] = [
    { at: box.minY, end: box.maxX, isHorizontal: true, start: box.minX },
    { at: box.maxY - across, end: box.maxX, isHorizontal: true, start: box.minX },
    { at: box.minX, end: box.maxY, isHorizontal: false, start: box.minY },
    { at: box.maxX - along, end: box.maxY, isHorizontal: false, start: box.minY },
  ];
  const element = createPathElement(document, sides.map((side) => readSideDashes(side)).join(" "));

  element.classList.add("straight-box");

  return element;
}
