import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { expectPoints, integerPoint } from "../../../tests";

import { tensionHandle } from "./tension-handle";

describe("tensionHandle (DERIV-handle-tension)", () => {
  it("[F12.AC4] gives the Catmull–Rom handle at tension 0, none at 100, half at 50", () => {
    // (0, 0) and (200, 100): (200, 100) / 6 = (33.33333, 16.66667).
    expectPoints(
      [
        tensionHandle({ x: 0, y: 0 }, { x: 200, y: 100 }, 0),
        tensionHandle({ x: 0, y: 0 }, { x: 200, y: 100 }, 50),
      ],
      [
        { x: 33.33333, y: 16.66667 },
        { x: 16.66667, y: 8.33333 },
      ],
      5,
    );

    expect(tensionHandle({ x: 0, y: 0 }, { x: 200, y: 100 }, 100)).toEqual({ x: 0, y: 0 });
  });

  test.prop({
    next: integerPoint(1e3),
    previous: integerPoint(1e3),
    tension: fc.integer({ max: 100, min: 0 }),
  })(
    "runs along the line from the point before to the point after",
    ({ next, previous, tension }) => {
      const handle = tensionHandle(previous, next, tension);
      const chord = { x: next.x - previous.x, y: next.y - previous.y };

      expect(handle.x * chord.y - handle.y * chord.x).toBeCloseTo(0, 6);
    },
  );
});
