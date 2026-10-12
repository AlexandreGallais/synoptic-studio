import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { commonRadius } from "./common-radius";

describe("commonRadius", () => {
  it("[F03.AC2] gives the radius shared by every vertex", () => {
    expect(commonRadius([10, 10, 10, 10])).toBe(10);
    expect(commonRadius([0, 0, 0])).toBe(0);
  });

  it("[F03.AC2] gives nothing when the radii differ: the field shows Mixed", () => {
    expect(commonRadius([10, 0, 0, 0])).toBeUndefined();
    expect(commonRadius([0, 0, 0, 1])).toBeUndefined();
  });

  it("gives nothing for a shape without vertices", () => {
    expect(commonRadius([])).toBeUndefined();
  });

  test.prop([fc.nat({ max: 1000 }), fc.integer({ max: 12, min: 2 }), fc.nat({ max: 11 })])(
    "gives the radius of a shape rounded alike, and nothing once one of its vertices differs",
    (radius, count, vertex) => {
      const radii = Array.from({ length: count }, () => radius);

      expect(commonRadius(radii)).toBe(radius);
      expect(commonRadius(radii.with(vertex % count, radius + 1))).toBeUndefined();
    },
  );
});
