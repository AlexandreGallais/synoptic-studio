import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { expectPoints } from "../../../tests";
import { norm } from "../../math";

import { angledHandle } from "./angled-handle";

describe("angledHandle (DERIV-handle-angle)", () => {
  it("[F12.AC4] counts the angle clockwise on screen from the x axis", () => {
    // 330°, 30: (30 cos 330°, 30 sin 330°) = (25.98076, −15); 90°, 10: (0, 10).
    expectPoints(
      [angledHandle({ x: 1, y: 0 }, 330, 30), angledHandle({ x: 1, y: 0 }, 90, 10)],
      [
        { x: 25.98076, y: -15 },
        { x: 0, y: 10 },
      ],
      5,
    );
  });

  it("[F12.AC4] counts it from a side going down the same way", () => {
    // Side (0, 1), 330°, 30: (15, 25.98076).
    expectPoints([angledHandle({ x: 0, y: 1 }, 330, 30)], [{ x: 15, y: 25.98076 }], 5);
  });

  test.prop({ angle: fc.integer({ max: 359, min: 0 }), length: fc.integer({ max: 1000, min: 0 }) })(
    "gives a handle of the length typed",
    ({ angle, length }) => {
      expect(norm(angledHandle({ x: 0.6, y: 0.8 }, angle, length))).toBeCloseTo(length, 9);
    },
  );
});
