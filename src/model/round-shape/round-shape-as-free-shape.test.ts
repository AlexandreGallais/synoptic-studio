import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { freeShapeCorners, isValidFreeShape } from "../free-shape";
import { turnedCorners } from "../placement";

import { roundShapeAsFreeShape } from "./round-shape-as-free-shape";
import { roundShapeCorners } from "./round-shape-corners";

describe("roundShapeAsFreeShape (F10.AC4, ADR-0048)", () => {
  it("[F10.AC4] gives the 4 points of an ellipse 120 × 80 with their handles, its box unchanged", () => {
    const shape = { angle: 0, height: 80, kind: "round", width: 120, x: 5, y: 7 } as const;
    const free = roundShapeAsFreeShape(shape);

    expect(free.corners).toEqual(roundShapeCorners(shape));
    expect(free.width).toBeCloseTo(120, 9);
    expect(free.height).toBeCloseTo(80, 9);
    expect(free.x).toBeCloseTo(5, 9);
    expect(free.y).toBeCloseTo(7, 9);
  });

  test.prop({
    angle: fc.integer({ max: 359, min: 0 }),
    height: fc.integer({ max: 1000, min: 1 }),
    width: fc.integer({ max: 1000, min: 1 }),
  })(
    "[F10.AC4] does not move a turned shape: its corners are drawn where they were",
    ({ angle, height, width }) => {
      const shape = { angle, height, kind: "round", width, x: 3, y: -4 } as const;
      const before = turnedCorners(roundShapeCorners(shape), shape);
      const free = roundShapeAsFreeShape(shape);
      const after = turnedCorners(freeShapeCorners(free), free);

      const gaps = after.map((corner, index) => {
        const drawn = before[index] ?? corner;

        return Math.max(
          Math.hypot(corner.point.x - drawn.point.x, corner.point.y - drawn.point.y),
          Math.hypot(
            corner.handles.after.x - drawn.handles.after.x,
            corner.handles.after.y - drawn.handles.after.y,
          ),
        );
      });

      expect(Math.max(...gaps)).toBeLessThan(1e-6);
      expect(free.angle).toBe(angle);
    },
  );

  it("gives no valid free shape for a box of width 0: it is not converted (F08)", () => {
    expect(
      isValidFreeShape(
        roundShapeAsFreeShape({ angle: 0, height: 50, kind: "round", width: 0, x: 0, y: 0 }),
      ),
    ).toBe(false);
  });
});
