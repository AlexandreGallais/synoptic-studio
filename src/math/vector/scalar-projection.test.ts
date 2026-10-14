import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { integerPoint } from "../../../tests";
import { quarterTurn } from "../rotation";

import { add } from "./add";
import { scalarProjection } from "./scalar-projection";
import { scale } from "./scale";

describe("scalarProjection (REF-MATHWORLD-SCALAR-PROJECTION)", () => {
  it("measures (3, 4) from the origin along the x axis: 3", () => {
    expect(scalarProjection({ x: 3, y: 4 }, { x: 0, y: 0 }, { x: 1, y: 0 })).toBe(3);
  });

  it("is negative behind the origin", () => {
    expect(scalarProjection({ x: -2, y: 7 }, { x: 1, y: 0 }, { x: 1, y: 0 })).toBe(-3);
  });

  test.prop({
    across: fc.integer({ max: 100, min: -100 }),
    along: fc.integer({ max: 100, min: -100 }),
    angle: fc.double({ max: 7, min: -7, noNaN: true }),
    origin: integerPoint(1e3),
  })(
    "measures the part along the direction, not the part across",
    ({ across, along, angle, origin }) => {
      const u = { x: Math.cos(angle), y: Math.sin(angle) };
      const offset = add(scale(u, along), scale(quarterTurn(u), across));
      const point = add(origin, offset);

      expect(scalarProjection(point, origin, u)).toBeCloseTo(along, 9);
    },
  );
});
