import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { pointUnderPointer } from "./point-under-pointer";

describe("pointUnderPointer (DERIV-view-zoom step 2)", () => {
  it("[F42.AC1] reads (400, 300) under (440, 340) at scale 2 from (180, 130)", () => {
    // (180 + 440 / 2, 130 + 340 / 2).
    expect(pointUnderPointer({ origin: { x: 180, y: 130 }, scale: 2 }, { x: 440, y: 340 })).toEqual(
      { x: 400, y: 300 },
    );
  });

  it("reads (0, 0) under (40, 40) at the opening view", () => {
    // (−40 + 40 / 1, −40 + 40 / 1).
    expect(pointUnderPointer({ origin: { x: -40, y: -40 }, scale: 1 }, { x: 40, y: 40 })).toEqual({
      x: 0,
      y: 0,
    });
  });

  it("reads the origin at the canvas's top-left", () => {
    // (180 + 0 / 2, 130 + 0 / 2).
    expect(pointUnderPointer({ origin: { x: 180, y: 130 }, scale: 2 }, { x: 0, y: 0 })).toEqual({
      x: 180,
      y: 130,
    });
  });

  test.prop({
    offset: fc.record({
      x: fc.integer({ max: 4000, min: 0 }),
      y: fc.integer({ max: 4000, min: 0 }),
    }),
    origin: fc.record({
      x: fc.integer({ max: 100_000, min: -100_000 }),
      y: fc.integer({ max: 100_000, min: -100_000 }),
    }),
    scale: fc.double({ max: 256, min: 0.01, noNaN: true }),
  })("inverts the viewBox's map, the view unchanged", ({ offset, origin, scale }) => {
    const view = { origin, scale };
    const before = JSON.stringify(view);
    const point = pointUnderPointer(view, offset);

    // (P − o) · s = p (DERIV-view-zoom step 1).
    expect((point.x - origin.x) * scale).toBeCloseTo(offset.x, 6);
    expect((point.y - origin.y) * scale).toBeCloseTo(offset.y, 6);
    expect(JSON.stringify(view)).toBe(before);
  });
});
