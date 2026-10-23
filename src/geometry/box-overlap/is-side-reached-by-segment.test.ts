import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { straightCubic } from "../curve";

import { isSideReachedByCubic } from "./is-side-reached-by-cubic";
import { isSideReachedBySegment } from "./is-side-reached-by-segment";

import type { Segment } from "../contour";

/** The diagonal from (0, 0) to (100, 100). */
const DIAGONAL = { end: { x: 100, y: 100 }, start: { x: 0, y: 0 } };

/** An irrational shift of a side's ends, so that no crossing on rationals falls on them. */
const SHIFT = Math.SQRT2 / 10;

/** A coordinate on integers. */
const COORDINATE = fc.integer({ max: 100, min: -100 });

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

describe("isSideReachedBySegment (DERIV-box-overlap step 5)", () => {
  it("reaches a side its crossing point lies on, not one beside it", () => {
    // y = 50 at t = 50 / 100 = ½, x = 50: in [40, 60], not in [60, 80].
    expect(isSideReachedBySegment(DIAGONAL, side(50, 40, 60))).toBe(true);
    expect(isSideReachedBySegment(DIAGONAL, side(50, 60, 80))).toBe(false);
  });

  it("reaches a side at its end, not one beyond it", () => {
    // y = 100 at t = 1, x = 100, in [90, 110]; y = 150 is past the end.
    expect(isSideReachedBySegment(DIAGONAL, side(100, 90, 110))).toBe(true);
    expect(isSideReachedBySegment(DIAGONAL, side(150, 90, 110))).toBe(false);
  });

  it("reaches the same side read backwards", () => {
    const reversed = { end: DIAGONAL.start, start: DIAGONAL.end };

    expect(isSideReachedBySegment(reversed, side(50, 40, 60))).toBe(true);
  });

  it("leaves out a segment along the side's line", () => {
    // Flat at y = 0: its ends and the vertical sides tell (step 8).
    const flat = { end: { x: 100, y: 0 }, start: { x: 0, y: 0 } };

    expect(isSideReachedBySegment(flat, side(0, 0, 100))).toBe(false);
  });

  it("leaves out a segment without length", () => {
    const dot = { end: { x: 5, y: 5 }, start: { x: 5, y: 5 } };

    expect(isSideReachedBySegment(dot, side(5, 0, 10))).toBe(false);
  });
});

describe("isSideReachedBySegment (properties)", () => {
  test.prop({
    end: fc.record({ x: COORDINATE, y: COORDINATE }),
    from: COORDINATE,
    level: COORDINATE,
    start: fc.record({ x: COORDINATE, y: COORDINATE }),
    width: fc.integer({ max: 100, min: 0 }),
  })("agrees with the straight cubic of the same ends", ({ end, from, level, start, width }) => {
    const ends = side(level, from + SHIFT, from + width + SHIFT);

    expect(isSideReachedBySegment({ end, start }, ends)).toBe(
      isSideReachedByCubic(straightCubic(start, end), ends),
    );
  });
});
