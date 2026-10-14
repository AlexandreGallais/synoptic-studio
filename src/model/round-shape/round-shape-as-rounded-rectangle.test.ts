import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { roundShapeAsRoundedRectangle } from "./round-shape-as-rounded-rectangle";

describe("roundShapeAsRoundedRectangle (F10.AC5, DERIV-round-shape)", () => {
  it("[F10.AC5] rounds the 4 corners of a box 120 × 80 by 40: a capsule", () => {
    const free = roundShapeAsRoundedRectangle({
      angle: 30,
      height: 80,
      kind: "round",
      width: 120,
      x: 5,
      y: 7,
    });

    expect(free.corners.map((corner) => corner.radius)).toEqual([40, 40, 40, 40]);
    expect(free).toMatchObject({ angle: 30, height: 80, width: 120, x: 5, y: 7 });
  });

  it("[F10.AC5] rounds a box 81 × 81 by 41, half its side rounded up (step 2)", () => {
    const free = roundShapeAsRoundedRectangle({
      angle: 0,
      height: 81,
      kind: "round",
      width: 81,
      x: 0,
      y: 0,
    });

    expect(free.corners.map((corner) => corner.radius)).toEqual([41, 41, 41, 41]);
  });

  test.prop({
    height: fc.integer({ max: 1000, min: 1 }),
    width: fc.integer({ max: 1000, min: 1 }),
  })("gives integer radii of at least half the smaller side, handles off", ({ height, width }) => {
    const free = roundShapeAsRoundedRectangle({
      angle: 0,
      height,
      kind: "round",
      width,
      x: 0,
      y: 0,
    });

    for (const corner of free.corners) {
      expect(Number.isSafeInteger(corner.radius)).toBe(true);
      expect(corner.radius * 2).toBeGreaterThanOrEqual(Math.min(width, height));
      expect(corner.handles.isOn).toBe(false);
    }
  });
});
