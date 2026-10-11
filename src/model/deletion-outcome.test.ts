import { describe, expect, it } from "vitest";

import { deletionOutcome } from "./deletion-outcome";
import { freeShapeWithMidpoint } from "./free-shape-with-midpoint";
import { rectangleAsFreeShape } from "./rectangle-as-free-shape";

/** A 100 square, sharp. */
const SQUARE = rectangleAsFreeShape({ height: 100, radii: [0, 0, 0, 0], width: 100 });

describe("deletionOutcome", () => {
  it("[F08.AC4] allows deleting one point of a square: a triangle is left", () => {
    expect(deletionOutcome(SQUARE, [2])).toBe("allowed");
  });

  it.each([{ removed: [0, 1] }, { removed: [0, 1, 2, 3] }, { removed: [3, 1] }])(
    "[F08.AC4] refuses deleting $removed of a square: fewer than 3 points left",
    ({ removed }) => {
      expect(deletionOutcome(SQUARE, removed)).toBe("too-few");
    },
  );

  it("[F08.AC4] refuses a deletion that leaves the points on one horizontal line", () => {
    // (0, 0), (50, 0), (100, 0), (100, 100), (0, 100): without the last two, three points at y 0.
    const five = freeShapeWithMidpoint(SQUARE, 0);

    expect(deletionOutcome(five, [3, 4])).toBe("flat");
  });
});
