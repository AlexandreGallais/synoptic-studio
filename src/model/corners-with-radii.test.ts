import { describe, expect, it } from "vitest";

import { cornersWithRadii } from "./corners-with-radii";

describe("cornersWithRadii", () => {
  it("[F03.AC1] gives each vertex the radius of the same number, in contour order", () => {
    const points = [
      { x: 0, y: 0 },
      { x: 100, y: 0 },
      { x: 100, y: 100 },
      { x: 0, y: 100 },
    ];

    expect(cornersWithRadii(points, [50, 0, 0, 0])).toEqual([
      { point: { x: 0, y: 0 }, radius: 50 },
      { point: { x: 100, y: 0 }, radius: 0 },
      { point: { x: 100, y: 100 }, radius: 0 },
      { point: { x: 0, y: 100 }, radius: 0 },
    ]);
  });

  it("gives no corner to no vertex", () => {
    expect(cornersWithRadii([], [])).toEqual([]);
  });
});
