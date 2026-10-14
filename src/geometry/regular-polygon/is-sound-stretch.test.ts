import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { isSoundStretch } from "./is-sound-stretch";
import { unitStretchedPolygon } from "./unit-stretched-polygon";

describe("isSoundStretch (DERIV-stretched-polygon step 3)", () => {
  it("accepts the regular polygons: every corner turns 2π/n, twice the guard", () => {
    for (const corners of [3, 4, 5, 6, 7, 8, 9, 10, 11, 12]) {
      expect(isSoundStretch(unitStretchedPolygon(corners, 1))).toBe(true);
    }
  });

  it("refuses a hexagon whose base corners turn less than π/6", () => {
    // tan φ₁ = λ √3 (step 2): λ = tan 25° / √3 gives base corners turning 25° < 30°.
    const scale = Math.tan((25 * Math.PI) / 180) / Math.sqrt(3);

    expect(isSoundStretch(unitStretchedPolygon(6, scale))).toBe(false);
  });

  it("accepts a hexagon whose base corners turn 35°, the others 110° and 35°", () => {
    const scale = Math.tan((35 * Math.PI) / 180) / Math.sqrt(3);

    expect(isSoundStretch(unitStretchedPolygon(6, scale))).toBe(true);
  });

  it("refuses a polygon that does not close: NaN turns", () => {
    const open = unitStretchedPolygon(5, Math.exp(-3));

    expect(isSoundStretch(open)).toBe(false);
  });
});

/** Number of corners of a polygon of the editor. */
const CORNERS = fc.integer({ max: 12, min: 3 });

describe("isSoundStretch (properties)", () => {
  test.prop({
    corners: CORNERS,
    size: fc.integer({ max: 5000, min: 1 }),
    x: fc.integer({ max: 5000, min: -5000 }),
    y: fc.integer({ max: 5000, min: -5000 }),
  })("accepts every regular polygon, whatever its size and place", ({ corners, size, x, y }) => {
    // A regular polygon turns 2π/n at each corner, twice the guard π/n. Clockwise on screen (y down).
    const polygon = Array.from({ length: corners }, (empty, index) => ({
      x: x + size * Math.cos((2 * Math.PI * index) / corners),
      y: y + size * Math.sin((2 * Math.PI * index) / corners),
    }));

    expect(isSoundStretch(polygon)).toBe(true);
    // Drawn the other way (counterclockwise on screen), every corner turns −2π/n, below the guard.
    expect(isSoundStretch(polygon.toReversed())).toBe(false);
  });
});
