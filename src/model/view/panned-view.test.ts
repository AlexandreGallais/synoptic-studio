import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { opposite } from "../../math";

import { pannedView } from "./panned-view";

/** The view of the examples, 200 % from (180, 130). */
const VIEW = { origin: { x: 180, y: 130 }, scale: 2 };

describe("pannedView (DERIV-view-zoom step 6)", () => {
  it("[F42.AC2] follows a drag of (100, 50) pixels at 200 %", () => {
    // o′ = o − d / s = (180 − 100 / 2, 130 − 50 / 2).
    expect(pannedView(VIEW, opposite({ x: 100, y: 50 }))).toEqual({
      origin: { x: 130, y: 105 },
      scale: 2,
    });
  });

  it("[F42.AC2] moves 50 units down by the wheel's (0, 100) at 200 %", () => {
    // o′ = o + δ / s = (180 + 0 / 2, 130 + 100 / 2).
    expect(pannedView(VIEW, { x: 0, y: 100 })).toEqual({ origin: { x: 180, y: 180 }, scale: 2 });
  });

  test.prop({
    delta: fc.record({
      x: fc.integer({ max: 4000, min: -4000 }),
      y: fc.integer({ max: 4000, min: -4000 }),
    }),
    origin: fc.record({
      x: fc.integer({ max: 100_000, min: -100_000 }),
      y: fc.integer({ max: 100_000, min: -100_000 }),
    }),
    scale: fc.double({ max: 256, min: 0.01, noNaN: true }),
  })("gives the view back by the opposite pan, the view unchanged", ({ delta, origin, scale }) => {
    const view = { origin, scale };
    const before = JSON.stringify(view);
    const back = pannedView(pannedView(view, delta), opposite(delta));

    expect(back.scale).toBe(scale);
    expect(back.origin.x).toBeCloseTo(origin.x, 6);
    expect(back.origin.y).toBeCloseTo(origin.y, 6);
    expect(JSON.stringify(view)).toBe(before);
  });
});
