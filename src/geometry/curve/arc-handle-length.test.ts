import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { cubicPoint } from "../placement";

import { arcHandleLength } from "./arc-handle-length";

describe("arcHandleLength (REF-POMAX-BEZIER)", () => {
  it("is 4(√2 − 1)/3 ≈ 0.5522847 for a quarter circle", () => {
    // tan(π/8) = √2 − 1 (DERIV-round-shape-curve step 1).
    expect(arcHandleLength(Math.PI / 2)).toBeCloseTo((4 * (Math.SQRT2 - 1)) / 3, 15);
  });

  it("is 0 for an arc of angle 0", () => {
    expect(arcHandleLength(0)).toBe(0);
  });

  test.prop({ angle: fc.double({ max: Math.PI, min: 0.01, noNaN: true }) })(
    "puts the middle of the cubic on the arc",
    ({ angle }) => {
      // The arc from angle 0 to θ on the unit circle; its handles along the tangents.
      const k = arcHandleLength(angle);
      const end = { x: Math.cos(angle), y: Math.sin(angle) };
      const middle = cubicPoint(
        {
          end,
          first: { x: 1, y: k },
          kind: "cubic",
          second: { x: end.x + k * Math.sin(angle), y: end.y - k * Math.cos(angle) },
          start: { x: 1, y: 0 },
        },
        1 / 2,
      );

      expect(Math.hypot(middle.x, middle.y)).toBeCloseTo(1, 12);
    },
  );
});
