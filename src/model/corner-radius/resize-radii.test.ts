import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { resizeRadii } from "./resize-radii";

describe("resizeRadii", () => {
  it("[F03.AC3] keeps the radii by vertex number when a hexagon gets fewer or more corners", () => {
    expect(resizeRadii([10, 0, 0, 0, 0, 0], 5)).toEqual([10, 0, 0, 0, 0]);
    expect(resizeRadii([10, 0, 0, 0, 0, 0], 8)).toEqual([10, 0, 0, 0, 0, 0, 0, 0]);
  });

  it("[F03.AC3] takes the radius of a removed vertex away, and gives a new vertex 0", () => {
    // Vertex 6 loses its radius at 5 corners; back at 6, the new vertex 6 is sharp.
    expect(resizeRadii(resizeRadii([1, 2, 3, 4, 5, 6], 5), 6)).toEqual([1, 2, 3, 4, 5, 0]);
  });

  it("keeps the same number unchanged", () => {
    expect(resizeRadii([3, 1, 4, 1], 4)).toEqual([3, 1, 4, 1]);
  });

  test.prop([fc.array(fc.nat({ max: 1000 }), { maxLength: 12 }), fc.nat({ max: 12 })])(
    "gives `count` radii, the first ones kept, the others 0",
    (radii, count) => {
      const resized = resizeRadii(radii, count);

      expect(resized).toHaveLength(count);
      expect(resized.slice(0, radii.length)).toEqual(radii.slice(0, count));
      expect(resized.slice(radii.length).every((radius) => radius === 0)).toBe(true);
    },
  );
});
