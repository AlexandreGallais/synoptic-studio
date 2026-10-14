import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { integerPoint } from "../../../tests";
import { norm } from "../vector";

import { rotate } from "./rotate";

describe("rotate (REF-MATHWORLD-ROTATION-MATRIX)", () => {
  it("turns (1, 0) by 60°", () => {
    // (cos 60°, sin 60°) = (0.5, √3/2): (1 · 0.5 − 0, 1 · √3/2 + 0).
    const turned = rotate({ x: 1, y: 0 }, { x: 0.5, y: Math.sqrt(3) / 2 });

    expect(turned.x).toBeCloseTo(0.5, 15);
    expect(turned.y).toBeCloseTo(0.8660254037844386, 15);
  });

  it("leaves a vector as it is for the direction (1, 0)", () => {
    expect(rotate({ x: 3, y: -4 }, { x: 1, y: 0 })).toEqual({ x: 3, y: -4 });
  });

  test.prop({ angle: fc.double({ max: 7, min: -7, noNaN: true }), v: integerPoint(1e4) })(
    "keeps the length",
    ({ angle, v }) => {
      const direction = { x: Math.cos(angle), y: Math.sin(angle) };

      expect(norm(rotate(v, direction))).toBeCloseTo(norm(v), 6);
    },
  );
});
