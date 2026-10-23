import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { boxHorizontalSides } from "./box-horizontal-sides";
import { isHorizontalSideReached } from "./is-horizontal-side-reached";
import { isSideReachedByPiece } from "./is-side-reached-by-piece";

import type { Cubic } from "../contour";

/** The arch of `DERIV-box-overlap`: y = −120 t (1 − t), x = 300 t² − 200 t³. */
const ARCH: Cubic = {
  end: { x: 100, y: 0 },
  first: { x: 0, y: -40 },
  kind: "cubic",
  second: { x: 100, y: -40 },
  start: { x: 0, y: 0 },
};

describe("isHorizontalSideReached (DERIV-box-overlap step 4)", () => {
  it("reaches a box by its top side", () => {
    // Top y = −15 from 0 to 10: the arch crosses it at x ≈ 5.80583.
    expect(isHorizontalSideReached(ARCH, { maxX: 10, maxY: -5, minX: 0, minY: -15 })).toBe(true);
  });

  it("reaches a box by its bottom side", () => {
    // Bottom y = −15 from 90 to 100: x ≈ 94.19417.
    expect(isHorizontalSideReached(ARCH, { maxX: 100, maxY: -15, minX: 90, minY: -25 })).toBe(true);
  });

  it("misses a box the arch enters by its vertical sides only", () => {
    // [40, 60] × [−35, −25]: nothing on y = −35, above the top −30; on y = −25,
    // t (1 − t) = 25 / 120, t ≈ 0.29588, x ≈ 21.08 and 78.92, outside [40, 60].
    expect(isHorizontalSideReached(ARCH, { maxX: 60, maxY: -25, minX: 40, minY: -35 })).toBe(false);
  });
});

describe("isHorizontalSideReached (properties)", () => {
  test.prop({
    height: fc.integer({ max: 40, min: 0 }),
    minX: fc.integer({ max: 110, min: -10 }),
    minY: fc.integer({ max: 10, min: -40 }),
    width: fc.integer({ max: 40, min: 0 }),
  })(
    "holds exactly when the top or the bottom side is reached",
    ({ height, minX, minY, width }) => {
      const box = { maxX: minX + width, maxY: minY + height, minX, minY };
      const [top, bottom] = boxHorizontalSides(box);

      expect(isHorizontalSideReached(ARCH, box)).toBe(
        isSideReachedByPiece(ARCH, top) || isSideReachedByPiece(ARCH, bottom),
      );
    },
  );
});
