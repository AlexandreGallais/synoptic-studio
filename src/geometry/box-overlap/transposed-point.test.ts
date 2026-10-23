import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { transposedPoint } from "./transposed-point";

describe("transposedPoint (DERIV-box-overlap step 8)", () => {
  it("exchanges the coordinates", () => {
    expect(transposedPoint({ x: 3, y: -7 })).toEqual({ x: -7, y: 3 });
  });

  it("keeps a point of the diagonal", () => {
    expect(transposedPoint({ x: 5, y: 5 })).toEqual({ x: 5, y: 5 });
  });
});

describe("transposedPoint (properties)", () => {
  test.prop({ x: fc.integer(), y: fc.integer() })(
    "gives the point back when applied twice",
    (point) => {
      expect(transposedPoint(transposedPoint(point))).toEqual(point);
    },
  );
});
