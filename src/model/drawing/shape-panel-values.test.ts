import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { rectangleAsFreeShape } from "../rectangle";

import { shapePanelValues } from "./shape-panel-values";

import type { PanelValues } from "./panel-values";
import type { Rectangle } from "../rectangle";
import type { Shape } from "../shape";

/**
 * A sharp rectangle of the model, stored as a free shape (ADR-0032).
 *
 * @param box - its angle, position and size
 * @returns it, its four corners sharp
 */
function rectangle(box: Omit<Rectangle, "radii">): Shape {
  return rectangleAsFreeShape({ ...box, radii: [0, 0, 0, 0] });
}

/**
 * Asserts each value to 5 decimals, the precision the panel shows them with (Q10).
 *
 * @param actual - values computed
 * @param expected - values computed by hand
 */
function expectValues(actual: PanelValues, expected: PanelValues): void {
  expect(actual.x).toBeCloseTo(expected.x, 5);
  expect(actual.y).toBeCloseTo(expected.y, 5);
  expect(actual.width).toBeCloseTo(expected.width, 5);
  expect(actual.height).toBeCloseTo(expected.height, 5);
}

/** A at (10, 20), 100 × 50, sharp and straight. */
const SHAPE_A = rectangle({ angle: 0, height: 50, width: 100, x: 10, y: 20 });

describe("shapePanelValues (F38)", () => {
  it("[F38.AC1] gives A's box: X 10, Y 20, W 100, H 50", () => {
    expect(shapePanelValues(SHAPE_A)).toEqual({ height: 50, width: 100, x: 10, y: 20 });
  });

  it("[F38.AC1] gives A turned by 30° its straight box's x and y, its own width and height", () => {
    // About its middle (60, 45), half sizes 50 cos 30° + 25 sin 30° = 43.30127 + 12.5 = 55.80127
    // across and 50 sin 30° + 25 cos 30° = 25 + 21.65064 = 46.65064 down:
    // x = 60 − 55.80127 = 4.19873, y = 45 − 46.65064 = −1.65064.
    expectValues(shapePanelValues({ ...SHAPE_A, angle: 30 }), {
      height: 50,
      width: 100,
      x: 4.19873,
      y: -1.65064,
    });
  });

  it("gives a rounded rectangle its box, its arcs reaching its sides", () => {
    // Radius 10 at each corner: the quarter arcs touch the sides at x = 10 and y = 20.
    const rounded = rectangleAsFreeShape({
      angle: 0,
      height: 50,
      radii: [10, 10, 10, 10],
      width: 100,
      x: 10,
      y: 20,
    });

    expectValues(shapePanelValues(rounded), { height: 50, width: 100, x: 10, y: 20 });
  });

  it("gives a circle turned by 45° the box it had straight, a circle turning onto itself", () => {
    // The circle in the box 100 × 100 at (0, 0) turns about its center (50, 50): the same circle.
    const circle: Shape = { angle: 45, height: 100, kind: "round", width: 100, x: 0, y: 0 };

    expectValues(shapePanelValues(circle), { height: 100, width: 100, x: 0, y: 0 });
  });

  it("gives a shape 0 × 80 at (10, 10) its line's x and y", () => {
    expect(shapePanelValues(rectangle({ angle: 0, height: 80, width: 0, x: 10, y: 10 }))).toEqual({
      height: 80,
      width: 0,
      x: 10,
      y: 10,
    });
  });

  test.prop({
    angle: fc.integer({ max: 359, min: 0 }),
    height: fc.integer({ max: 500, min: 1 }),
    width: fc.integer({ max: 500, min: 1 }),
    x: fc.integer({ max: 500, min: -500 }),
    y: fc.integer({ max: 500, min: -500 }),
  })("gives a rectangle its own size, and the same box turned by half a turn more", (box) => {
    const values = shapePanelValues(rectangle(box));
    const halfTurn = shapePanelValues(rectangle({ ...box, angle: (box.angle + 180) % 360 }));

    expect(values.width).toBe(box.width);
    expect(values.height).toBe(box.height);
    // Its straight box holds its middle and lies about it alike after half a turn.
    expect(values.x).toBeLessThanOrEqual(box.x + box.width / 2);
    expect(values.y).toBeLessThanOrEqual(box.y + box.height / 2);
    expect(halfTurn.x).toBeCloseTo(values.x, 6);
    expect(halfTurn.y).toBeCloseTo(values.y, 6);
  });

  test.prop({
    height: fc.integer({ max: 500, min: 0 }),
    width: fc.integer({ max: 500, min: 0 }),
    x: fc.integer({ max: 500, min: -500 }),
    y: fc.integer({ max: 500, min: -500 }),
  })("gives a straight sharp rectangle its own x, y, width and height", (box) => {
    expect(shapePanelValues(rectangle({ ...box, angle: 0 }))).toEqual(box);
  });
});
