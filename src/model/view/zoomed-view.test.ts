import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { pointUnderPointer } from "./point-under-pointer";
import { zoomedView } from "./zoomed-view";

/** The view as the page opens (ADR-0098, decision 1). */
const OPENING = { origin: { x: -40, y: -40 }, scale: 1 };

/** The pointer of the examples, over (400, 300) at the opening view. */
const POINTER = { x: 440, y: 340 };

describe("zoomedView (DERIV-view-zoom steps 3 and 4)", () => {
  it("[F42.AC1] zooms to 200 % about the pointer, (400, 300) kept under it", () => {
    // P = (400, 300); o′ = (400 − 440 / 2, 300 − 340 / 2).
    expect(zoomedView(OPENING, POINTER, 2)).toEqual({ origin: { x: 180, y: 130 }, scale: 2 });
  });

  it("takes 300 at the upper bound, 256, the point still kept", () => {
    // o′ = (400 − 440 / 256, 300 − 340 / 256) = (400 − 1.71875, 300 − 1.328125).
    expect(zoomedView(OPENING, POINTER, 300)).toEqual({
      origin: { x: 398.28125, y: 298.671875 },
      scale: 256,
    });
  });

  it("takes 0.001 at the lower bound, 0.01", () => {
    // o′ = (400 − 440 / 0.01, 300 − 340 / 0.01) = (400 − 44 000, 300 − 34 000).
    const view = zoomedView(OPENING, POINTER, 0.001);

    expect(view.scale).toBe(0.01);
    expect(view.origin.x).toBeCloseTo(-43_600, 6);
    expect(view.origin.y).toBeCloseTo(-33_700, 6);
  });

  it("changes nothing at the same scale", () => {
    // o′ = o + p / 2 − p / 2 = o.
    expect(zoomedView({ origin: { x: 180, y: 130 }, scale: 2 }, POINTER, 2)).toEqual({
      origin: { x: 180, y: 130 },
      scale: 2,
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
    zoom: fc.double({ max: 1000, min: 0.0001, noNaN: true }),
  })(
    "[F42.AC1] keeps the point under the pointer, the view unchanged",
    ({ offset, origin, scale, zoom }) => {
      const view = { origin, scale };
      const before = JSON.stringify(view);
      const kept = pointUnderPointer(view, offset);
      const zoomed = zoomedView(view, offset, zoom);
      const after = pointUnderPointer(zoomed, offset);

      expect(after.x).toBeCloseTo(kept.x, 4);
      expect(after.y).toBeCloseTo(kept.y, 4);
      expect(zoomed.scale).toBeGreaterThanOrEqual(0.01);
      expect(zoomed.scale).toBeLessThanOrEqual(256);
      expect(JSON.stringify(view)).toBe(before);
    },
  );
});
