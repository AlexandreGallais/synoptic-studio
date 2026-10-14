import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { bentRectangle } from "../../../tests";

import { cornerSides } from "./corner-sides";

describe("cornerSides (DERIV-fillet-beside-curve step 0)", () => {
  it("writes each side from its corner: the curve into point 4 reversed, the straight sides at thirds", () => {
    // Point 4 (0, 80): the curve from point 3 comes into it, reversed so that it starts at (0, 80).
    const fourth = cornerSides(bentRectangle(40, [20, 20, 20, 20]))[3];

    expect(fourth?.[0]).toMatchObject({
      cubic: { end: { x: 120, y: 80 }, start: { x: 0, y: 80 } },
      isStraight: false,
    });

    expect(fourth?.[1]).toMatchObject({ isStraight: true, length: 80, limit: 1 });
    expect(fourth?.[1].cubic.first.y).toBeCloseTo(80 - 80 / 3, 12);
  });

  test.prop({ handle: fc.integer({ max: 80, min: 0 }) })(
    "gives every corner two sides starting at its point",
    ({ handle }) => {
      const corners = bentRectangle(handle, [0, 0, 0, 0]);

      for (const [index, pair] of cornerSides(corners).entries()) {
        expect(pair.map((side) => side.cubic.start)).toEqual([
          corners[index]?.point,
          corners[index]?.point,
        ]);
      }
    },
  );
});
