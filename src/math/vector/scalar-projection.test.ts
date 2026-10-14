import { test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { integerPoint } from "../../../tests";

import { scalarProjection } from "./scalar-projection";

describe("scalarProjection (REF-MATHWORLD-SCALAR-PROJECTION)", () => {
  it("measures (3, 4) from the origin along the x axis: 3", () => {
    expect(scalarProjection({ x: 3, y: 4 }, { x: 0, y: 0 }, { x: 1, y: 0 })).toBe(3);
  });

  it("is negative behind the origin", () => {
    expect(scalarProjection({ x: -2, y: 7 }, { x: 1, y: 0 }, { x: 1, y: 0 })).toBe(-3);
  });

  test.prop({ origin: integerPoint(1e3), point: integerPoint(1e3) })(
    "is 0 across a line's direction turned a quarter",
    ({ origin, point }) => {
      // Along (0, 1), the x offset does not count.
      expect(scalarProjection(point, origin, { x: 0, y: 1 })).toBe(point.y - origin.y);
    },
  );
});
