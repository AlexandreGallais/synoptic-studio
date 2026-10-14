import { test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { integerPoint } from "../../../tests";

import { isZeroVector } from "./is-zero-vector";
import { norm } from "./norm";

describe("isZeroVector (REF-OPENSTAX-CALC3-VECTORS)", () => {
  it.each([
    { isZero: true, v: { x: 0, y: -0 } },
    { isZero: false, v: { x: 5e-324, y: 0 } },
    { isZero: false, v: { x: 0, y: 5e-324 } },
    { isZero: false, v: { x: NaN, y: 0 } },
  ])("is $isZero for ($v.x, $v.y)", ({ isZero, v }) => {
    expect(isZeroVector(v)).toBe(isZero);
  });

  test.prop({ v: integerPoint(1e4) })("is true exactly when the length is 0", ({ v }) => {
    expect(isZeroVector(v)).toBe(norm(v) === 0);
  });
});
