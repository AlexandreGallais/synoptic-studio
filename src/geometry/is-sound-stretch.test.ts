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
