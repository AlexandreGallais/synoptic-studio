import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { isPointInBox } from "./is-point-in-box";

/** A box from (0, 0) to (10, 10). */
const BOX = { maxX: 10, maxY: 10, minX: 0, minY: 0 };

describe("isPointInBox (DERIV-box-overlap step 1)", () => {
  it("holds inside the box and on its sides and corners", () => {
    // 0 ≤ 5 ≤ 10 on both axes; (10, 0) is a corner; (0, 7) on the left side.
    expect(isPointInBox(BOX, { x: 5, y: 5 })).toBe(true);
    expect(isPointInBox(BOX, { x: 10, y: 0 })).toBe(true);
    expect(isPointInBox(BOX, { x: 0, y: 7 })).toBe(true);
  });

  it("fails outside the box on either axis", () => {
    // 11 > 10 on x; −1 < 0 on y.
    expect(isPointInBox(BOX, { x: 11, y: 5 })).toBe(false);
    expect(isPointInBox(BOX, { x: 5, y: -1 })).toBe(false);
  });

  it("holds in a box of one point at that point only", () => {
    const point = { maxX: 3, maxY: 4, minX: 3, minY: 4 };

    expect(isPointInBox(point, { x: 3, y: 4 })).toBe(true);
    expect(isPointInBox(point, { x: 3, y: 4.5 })).toBe(false);
  });
});

describe("isPointInBox (properties)", () => {
  test.prop({
    x: fc.integer({ max: 20, min: -10 }),
    y: fc.integer({ max: 20, min: -10 }),
  })("holds exactly when each coordinate is in the box's interval", ({ x, y }) => {
    expect(isPointInBox(BOX, { x, y })).toBe(x >= 0 && x <= 10 && y >= 0 && y <= 10);
  });
});
