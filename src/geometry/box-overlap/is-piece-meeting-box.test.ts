import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { contourBoundingBox, cubicPoint } from "../placement";

import { isPieceMeetingBox } from "./is-piece-meeting-box";

import type { Arc, Cubic, Segment } from "../contour";
import type { BoundingBox } from "../placement";

/** The arch of `DERIV-box-overlap`: y = −120 t (1 − t), x = 300 t² − 200 t³, its box (0, −30) to (100, 0). */
const ARCH: Cubic = {
  end: { x: 100, y: 0 },
  first: { x: 0, y: -40 },
  kind: "cubic",
  second: { x: 100, y: -40 },
  start: { x: 0, y: 0 },
};

/** The quarter from the left (0, 50) to the top (50, 0) of the circle about (50, 50), positive. */
const QUARTER: Arc = {
  center: { x: 50, y: 50 },
  end: { x: 50, y: 0 },
  kind: "arc",
  positive: true,
  radius: 50,
  start: { x: 0, y: 50 },
};

/** The diagonal from (0, 0) to (100, 100). */
const DIAGONAL: Segment = { end: { x: 100, y: 100 }, kind: "segment", start: { x: 0, y: 0 } };

/** How far a box reaches on each side of a point of the curve. */
const REACH = 0.01;

/**
 * Whether two upright boxes are apart on one axis at least.
 *
 * @param box - the box drawn
 * @param other - the piece's own straight box
 * @returns true when their intervals miss each other on x or on y
 */
function isApart(box: BoundingBox, other: BoundingBox): boolean {
  return (
    box.maxX < other.minX || box.minX > other.maxX || box.maxY < other.minY || box.minY > other.maxY
  );
}

describe("isPieceMeetingBox (DERIV-box-overlap steps 4 and 8)", () => {
  it("meets a box holding one of its ends", () => {
    // (100, 100) is in [90, 110]²; (0, 0) in [−5, 5]².
    expect(isPieceMeetingBox(DIAGONAL, { maxX: 110, maxY: 110, minX: 90, minY: 90 })).toBe(true);
    expect(isPieceMeetingBox(DIAGONAL, { maxX: 5, maxY: 5, minX: -5, minY: -5 })).toBe(true);
  });

  it("meets a box the arch's top lies in, entered by its vertical sides", () => {
    // (50, −30) is in [45, 55] × [−35, −25], both ends outside; on x = 45, t ≈ 0.4665,
    // y ≈ −29.87, within [−35, −25].
    expect(isPieceMeetingBox(ARCH, { maxX: 55, maxY: -25, minX: 45, minY: -35 })).toBe(true);
  });

  it("misses the corner of the arch's box", () => {
    // [0, 10] × [−30, −20]: on y = −20, x ≈ 11.51 > 10; on x = 10, t ≈ 0.1955,
    // y ≈ −18.87, below −20.
    expect(isPieceMeetingBox(ARCH, { maxX: 10, maxY: -20, minX: 0, minY: -30 })).toBe(false);
  });

  it("meets a box the arc crosses, its ends outside", () => {
    // On y = 25 the arc is at x ≈ 6.69873, in [0, 10] × [20, 30].
    expect(isPieceMeetingBox(QUARTER, { maxX: 10, maxY: 30, minX: 0, minY: 20 })).toBe(true);
  });

  it("misses a box the circle crosses off the arc, and one in the corner of the circle's box", () => {
    // (93.30127, 25) is on the circle, off the quarter; (10, 10) is 40√2 ≈ 56.57 from the centre.
    expect(isPieceMeetingBox(QUARTER, { maxX: 100, maxY: 30, minX: 90, minY: 20 })).toBe(false);
    expect(isPieceMeetingBox(QUARTER, { maxX: 10, maxY: 10, minX: 0, minY: 0 })).toBe(false);
  });

  it("meets a box a flat segment runs across, by a vertical side", () => {
    // y = 5 from x = −10 to 20 across [0, 10]², both ends outside.
    const flat: Segment = { end: { x: 20, y: 5 }, kind: "segment", start: { x: -10, y: 5 } };

    expect(isPieceMeetingBox(flat, { maxX: 10, maxY: 10, minX: 0, minY: 0 })).toBe(true);
  });

  it("meets a box a vertical segment runs across, by a horizontal side", () => {
    // x = 5 from y = −10 to 20 across [0, 10]².
    const upright: Segment = { end: { x: 5, y: 20 }, kind: "segment", start: { x: 5, y: -10 } };

    expect(isPieceMeetingBox(upright, { maxX: 10, maxY: 10, minX: 0, minY: 0 })).toBe(true);
  });
});

describe("isPieceMeetingBox (properties)", () => {
  test.prop({ t: fc.double({ max: 1, min: 0, noNaN: true }) })(
    "meets a small box about any point of the arch",
    ({ t }) => {
      const { x, y } = cubicPoint(ARCH, t);

      expect(
        isPieceMeetingBox(ARCH, {
          maxX: x + REACH,
          maxY: y + REACH,
          minX: x - REACH,
          minY: y - REACH,
        }),
      ).toBe(true);
    },
  );

  test.prop({
    minX: fc.integer({ max: 200, min: -200 }),
    minY: fc.integer({ max: 200, min: -200 }),
    size: fc.integer({ max: 50, min: 0 }),
  })("never meets a box apart from the piece's own box", ({ minX, minY, size }) => {
    const box = { maxX: minX + size, maxY: minY + size, minX, minY };

    for (const piece of [ARCH, QUARTER, DIAGONAL]) {
      expect(isApart(box, contourBoundingBox([piece])) && isPieceMeetingBox(piece, box)).toBe(
        false,
      );
    }
  });
});
