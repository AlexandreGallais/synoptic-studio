import { describe, expect, it } from "vitest";

import { rectangleToPathData } from "./rectangle-to-path-data";

describe("rectangleToPathData (REF-SVG2-PATHS)", () => {
  it("writes an underline clockwise from its top-left corner", () => {
    // Corners (0, 16.484375), (36.4609375, 16.484375), (36.4609375, 17.484375), (0, 17.484375),
    // at 5 decimals.
    expect(rectangleToPathData({ height: 1, width: 36.4609375, x: 0, y: 16.484375 })).toBe(
      "M0 16.48438 L36.46094 16.48438 L36.46094 17.48438 L0 17.48438 Z",
    );
  });

  it("writes a rectangle 0 high as the line it is", () => {
    expect(rectangleToPathData({ height: 0, width: 10, x: 2, y: 3 })).toBe(
      "M2 3 L12 3 L12 3 L2 3 Z",
    );
  });
});
