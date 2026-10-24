import { describe, expect, it } from "vitest";

import { rectangleAsFreeShape } from "../rectangle";

import { pickAreasBox } from "./pick-areas-box";
import { shapePickArea } from "./shape-pick-area";

/** A, 100 × 50 at (10, 20), sharp and straight: its box from (10, 20) to (110, 70). */
const AREA_A = shapePickArea(
  rectangleAsFreeShape({ angle: 0, height: 50, radii: [0, 0, 0, 0], width: 100, x: 10, y: 20 }),
);

/** B, 100 × 50 at (200, 20), sharp, turned by 30° about its middle (250, 45). */
const AREA_B = shapePickArea(
  rectangleAsFreeShape({ angle: 30, height: 50, radii: [0, 0, 0, 0], width: 100, x: 200, y: 20 }),
);

describe("pickAreasBox (US-072)", () => {
  it("[F39.AC2] gives A and B the dashed box from (10, −1.65064) to (305.80127, 91.65064)", () => {
    // B's half sizes 50 cos 30° + 25 sin 30° = 55.80127 across and 50 sin 30° + 25 cos 30° =
    // 46.65064 down about (250, 45): from (194.19873, −1.65064) to (305.80127, 91.65064).
    const box = pickAreasBox([AREA_A, AREA_B]);

    expect(box.minX).toBeCloseTo(10, 9);
    expect(box.minY).toBeCloseTo(-1.65064, 5);
    expect(box.maxX).toBeCloseTo(305.80127, 5);
    expect(box.maxY).toBeCloseTo(91.65064, 5);
  });

  it("[F39.AC2] gives one shape its own straight box", () => {
    expect(pickAreasBox([AREA_A])).toEqual({ maxX: 110, maxY: 70, minX: 10, minY: 20 });
  });

  it("leaves out a shape offering no corner, a text never measured", () => {
    const none = { corners: [], isFlat: false, pieces: [] };

    expect(pickAreasBox([none, AREA_A])).toEqual({ maxX: 110, maxY: 70, minX: 10, minY: 20 });
  });

  it("gives no shape the empty box, its sides infinite", () => {
    expect(pickAreasBox([])).toEqual({
      maxX: -Infinity,
      maxY: -Infinity,
      minX: Infinity,
      minY: Infinity,
    });
  });
});
