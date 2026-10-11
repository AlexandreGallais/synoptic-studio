import { describe, expect, it } from "vitest";

import { boxToViewBox } from "./box-to-view-box";

/** A 400 × 300 canvas. */
const CANVAS = { height: 300, width: 400 };

describe("boxToViewBox", () => {
  it("keeps the origin a margin from the top-left for a shape at positive coordinates", () => {
    expect(boxToViewBox({ maxX: 100, maxY: 50, minX: 0, minY: 0 }, CANVAS, 10)).toEqual({
      height: 300,
      width: 400,
      x: -10,
      y: -10,
    });
  });

  it("ignores a rounding error just below 0, on both axes", () => {
    // −1.8e-15 + 1e-9 > 0: floor gives 0.
    const view = boxToViewBox({ maxX: 100, maxY: 100, minX: -1.8e-15, minY: -1.8e-15 }, CANVAS, 10);

    expect(view.x).toBe(-10);
    expect(view.y).toBe(-10);
  });

  it("keeps the origin for a shape placed at a positive position", () => {
    // min(0, 30) = 0: the view does not follow the shape away from the origin.
    const view = boxToViewBox({ maxX: 130, maxY: 90, minX: 30, minY: 40 }, CANVAS, 10);

    expect(view.x).toBe(-10);
    expect(view.y).toBe(-10);
  });

  it("[F04.AC3] shows a turned shape with negative coordinates, from a whole unit", () => {
    // A 100 × 50 rectangle turned by 90°: x from 25, y from −25; a 100 square at 45°: x from
    // −20.71068, floored to −21.
    expect(boxToViewBox({ maxX: 75, maxY: 75, minX: 25, minY: -25 }, CANVAS, 10).y).toBe(-35);

    expect(
      boxToViewBox({ maxX: 120.7, maxY: 120.7, minX: -20.71068, minY: -20.71068 }, CANVAS, 10).x,
    ).toBe(-31);
  });
});
