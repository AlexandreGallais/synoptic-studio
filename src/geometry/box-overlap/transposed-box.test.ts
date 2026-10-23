import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { transposedBox } from "./transposed-box";

describe("transposedBox (DERIV-box-overlap step 8)", () => {
  it("exchanges the intervals of the two axes", () => {
    // [0, 10] × [−30, −20] becomes [−30, −20] × [0, 10].
    expect(transposedBox({ maxX: 10, maxY: -20, minX: 0, minY: -30 })).toEqual({
      maxX: -20,
      maxY: 10,
      minX: -30,
      minY: 0,
    });
  });
});

describe("transposedBox (properties)", () => {
  test.prop({ maxX: fc.integer(), maxY: fc.integer(), minX: fc.integer(), minY: fc.integer() })(
    "gives the box back when applied twice",
    (box) => {
      expect(transposedBox(transposedBox(box))).toEqual(box);
    },
  );
});
