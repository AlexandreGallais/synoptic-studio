import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { rectangleSides } from "./rectangle-sides";

describe("rectangleSides (Q11)", () => {
  it("gives an underline's four sides clockwise from its top-left corner", () => {
    // Corners (0, 16.484375), (36.4609375, 16.484375), (36.4609375, 17.484375), (0, 17.484375).
    const sides = rectangleSides({ x: 0, y: 16.484375 }, { height: 1, width: 36.4609375 });

    expect(sides.map((side) => side.start)).toEqual([
      { x: 0, y: 16.484375 },
      { x: 36.4609375, y: 16.484375 },
      { x: 36.4609375, y: 17.484375 },
      { x: 0, y: 17.484375 },
    ]);

    expect(sides[3]?.end).toEqual({ x: 0, y: 16.484375 });
  });

  it("gives a rectangle 0 high as sides along a line", () => {
    expect(
      rectangleSides({ x: 2, y: 3 }, { height: 0, width: 10 }).map((side) => side.end),
    ).toEqual([
      { x: 12, y: 3 },
      { x: 12, y: 3 },
      { x: 2, y: 3 },
      { x: 2, y: 3 },
    ]);
  });

  const length = fc.integer({ max: 100_000, min: 0 });
  const coordinate = fc.integer({ max: 100_000, min: -100_000 });

  test.prop({ height: length, width: length, x: coordinate, y: coordinate })(
    "closes, each side starting where the previous one ends, its perimeter 2 (w + h)",
    ({ height, width, x, y }) => {
      const sides = rectangleSides({ x, y }, { height, width });
      const perimeter = sides.reduce(
        (sum, side) =>
          sum + Math.abs(side.end.x - side.start.x) + Math.abs(side.end.y - side.start.y),
        0,
      );

      expect(sides.map((side, index) => sides.at(index - 1)?.end)).toEqual(
        sides.map((side) => side.start),
      );

      expect(perimeter).toBe(2 * (width + height));
    },
  );
});
