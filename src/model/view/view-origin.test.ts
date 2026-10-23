import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { pointUnderPointer } from "./point-under-pointer";
import { viewOrigin } from "./view-origin";

describe("viewOrigin (DERIV-view-zoom step 3)", () => {
  it("shows (400, 300) at (440, 340) from (180, 130) at scale 2", () => {
    // (400 − 440 / 2, 300 − 340 / 2).
    expect(viewOrigin({ x: 400, y: 300 }, { x: 440, y: 340 }, 2)).toEqual({ x: 180, y: 130 });
  });

  it("shows a point at the canvas's top-left from that point", () => {
    // (50 − 0 / 4, 60 − 0 / 4).
    expect(viewOrigin({ x: 50, y: 60 }, { x: 0, y: 0 }, 4)).toEqual({ x: 50, y: 60 });
  });

  test.prop({
    offset: fc.record({
      x: fc.integer({ max: 4000, min: 0 }),
      y: fc.integer({ max: 4000, min: 0 }),
    }),
    point: fc.record({
      x: fc.integer({ max: 100_000, min: -100_000 }),
      y: fc.integer({ max: 100_000, min: -100_000 }),
    }),
    scale: fc.double({ max: 256, min: 0.01, noNaN: true }),
  })("shows the point at the offset", ({ offset, point, scale }) => {
    const shown = pointUnderPointer({ origin: viewOrigin(point, offset, scale), scale }, offset);

    expect(shown.x).toBeCloseTo(point.x, 6);
    expect(shown.y).toBeCloseTo(point.y, 6);
  });
});
