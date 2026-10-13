import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { boxAspect } from "../box";

import { isSoundStretch } from "./is-sound-stretch";
import { stretchScale } from "./stretch-scale";
import { unitStretchedPolygon } from "./unit-stretched-polygon";

/** Every number of corners of a polygon (Q19). */
const CORNERS = [3, 4, 5, 6, 7, 8, 9, 10, 11, 12];

/** Values of `ln λ` swept, from −3 to 3 in steps of 0.001 (`DERIV-stretched-polygon` step 5). */
const LOGS = Array.from({ length: 6001 }, (empty, index) => (index - 3000) / 1000);

/**
 * Number of places where a list of booleans changes value.
 *
 * @param values - booleans in order
 * @returns how many neighbours differ
 */
function changeCount(values: readonly boolean[]): number {
  return values.filter((isValue, index) => index > 0 && isValue !== values[index - 1]).length;
}

describe("stretchScale (DERIV-stretched-polygon step 5)", () => {
  it("gives λ = 1 when the box has the regular aspect: the octagon in a square", () => {
    expect(stretchScale(8, 1)).toBeCloseTo(1, 12);
  });

  it("gives the hexagon in a square λ = tan φ₁ / √3 = 1.27898 (note 0011)", () => {
    // φ₁ = 65.7048° makes 2 sin φ₁ = 1 + 2 cos φ₁ (step 4).
    expect(stretchScale(6, 1)).toBeCloseTo(1.27898, 5);
  });

  it("stops at the corner guard in a box too wide or too tall: the hexagon's base corners turn 30° or its side corners 30°", () => {
    // φ₁ = 30° is λ = tan 30° / √3 = 1/3; φ₁ = 75° is λ = tan 75° / √3.
    expect(stretchScale(6, 50 / 300)).toBeCloseTo(1 / 3, 9);
    expect(stretchScale(6, 300 / 50)).toBeCloseTo(Math.tan((75 * Math.PI) / 180) / Math.sqrt(3), 9);
  });

  it("gives the triangle and the square a scale whatever the box, their shape not depending on it", () => {
    for (const ratio of [0, 0.1, 1, 10, Infinity, NaN]) {
      expect(stretchScale(3, ratio)).toBeGreaterThan(0);
      expect(stretchScale(4, ratio)).toBeGreaterThan(0);
    }
  });

  it("reaches the pentagon's highest sound scale, λ = 6.74030, beyond ln λ = 1.9", () => {
    // Its side corners then turn exactly π/5 (review of EN-017).
    expect(stretchScale(5, 300 / 100)).toBeCloseTo(6.7403, 4);
  });

  it("gives a box of 0 × 0 (ratio NaN) the lowest sound scale", () => {
    expect(stretchScale(6, NaN)).toBeCloseTo(1 / 3, 9);
  });

  it("checks what the bisections assume: the aspect grows with λ, wherever the polygon closes", () => {
    for (const corners of CORNERS) {
      const aspects = LOGS.map((log) => boxAspect(unitStretchedPolygon(corners, Math.exp(log))));
      const closed = aspects.filter((aspect) => !Number.isNaN(aspect));
      const isGrowing = closed.every(
        (aspect, index) => index === 0 || aspect >= (closed[index - 1] ?? 0),
      );

      expect(isGrowing).toBe(true);
    }
  });

  it("checks what the bisections assume: the sound scales form one interval containing λ = 1", () => {
    for (const corners of CORNERS) {
      const sound = LOGS.map((log) => isSoundStretch(unitStretchedPolygon(corners, Math.exp(log))));
      const below = sound.slice(0, 3001);
      const above = sound.slice(3000);

      // At most one change on each side of λ = 1 (index 3000): one bound each for the bisections.
      expect(sound[3000]).toBe(true);
      // The bracket [−3, 3] holds every sound scale: its ends are unsound from 5 corners on.
      expect(corners < 5 || (sound[0] === false && sound[6000] === false)).toBe(true);
      expect(changeCount(below)).toBeLessThanOrEqual(1);
      expect(changeCount(above)).toBeLessThanOrEqual(1);
    }
  });
});

/**
 * Aspect of the polygon drawn for a box ratio.
 *
 * @param corners - number of corners
 * @param ratio - height of the box over its width
 * @returns the aspect of the unit polygon at the scale chosen for the ratio
 */
function scaledAspect(corners: number, ratio: number): number {
  return boxAspect(unitStretchedPolygon(corners, stretchScale(corners, ratio)));
}

describe("stretchScale (properties)", () => {
  // From 5 corners on: the triangle's and the square's shapes do not depend on λ (examples above).
  test.prop({
    corners: fc.integer({ max: 12, min: 5 }),
    share: fc.integer({ max: 100, min: 0 }),
  })(
    "gives back a ratio between the guard aspects as the aspect of its polygon",
    ({ corners, share }) => {
      // Round trip inside the clamp: aspect(λ(r)) = r for r between the aspects at the two guards.
      const lowest = scaledAspect(corners, 0);
      const highest = scaledAspect(corners, Infinity);
      const ratio = lowest + ((highest - lowest) * share) / 100;

      expect(scaledAspect(corners, ratio)).toBeCloseTo(ratio, 6);
    },
  );

  test.prop({
    corners: fc.integer({ max: 12, min: 5 }),
    first: fc.integer({ max: 1000, min: 1 }),
    second: fc.integer({ max: 1000, min: 1 }),
  })(
    "gives a taller box a scale at least as large, always positive",
    ({ corners, first, second }) => {
      // Ratios h / w from 0.01 to 10: λ grows with the ratio, then stops at the corner guard.
      const low = stretchScale(corners, Math.min(first, second) / 100);
      const high = stretchScale(corners, Math.max(first, second) / 100);

      expect(low).toBeGreaterThan(0);
      expect(high).toBeGreaterThanOrEqual(low);
    },
  );
});
