import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { integerPoint } from "../../../tests";
import { dot, norm } from "../../math";

import { acrossSide } from "./across-side";

describe("acrossSide (DERIV-handle-share)", () => {
  it("[F12.AC4] points above a side going right, from either end", () => {
    expect(acrossSide({ isBefore: false, point: { x: 0, y: 0 }, side: { x: 100, y: 0 } })).toEqual({
      x: 0,
      y: -100,
    });

    expect(acrossSide({ isBefore: true, point: { x: 0, y: 0 }, side: { x: -100, y: 0 } })).toEqual({
      x: 0,
      y: -100,
    });
  });

  test.prop({ isBefore: fc.boolean(), side: integerPoint(1e3) })(
    "is as long as the side and square to it",
    ({ isBefore, side }) => {
      const across = acrossSide({ isBefore, point: { x: 0, y: 0 }, side });

      expect(norm(across)).toBeCloseTo(norm(side), 9);
      expect(dot(across, side)).toBeCloseTo(0, 9);
    },
  );
});
