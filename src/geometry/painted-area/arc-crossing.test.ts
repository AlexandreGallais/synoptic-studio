import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { arcCrossing } from "./arc-crossing";

import type { Arc } from "../contour";

/** The fillet of the top-left corner of a box 100 × 100 of radius 20, from (0, 20) up to (20, 0). */
const FILLET: Arc = {
  center: { x: 20, y: 20 },
  end: { x: 20, y: 0 },
  kind: "arc",
  positive: true,
  radius: 20,
  start: { x: 0, y: 20 },
};

/** The same fillet travelled the other way. */
const REVERSED: Arc = { ...FILLET, end: FILLET.start, positive: false, start: FILLET.end };

/** A coordinate, an integer, so that the side tests are exact. */
const COORDINATE = fc.integer({ max: 40, min: -20 });

describe("arcCrossing (DERIV-painted-area step 3)", () => {
  it("counts 0 for (8, 8): the ray meets the arc at x = 4 only, left of the point", () => {
    // Chord: y falls, d = −1; (20, −20) × (8, −12) = −80 < 0: −1. Inside the circular segment,
    // σ = +1: −1 + 1 = 0. On the circle at y = 8, (x − 20)² = 400 − 144 = 256: x = 4 on the arc.
    expect(arcCrossing(FILLET, { x: 8, y: 8 })).toBe(0);
  });

  it("counts −1 for (2, 8): the arc crosses at x = 4, right of it, y falling", () => {
    // Chord: (20, −20) × (2, −12) = −240 + 40 = −200 < 0: −1; |P − C| = √468 ≈ 21.63 > 20.
    expect(arcCrossing(FILLET, { x: 2, y: 8 })).toBe(-1);
  });

  it("counts the chord alone for (15, 15), on the centre's side", () => {
    // (20, −20) × (15, −5) = 200, not of the sign of d = −1: the chord counts 0, and so does the arc.
    expect(arcCrossing(FILLET, { x: 15, y: 15 })).toBe(0);
  });

  it("counts +1 for (2, 8) with the arc travelled the other way, y growing", () => {
    // Chord from (20, 0) to (0, 20): d = +1; (−20, 20) × (−18, 8) = −160 + 360 = 200 > 0: +1.
    expect(arcCrossing(REVERSED, { x: 2, y: 8 })).toBe(1);
  });

  it("counts 0 for (8, 8) with the arc travelled the other way", () => {
    // Chord from (20, 0) to (0, 20): d = +1; (−20, 20) × (−12, 8) = −160 + 240 = 80 > 0: +1.
    // Inside the circular segment, the arc travelled negatively, σ = −1: +1 − 1 = 0.
    expect(arcCrossing(REVERSED, { x: 8, y: 8 })).toBe(0);
  });
});

describe("arcCrossing (properties)", () => {
  test.prop({ x: COORDINATE, y: COORDINATE })("changes sign travelled the other way", (point) => {
    expect(arcCrossing(REVERSED, point) + arcCrossing(FILLET, point)).toBe(0);
  });

  test.prop({ x: COORDINATE, y: COORDINATE })(
    "counts −1 left of the arc and 0 elsewhere on the levels it spans",
    (point) => {
      // Left of the arc on 0 ≤ y < 20, the levels its chord counts: x < 20 − √(400 − (y − 20)²);
      // the arc's own points left out ((8, 4) is one: 20 − √(400 − 256) = 8).
      const left = 20 - Math.sqrt(400 - (point.y - 20) ** 2);

      fc.pre(point.y >= 0 && point.y < 20 && point.x !== left);

      expect(arcCrossing(FILLET, point)).toBe(point.x < left ? -1 : 0);
    },
  );
});
