import { describe, expect, it } from "vitest";

import { isConvertible } from "./is-convertible";
import { rectangleAsFreeShape } from "./rectangle-as-free-shape";

/**
 * A hexagon 100 high.
 *
 * @param width - width of its box
 * @returns a regular polygon of 6 corners, centered
 */
function hexagon(width: number): Parameters<typeof isConvertible>[0] {
  return {
    anchor: { horizontal: "mid", vertical: "mid" },
    angle: 0,
    corners: 6,
    height: 100,
    radii: [0, 0, 0, 0, 0, 0],
    width,
    x: 0,
    y: 0,
  };
}

describe("isConvertible (ADR-0033, ADR-0036)", () => {
  it("[F10.AC3] converts a polygon with a box, and a round shape of any size", () => {
    expect(isConvertible(hexagon(100))).toBe(true);
    expect(isConvertible(hexagon(0))).toBe(false);
    expect(isConvertible({ angle: 0, height: 0, kind: "round", width: 50, x: 0, y: 0 })).toBe(true);
  });

  it("[F10.AC3] never converts a free shape, one already", () => {
    expect(
      isConvertible(
        rectangleAsFreeShape({ angle: 0, height: 50, radii: [0, 0, 0, 0], width: 100, x: 0, y: 0 }),
      ),
    ).toBe(false);
  });
});
