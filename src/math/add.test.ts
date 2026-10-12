import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { add } from "./add";
import { subtract } from "./subtract";

/** Point with integer coordinates. */
const POINT = fc.record({
  x: fc.integer({ max: 1e6, min: -1e6 }),
  y: fc.integer({ max: 1e6, min: -1e6 }),
});

describe("add (REF-MATHWORLD-VECTOR-ADDITION)", () => {
  it("adds component-wise", () => {
    // (1 + 3, 2 + (−4)) = (4, −2).
    expect(add({ x: 1, y: 2 }, { x: 3, y: -4 })).toEqual({ x: 4, y: -2 });
  });

  test.prop({ a: POINT, b: POINT })(
    "moves a point onto another by their difference",
    ({ a, b }) => {
      // a + (b − a) = b, exact on integers.
      expect(add(a, subtract(b, a))).toEqual(b);
    },
  );
});
