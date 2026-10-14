import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { integerPoint } from "../../../tests";
import { pointBetween } from "../../math";
import { cubicPoint } from "../placement";

import { straightCubic } from "./straight-cubic";

describe("straightCubic (DERIV-fillet-beside-curve)", () => {
  it("puts its control points at the thirds", () => {
    expect(straightCubic({ x: 0, y: 0 }, { x: 30, y: -60 })).toMatchObject({
      first: { x: 10, y: -20 },
      second: { x: 20, y: -40 },
    });
  });

  test.prop({
    end: integerPoint(500),
    start: integerPoint(500),
    t: fc.double({ max: 1, min: 0, noNaN: true }),
  })("runs at constant speed: the point at t is t of the way", ({ end, start, t }) => {
    const point = cubicPoint(straightCubic(start, end), t);

    expect(point.x).toBeCloseTo(pointBetween(start, end, t).x, 9);
    expect(point.y).toBeCloseTo(pointBetween(start, end, t).y, 9);
  });
});
