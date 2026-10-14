import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { directionAt } from "./direction-at";
import { signedAngle } from "./signed-angle";

describe("signedAngle (DERIV-turning-angle)", () => {
  it.each([
    { expected: Math.PI / 2, to: { x: 0, y: 1 } },
    { expected: Math.PI, to: { x: -1, y: 0 } },
    { expected: -Math.PI / 2, to: { x: 0, y: -1 } },
    { expected: 0, to: { x: 0, y: 0 } },
  ])("from (1, 0) to ($to.x, $to.y) is $expected", ({ expected, to }) => {
    expect(signedAngle({ x: 1, y: 0 }, to)).toBe(expected);
  });

  it("is π from (−1, −1) to a zero vector: atan2(+0, −0)", () => {
    expect(signedAngle({ x: -1, y: -1 }, { x: 0, y: 0 })).toBe(Math.PI);
  });

  test.prop({
    a: fc.double({ max: 3, min: -3, noNaN: true }),
    b: fc.double({ max: 3, min: -3, noNaN: true }),
  })("is the difference of the two directions' angles, turned into [−π, π]", ({ a, b }) => {
    const turn = signedAngle(directionAt(a), directionAt(b));

    expect(Math.cos(turn)).toBeCloseTo(Math.cos(b - a), 9);
    expect(Math.sin(turn)).toBeCloseTo(Math.sin(b - a), 9);
  });
});
