import { cyclicItem } from "../../geometry";
import { add, norm, scale, subtract } from "../../math";

import { acrossSide } from "./across-side";

import type { PastedCurve } from "./pasted-curve";
import type { Point } from "../../math";

/**
 * A number as SVG path data writes it (`REF-SVG2-PATHS` §9.3.9): a sign, digits with an optional
 * fraction, an exponent; never followed by a digit or a point, so that one run of digits is never
 * read as two numbers (the grammar consumes as much as it can).
 */
const NUMBER = String.raw`[-+]?(?:\d*\.\d+|\d+)(?:[eE][-+]?\d+)?(?![\d.])`;

/** A separator between numbers: spaces, or a comma with spaces around it. */
const NUMBER_GAP = String.raw`\s*,?\s*`;

/** One SVG `C` command with its six numbers (`REF-SVG2-PATHS` §9.3.6). */
const CUBIC_COMMAND = new RegExp(
  String.raw`^\s*C\s*(${NUMBER})${NUMBER_GAP}(${NUMBER})${NUMBER_GAP}(${NUMBER})${NUMBER_GAP}(${NUMBER})${NUMBER_GAP}(${NUMBER})${NUMBER_GAP}(${NUMBER})\s*$`,
  "u",
);

/** A CSS `cubic-bezier()` value with its four numbers (`REF-CSS-EASING` §2.2). */
const CSS_CUBIC = new RegExp(
  String.raw`^\s*cubic-bezier\(\s*(${NUMBER})\s*,\s*(${NUMBER})\s*,\s*(${NUMBER})\s*,\s*(${NUMBER})\s*\)\s*$`,
  "u",
);

/** How far a pasted `C`'s end may be from the side's end: the precision of SVG numbers (ADR-0003). */
const END_TOLERANCE = 1e-5;

/** The origin: a point missing from what was pasted. */
const ORIGIN = { x: 0, y: 0 };

/**
 * Numbers read two by two as points, x then y.
 *
 * @kind domain
 * @param numbers - an even count of numbers
 * @returns the points, in order
 * @see REF-SVG2-PATHS
 */
function pairsOf(numbers: readonly number[]): readonly Point[] {
  return numbers
    .map((x, index) => ({ index, point: { x, y: cyclicItem(numbers, index + 1, 0) } }))
    .filter(({ index }) => index % 2 === 0)
    .map(({ point }) => point);
}

/**
 * Whether an x of CSS's unit square is allowed: from 0 to 1 (`REF-CSS-EASING`).
 *
 * @kind domain
 * @param point - a control point of the unit square
 * @returns `true` when its x is from 0 to 1
 * @see REF-CSS-EASING
 */
function isInUnitWidth(point: Point): boolean {
  return point.x >= 0 && point.x <= 1;
}

/**
 * A point of CSS's unit square laid on a side, its diagonal from (0, 0) to (1, 1) on the side:
 * `(x + y) / 2` along it, `(y − x) / 2` across it to the left of travel (`DERIV-handle-share` step 4).
 *
 * @kind domain
 * @param start - where the side begins, the image of (0, 0)
 * @param side - from the start to the end
 * @param square - the point of the unit square
 * @returns the point on the page
 * @see DERIV-handle-share
 */
function pointOnSide(start: Point, side: Point, square: Point): Point {
  const across = acrossSide({ isBefore: false, point: start, side });

  return add(
    add(start, scale(side, (square.x + square.y) / 2)),
    scale(across, (square.y - square.x) / 2),
  );
}

/**
 * The control points of a CSS `cubic-bezier()` laid on a side; x values outside 0 to 1 refused,
 * as CSS refuses them (`REF-CSS-EASING`).
 *
 * @kind domain
 * @param numbers - x1, y1, x2, y2
 * @param start - the side's start
 * @param end - the side's end
 * @returns the curve, or why it was refused
 * @see DERIV-handle-share
 */
function cssCurve(numbers: readonly number[], start: Point, end: Point): PastedCurve {
  const square = pairsOf(numbers);
  const side = subtract(end, start);

  return square.every((point) => isInUnitWidth(point))
    ? {
        first: pointOnSide(start, side, cyclicItem(square, 0, ORIGIN)),
        second: pointOnSide(start, side, cyclicItem(square, 1, ORIGIN)),
      }
    : { refusal: "A cubic-bezier() takes x values from 0 to 1." };
}

/**
 * The control points of an SVG `C` command; refused when its end is not the side's end.
 *
 * @kind domain
 * @param numbers - x1, y1, x2, y2, x, y
 * @param end - the side's end
 * @returns the curve, or why it was refused
 * @see REF-SVG2-PATHS
 */
function pathCurve(numbers: readonly number[], end: Point): PastedCurve {
  const points = pairsOf(numbers);
  const last = cyclicItem(points, 2, ORIGIN);

  return norm(subtract(last, end)) > END_TOLERANCE
    ? { refusal: "The C must end on the side's last point." }
    : { first: cyclicItem(points, 0, ORIGIN), second: cyclicItem(points, 1, ORIGIN) };
}

/** Why a number pasted was refused: too large to be a number (`1e400`). */
const TOO_LARGE = { refusal: "A number pasted is too large." };

/**
 * The curve of one SVG `C` command, if the text is one.
 *
 * @kind domain
 * @param text - what was pasted
 * @param end - the side's end
 * @returns the curve or why it was refused; `undefined` when the text is no `C` command
 * @see REF-SVG2-PATHS
 */
function commandCurve(text: string, end: Point): PastedCurve | undefined {
  const numbers = CUBIC_COMMAND.exec(text)?.slice(1).map(Number);

  if (numbers === undefined) {
    return undefined;
  }

  return numbers.every((value) => Number.isFinite(value)) ? pathCurve(numbers, end) : TOO_LARGE;
}

/**
 * The curve of a CSS `cubic-bezier()` value laid on the side, if the text is one.
 *
 * @kind domain
 * @param text - what was pasted
 * @param start - the side's start
 * @param end - the side's end
 * @returns the curve or why it was refused; `undefined` when the text is no `cubic-bezier()`
 * @see REF-CSS-EASING
 */
function easingCurve(text: string, start: Point, end: Point): PastedCurve | undefined {
  const numbers = CSS_CUBIC.exec(text)?.slice(1).map(Number);

  if (numbers === undefined) {
    return undefined;
  }

  return numbers.every((value) => Number.isFinite(value))
    ? cssCurve(numbers, start, end)
    : TOO_LARGE;
}

/**
 * The two control points of a curve pasted on a side, from one SVG `C` command (its end on the
 * side's end) or a CSS `cubic-bezier()` value laid on the side (research note 0013, method 6);
 * any other text refused with a line saying why. Decimals are kept: handles are stored exactly
 * (ADR-0046).
 *
 * @kind domain
 * @param text - what the designer pasted, as it is
 * @param start - where the side begins: the point whose handle after is set
 * @param end - where the side ends: the point whose handle before is set
 * @returns the control points, or why the text was refused
 * @see DERIV-handle-share
 */
export function pastedControlPoints(text: string, start: Point, end: Point): PastedCurve {
  return (
    commandCurve(text, end) ??
    easingCurve(text, start, end) ?? {
      refusal: "Paste one C command (C x1 y1 x2 y2 x y) or a cubic-bezier() value.",
    }
  );
}
