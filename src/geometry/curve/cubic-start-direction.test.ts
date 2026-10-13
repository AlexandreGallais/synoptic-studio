import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { cubicThrough, integerPoint } from "../../../tests";
import { norm } from "../../math";

import { cubicStartDirection } from "./cubic-start-direction";

describe("cubicStartDirection (DERIV-fillet-beside-curve step 1)", () => {
  it("leaves along the first handle, else towards the second control point, else the end", () => {
    const origin = { x: 0, y: 0 };

    expect(
      cubicStartDirection(cubicThrough([origin, { x: 3, y: 4 }, { x: 9, y: 9 }, { x: 9, y: 0 }])),
    ).toEqual({ x: 0.6, y: 0.8 });

    expect(
      cubicStartDirection(cubicThrough([origin, origin, { x: 0, y: 5 }, { x: 9, y: 0 }])),
    ).toEqual({
      x: 0,
      y: 1,
    });

    expect(cubicStartDirection(cubicThrough([origin, origin, origin, { x: -2, y: 0 }]))).toEqual({
      x: -1,
      y: 0,
    });

    expect(cubicStartDirection(cubicThrough([origin, origin, origin, origin]))).toEqual({
      x: 0,
      y: 0,
    });
  });

  test.prop({
    points: fc.tuple(integerPoint(50), integerPoint(50), integerPoint(50), integerPoint(50)),
  })("is a unit vector, or none for a cubic without length", ({ points }) => {
    const length = norm(cubicStartDirection(cubicThrough(points)));
    const [start] = points;
    const isPoint = points.every((point) => point.x === start.x && point.y === start.y);

    expect(length).toBeCloseTo(isPoint ? 0 : 1, 12);
  });
});
