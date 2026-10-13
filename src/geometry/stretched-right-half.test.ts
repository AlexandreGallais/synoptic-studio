import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { stretchedRightHalf } from "./stretched-right-half";

import type { Point } from "../math";

/** Decimals compared: sums of cosines and sines, correct to their last bits. */
const DECIMALS = 12;

describe("stretchedRightHalf (DERIV-stretched-polygon step 1)", () => {
  it("builds the regular hexagon's right half at λ = 1 from the right end of the half base", () => {
    // Sides of length 1 at π/3 then 2π/3 from (1/2, 0): (1, √3/2), then (1/2, √3).
    const half = stretchedRightHalf(6, 1);

    expect(half).toHaveLength(3);
    expect(half[0]).toEqual({ x: 0.5, y: 0 });
    expect(half[1]?.x).toBeCloseTo(1, DECIMALS);
    expect(half[1]?.y).toBeCloseTo(Math.sqrt(3) / 2, DECIMALS);
    expect(half[2]?.x).toBeCloseTo(0.5, DECIMALS);
    expect(half[2]?.y).toBeCloseTo(Math.sqrt(3), DECIMALS);
  });

  it("ends an odd polygon on the axis: the triangle's apex at (0, √3/2)", () => {
    // One side at 2π/3 from (1/2, 0): (1/2 − 1/2, √3/2).
    const [, apex] = stretchedRightHalf(3, 1);

    expect(apex?.x).toBeCloseTo(0, DECIMALS);
    expect(apex?.y).toBeCloseTo(Math.sqrt(3) / 2, DECIMALS);
  });
});

/** Vertical scale `λ = e^t`, `t` from −1 to 1 in steps of 1/100. */
const SCALE = fc.integer({ max: 100, min: -100 }).map((step) => Math.exp(step / 100));

/**
 * Right half without its closing vertex when that one cannot exist: only the closing vertex of an
 * odd polygon may be NaN, when no side reaches the axis.
 *
 * @param corners - number of corners
 * @param scale - vertical scale
 * @returns the vertices, every one finite
 */
function closedHalf(corners: number, scale: number): readonly Point[] {
  const half = stretchedRightHalf(corners, scale);
  const isOpen = corners % 2 === 1 && Number.isNaN(half.at(-1)?.x);
  const closed = isOpen ? half.slice(0, -1) : half;

  return closed.every((vertex) => Number.isFinite(vertex.x) && Number.isFinite(vertex.y))
    ? closed
    : [];
}

describe("stretchedRightHalf (properties)", () => {
  test.prop({ corners: fc.integer({ max: 12, min: 3 }), scale: SCALE })(
    "starts at (1/2, 0) and joins its vertices by sides of length 1",
    ({ corners, scale }) => {
      const half = stretchedRightHalf(corners, scale);

      expect(half).toHaveLength(Math.ceil(corners / 2));
      expect(half[0]).toEqual({ x: 0.5, y: 0 });

      for (const [index, vertex] of closedHalf(corners, scale).slice(1).entries()) {
        const previous = half[index] ?? vertex;

        expect(Math.hypot(vertex.x - previous.x, vertex.y - previous.y)).toBeCloseTo(1, 12);
        // Every side rises: its direction is in (0, π).
        expect(vertex.y).toBeGreaterThan(previous.y);
      }
    },
  );

  test.prop({ corners: fc.integer({ max: 12, min: 3 }), scale: SCALE })(
    "ends on the axis (odd n) or above the base's right end (even n)",
    ({ corners, scale }) => {
      const end = stretchedRightHalf(corners, scale).at(-1)?.x ?? NaN;

      // An odd polygon whose last side cannot reach the axis has no end to check.
      fc.pre(!Number.isNaN(end));
      // The apex of an odd polygon is at x = 0; the top side of an even one mirrors the base, x = 1/2.
      expect(end).toBeCloseTo(corners % 2 === 1 ? 0 : 0.5, 9);
    },
  );
});
