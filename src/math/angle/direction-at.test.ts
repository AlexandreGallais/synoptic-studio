import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { norm } from "../vector";

import { directionAt } from "./direction-at";

describe("directionAt (REF-OPENSTAX-UNIT-CIRCLE)", () => {
  it("is (1, 0) at 0 and almost (0, 1) at π/2", () => {
    expect(directionAt(0)).toEqual({ x: 1, y: 0 });
    // cos(π/2) rounds to 6.123233995736766e-17: why quarter turns are made exactly instead.
    expect(directionAt(Math.PI / 2)).toEqual({ x: 6.123233995736766e-17, y: 1 });
  });

  test.prop({ angle: fc.double({ max: 7, min: -7, noNaN: true }) })("has length 1", ({ angle }) => {
    expect(norm(directionAt(angle))).toBeCloseTo(1, 15);
  });
});
