import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { isInPaintedArea } from "./is-in-painted-area";
import { polygonSegments } from "./polygon-segments";

import type { Arc, ContourPiece } from "../contour";

/** A box 100 × 100 at (0, 0), clockwise on screen. */
const SQUARE = polygonSegments([
  { x: 0, y: 0 },
  { x: 100, y: 0 },
  { x: 100, y: 100 },
  { x: 0, y: 100 },
]);

/** The same box, counter-clockwise on screen. */
const REVERSED = SQUARE.map((piece) => ({ ...piece, end: piece.start, start: piece.end }));

/** The left point of a circle of radius 50 about (50, 50). */
const LEFT = { x: 0, y: 50 };

/** Its top point. */
const TOP = { x: 50, y: 0 };

/** Its right point. */
const RIGHT = { x: 100, y: 50 };

/** Its bottom point. */
const BOTTOM = { x: 50, y: 100 };

/** A quarter of that circle, clockwise on screen. */
const QUARTER: Pick<Arc, "center" | "kind" | "positive" | "radius"> = {
  center: { x: 50, y: 50 },
  kind: "arc",
  positive: true,
  radius: 50,
};

/** The circle as four quarter arcs, clockwise on screen from its left. */
const CIRCLE: readonly ContourPiece[] = [
  { ...QUARTER, end: TOP, start: LEFT },
  { ...QUARTER, end: RIGHT, start: TOP },
  { ...QUARTER, end: BOTTOM, start: RIGHT },
  { ...QUARTER, end: LEFT, start: BOTTOM },
];

/** An arch from (0, 0) to (100, 0), 30 high, closed by the segment back to (0, 0). */
const CLOSED_ARCH: readonly ContourPiece[] = [
  {
    end: { x: 100, y: 0 },
    first: { x: 0, y: -40 },
    kind: "cubic",
    second: { x: 100, y: -40 },
    start: { x: 0, y: 0 },
  },
  { end: { x: 0, y: 0 }, kind: "segment", start: { x: 100, y: 0 } },
];

describe("isInPaintedArea (DERIV-painted-area step 5)", () => {
  it("holds inside a box, whichever way it is drawn", () => {
    // Winding numbers +1 and −1: both not 0.
    expect(isInPaintedArea(SQUARE, { x: 50, y: 50 })).toBe(true);
    expect(isInPaintedArea(REVERSED, { x: 50, y: 50 })).toBe(true);
  });

  it("fails outside a box", () => {
    expect(isInPaintedArea(SQUARE, { x: 150, y: 50 })).toBe(false);
  });

  it("holds where a box is travelled twice, which the even-odd rule would leave empty", () => {
    // Winding number 2: nonzero.
    expect(isInPaintedArea([...SQUARE, ...SQUARE], { x: 50, y: 50 })).toBe(true);
  });

  it("holds under the arch and fails beneath its chord", () => {
    // (50, −15): the way down crosses at x ≈ 94.19, +1; (50, 5): no piece spans y = 5.
    expect(isInPaintedArea(CLOSED_ARCH, { x: 50, y: -15 })).toBe(true);
    expect(isInPaintedArea(CLOSED_ARCH, { x: 50, y: 5 })).toBe(false);
  });

  it("holds in a circle of arcs and fails in its box's corner", () => {
    // (50, 50) is the centre; (5, 5) is 45√2 ≈ 63.6 from it, over 50.
    expect(isInPaintedArea(CIRCLE, { x: 50, y: 50 })).toBe(true);
    expect(isInPaintedArea(CIRCLE, { x: 5, y: 5 })).toBe(false);
  });
});

describe("isInPaintedArea (properties)", () => {
  test.prop({ x: fc.double({ max: 120, min: -20 }), y: fc.double({ max: 120, min: -20 }) })(
    "holds in a circle of arcs exactly within its radius",
    (point) => {
      const distance = Math.hypot(point.x - 50, point.y - 50);

      fc.pre(Math.abs(distance - 50) > 1e-6);

      expect(isInPaintedArea(CIRCLE, point)).toBe(distance < 50);
    },
  );
});
