import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { centeredPosition } from "./centered-position";

/** A box 100 × 100, the size a click places (F35). */
const PLACED = { height: 100, width: 100 };

/** A coordinate of a click in eighths of a unit, exact in a double, on a canvas of 5 000 pixels. */
const COORDINATE = fc.integer({ max: 40_000, min: -40_000 }).map((eighths) => eighths / 8);

/** A size of a box in eighths of a unit, exact in a double. */
const SIZE = fc.integer({ max: 40_000, min: 0 }).map((eighths) => eighths / 8);

describe("centeredPosition (F35)", () => {
  it("[F35.AC1] places a box 100 × 100 clicked at (250, 140) at (200, 90)", () => {
    // 250 − 100 / 2 = 200, 140 − 100 / 2 = 90.
    expect(centeredPosition({ x: 250, y: 140 }, PLACED)).toEqual({ x: 200, y: 90 });
  });

  it.each([
    // 250.4 − 50 = 200.4 → 200; 139.6 − 50 = 89.6 → 90.
    { expected: { x: 200, y: 90 }, point: { x: 250.4, y: 139.6 } },
    // 250.5 − 50 = 200.5 and 140.5 − 50 = 90.5: a tie rounds up, toward +∞, to 201 and 91.
    { expected: { x: 201, y: 91 }, point: { x: 250.5, y: 140.5 } },
    // 49.5 − 50 = −0.5: a tie rounds up to 0, never −0; 48.5 − 50 = −1.5 rounds up to −1.
    { expected: { x: 0, y: -1 }, point: { x: 49.5, y: 48.5 } },
  ])(
    "[F35.AC1] lands on integers when $point.x, $point.y is clicked between two integers",
    ({ expected, point }) => {
      const { x, y } = centeredPosition(point, PLACED);

      expect(x).toBe(expected.x);
      expect(y).toBe(expected.y);
    },
  );

  it("[F35.AC1] centres a text's measured block, its size fractional", () => {
    // 100 − 87.3 / 2 = 56.35 → 56; 50 − 38.4 / 2 = 30.8 → 31.
    expect(centeredPosition({ x: 100, y: 50 }, { height: 38.4, width: 87.3 })).toEqual({
      x: 56,
      y: 31,
    });
  });

  it("places a box of no size on the point, rounded", () => {
    // 3.5 rounds up to 4, 2.2 down to 2.
    expect(centeredPosition({ x: 3.5, y: 2.2 }, { height: 0, width: 0 })).toEqual({ x: 4, y: 2 });
  });

  test.prop({ height: SIZE, width: SIZE, x: COORDINATE, y: COORDINATE })(
    "gives integers whose box's centre lies within half a unit of the point",
    ({ height, width, x, y }) => {
      const position = centeredPosition({ x, y }, { height, width });

      expect([Number.isSafeInteger(position.x), Number.isSafeInteger(position.y)]).toEqual([
        true,
        true,
      ]);

      expect(Math.abs(position.x + width / 2 - x)).toBeLessThanOrEqual(0.5);
      expect(Math.abs(position.y + height / 2 - y)).toBeLessThanOrEqual(0.5);
    },
  );

  test.prop({ shift: fc.integer({ max: 1000, min: -1000 }), x: COORDINATE, y: COORDINATE })(
    "moves by the same integer as the point",
    ({ shift, x, y }) => {
      const position = centeredPosition({ x, y }, PLACED);
      const moved = centeredPosition({ x: x + shift, y: y + shift }, PLACED);

      expect([moved.x - position.x, moved.y - position.y]).toEqual([shift, shift]);
    },
  );
});
