import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { bentRectangle } from "../../../tests";

import { cornerAsks } from "./corner-asks";

describe("cornerAsks (DERIV-fillet-beside-curve)", () => {
  it("asks no radius at a point in handles mode, and a fillet only beside a curve", () => {
    const asks = cornerAsks(bentRectangle(40, [20, 20, 20, 20]));

    expect(asks.map((ask) => ask.radius)).toEqual([20, 20, 0, 20]);
    expect(asks.map((ask) => ask.fillet === undefined)).toEqual([true, false, false, false]);
  });

  test.prop({ radius: fc.integer({ max: 60, min: 0 }) })(
    "turns each corner by a quarter turn, as a rectangle",
    ({ radius }) => {
      const asks = cornerAsks(bentRectangle(0, [radius, radius, radius, radius]));

      expect(asks.map((ask) => Math.abs(ask.angle))).toEqual(
        Array.from({ length: 4 }, () => Math.PI / 2),
      );
    },
  );
});
