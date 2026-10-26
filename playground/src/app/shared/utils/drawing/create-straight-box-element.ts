import {
  boxSize,
  ceiling,
  createPathElement,
  elementAddress,
  formatSvgNumber,
  halfSize,
  screenToUser,
} from "editor";

import { readView } from "../view";

import type { BoundingBox } from "editor";

/** Length of a dash, in screen pixels. */
const DASH = 4;

/** Distance from one dash to the next, in screen pixels. */
const PERIOD = 7;

/** Thickness of the dashes, in screen pixels. */
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

/** The sizes of the dashes, in user units at the view's scale. */
type DashSizes = {
  /** Length of a dash. */
  readonly length: number;
  /** Distance from one dash to the next. */
  readonly period: number;
  /** Thickness of the dashes. */
  readonly thickness: number;
};

/**
 * Path data of one dash, a thin filled rectangle from `from` along its side, its length or up to
 * the side's end.
 *
 * @kind procedure
 * @param side - the side it lies on
 * @param from - its first coordinate along the side
 * @param sizes - the dashes' length and thickness, in user units
 * @returns the rectangle's path data
 */
function readDash(side: Side, from: number, sizes: DashSizes): string {
  const to = Math.min(from + sizes.length, side.end);
  const rectangle = side.isHorizontal
    ? { bottom: side.at + sizes.thickness, left: from, right: to, top: side.at }
    : { bottom: to, left: side.at, right: side.at + sizes.thickness, top: from };
  const left = formatSvgNumber(rectangle.left);

  return `M${left} ${formatSvgNumber(rectangle.top)}H${formatSvgNumber(rectangle.right)}V${formatSvgNumber(rectangle.bottom)}H${left}Z`;
}

/**
 * Path data of the dashes along one side.
 *
 * @kind procedure
 * @param side - one of the four sides of the box
 * @param sizes - the dashes' length, period and thickness, in user units
 * @returns the dashes' path data
 */
function readSideDashes(side: Side, sizes: DashSizes): string {
  const count = Math.max(ceiling((side.end - side.start) / sizes.period), 0);

  return Array.from({ length: count })
    .keys()
    .map((index) => elementAddress(side.start, sizes.period, index))
    .map((from) => readDash(side, from, sizes))
    .toArray()
    .join(" ");
}

/**
 * Draws the straight box of a turned shape, dashed (as Inkscape's dashed bounding box): the
 * dashes are thin filled rectangles inside the box, never a native stroke, 4 screen pixels long
 * every 7, 1 thick, divided by the view's scale so that they keep their size on screen at any zoom
 * (`screenToUser`, ADR-0098, decision 5).
 *
 * @kind procedure
 * @param document - page document
 * @param box - straight box of the drawn shape
 * @returns the dashed frame's `<path>`
 */
export function createStraightBoxElement(document: Document, box: BoundingBox): SVGPathElement {
  const { scale } = readView(document);
  const sizes = {
    length: screenToUser(DASH, scale),
    period: screenToUser(PERIOD, scale),
    thickness: screenToUser(LINE, scale),
  };
  // In a box thinner than two lines, the far dashes stay inside it.
  const half = halfSize(boxSize(box));
  const across = Math.min(sizes.thickness, half.y);
  const along = Math.min(sizes.thickness, half.x);
  const sides: readonly Side[] = [
    { at: box.minY, end: box.maxX, isHorizontal: true, start: box.minX },
    { at: box.maxY - across, end: box.maxX, isHorizontal: true, start: box.minX },
    { at: box.minX, end: box.maxY, isHorizontal: false, start: box.minY },
    { at: box.maxX - along, end: box.maxY, isHorizontal: false, start: box.minY },
  ];
  const element = createPathElement(
    document,
    sides.map((side) => readSideDashes(side, sizes)).join(" "),
  );

  element.classList.add("straight-box");

  return element;
}
