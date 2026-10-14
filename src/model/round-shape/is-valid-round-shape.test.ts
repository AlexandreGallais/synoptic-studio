import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { isValidRoundShape } from "./is-valid-round-shape";

/** A circle of diameter 100 at the origin. */
const CIRCLE = { angle: 0, height: 100, kind: "round", width: 100, x: 0, y: 0 } as const;

describe("isValidRoundShape (Q15, ADR-0035)", () => {
  it("accepts a circle, an ellipse and a box of width 0", () => {
    expect(isValidRoundShape(CIRCLE)).toBe(true);
    expect(isValidRoundShape({ ...CIRCLE, width: 120 })).toBe(true);
    expect(isValidRoundShape({ ...CIRCLE, width: 0 })).toBe(true);
  });

  it.each([
    { field: "width", value: -1 },
    { field: "height", value: 2.5 },
    { field: "angle", value: 360 },
    { field: "x", value: NaN },
  ])("refuses $field = $value", ({ field, value }) => {
    expect(isValidRoundShape({ ...CIRCLE, [field]: value })).toBe(false);
  });

  test.prop({ height: fc.nat(100_000), width: fc.nat(100_000) })(
    "accepts every integer size ≥ 0",
    ({ height, width }) => {
      expect(isValidRoundShape({ ...CIRCLE, height, width })).toBe(true);
    },
  );
});
