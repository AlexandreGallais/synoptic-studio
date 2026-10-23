import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { isSideReachedByPiece } from "./is-side-reached-by-piece";

import type { Arc, Cubic, Segment } from "../contour";

/** The arch of `DERIV-box-overlap`: y = −120 t (1 − t), x = 300 t² − 200 t³. */
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

/**
 * A horizontal side.
 *
 * @param level - its ordinate
 * @param from - its left abscissa
 * @param to - its right abscissa
 * @returns its start and its end
 */
function side(level: number, from: number, to: number): Pick<Segment, "end" | "start"> {
  return { end: { x: to, y: level }, start: { x: from, y: level } };
}

describe("isSideReachedByPiece (DERIV-box-overlap steps 5 to 7)", () => {
  it("takes a segment by its crossing point", () => {
    // x = 50 on y = 50.
    expect(isSideReachedByPiece(DIAGONAL, side(50, 40, 60))).toBe(true);
    expect(isSideReachedByPiece(DIAGONAL, side(50, 60, 80))).toBe(false);
  });

  it("takes a fillet arc by its circle's points on the arc", () => {
    // x ≈ 6.69873 on the arc, 93.30127 off it.
    expect(isSideReachedByPiece(QUARTER, side(25, 0, 10))).toBe(true);
    expect(isSideReachedByPiece(QUARTER, side(25, 90, 100))).toBe(false);
  });

  it("takes a cubic side by its parts monotone in y", () => {
    // x ≈ 5.80583 and 94.19417 on y = −15.
    expect(isSideReachedByPiece(ARCH, side(-15, 0, 10))).toBe(true);
    expect(isSideReachedByPiece(ARCH, side(-15, 10, 90))).toBe(false);
  });
});

describe("isSideReachedByPiece (properties)", () => {
  test.prop({ from: fc.integer({ max: 100, min: -100 }), width: fc.integer({ max: 100, min: 0 }) })(
    "never reaches a side above every piece",
    ({ from, width }) => {
      // The arch tops at −30, the quarter at 0, the diagonal at 0: y = −31 is above them all.
      for (const piece of [ARCH, QUARTER, DIAGONAL]) {
        expect(isSideReachedByPiece(piece, side(-31, from, from + width))).toBe(false);
      }
    },
  );
});
