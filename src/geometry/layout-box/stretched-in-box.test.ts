import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { integerPoint } from "../../../tests";
import { boundingBox } from "../placement";

import { stretchToSize } from "./stretch-to-size";
import { stretchedInBox } from "./stretched-in-box";

describe("stretchedInBox (DERIV-free-shape-stretch)", () => {
  it("[F12.AC6] stretches the points so that the curve's box takes the size", () => {
    // Box (0, −30) – (100, 100), 130 high, made 100 high: y = −30 + 100 (y + 30) / 130.
    const points = [
      { x: 0, y: 0 },
      { x: 100, y: 100 },
    ];
    const stretched = stretchedInBox(
      points,
      { maxX: 100, maxY: 100, minX: 0, minY: -30 },
      { height: 100, width: 100 },
    );

    expect(stretched[0]?.y).toBeCloseTo(-30 + 3000 / 130, 9);
    expect(stretched[1]).toEqual({ x: 100, y: 70 });
  });

  test.prop({ points: fc.array(integerPoint(1e3), { minLength: 2 }), size: integerPoint(1e3) })(
    "is stretchToSize when the box is the points' own",
    ({ points, size }) => {
      const asked = { height: Math.abs(size.y), width: Math.abs(size.x) };

      expect(stretchedInBox(points, boundingBox(points), asked)).toEqual(
        stretchToSize(points, asked),
      );
    },
  );
});
