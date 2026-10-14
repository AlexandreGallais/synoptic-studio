import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { integerPoint } from "../../../tests";
import { EPSILON } from "../../math";

import { hasLength } from "./has-length";

import type { Cubic } from "../contour";

/**
 * A cubic from (0, 0), its control points on the y axis, its end on the x axis.
 *
 * @param first - y of the first control point
 * @param second - y of the second
 * @param end - x of the end
 * @returns the cubic piece
 */
function cubic(first: number, second: number, end: number): Cubic {
  return {
    end: { x: end, y: 0 },
    first: { x: 0, y: first },
    kind: "cubic",
    second: { x: 0, y: second },
    start: { x: 0, y: 0 },
  };
}

describe("hasLength (EPSILON, Q10)", () => {
  it("keeps a piece whose ends are exactly EPSILON apart", () => {
    expect(hasLength({ end: { x: EPSILON, y: 0 }, kind: "segment", start: { x: 0, y: 0 } })).toBe(
      true,
    );
  });

  it("drops a piece whose ends are closer than EPSILON", () => {
    expect(
      hasLength({ end: { x: EPSILON / 2, y: 0 }, kind: "segment", start: { x: 0, y: 0 } }),
    ).toBe(false);
  });

  it("drops the degenerate arc of a corner without fillet", () => {
    const vertex = { x: 50, y: 0 };

    expect(
      hasLength({
        center: vertex,
        end: vertex,
        kind: "arc",
        positive: false,
        radius: 30,
        start: vertex,
      }),
    ).toBe(false);
  });
});

/** Arbitrary point with integer coordinates. */
const POINT = integerPoint(1e6);

/** A step of −1, 0 or 1 along an axis. */
const STEP = fc.integer({ max: 1, min: -1 });

describe("hasLength (properties)", () => {
  test.prop({ dx: STEP, dy: STEP, start: POINT })(
    "keeps a segment between integer points exactly when they differ, whichever way it runs",
    ({ dx, dy, start }) => {
      // Steps of −1, 0 or 1 make the two ends equal about one time in nine.
      const end = { x: start.x + dx, y: start.y + dy };
      // Two integer points that differ are at least 1 apart, far above EPSILON.
      const isDrawn = start.x !== end.x || start.y !== end.y;

      expect(hasLength({ end, kind: "segment", start })).toBe(isDrawn);
      expect(hasLength({ end: start, kind: "segment", start: end })).toBe(isDrawn);
    },
  );

  it("[F12.AC3] gives a cubic length when a control point leaves its start, a loop too", () => {
    expect(hasLength(cubic(-40, -40, 0))).toBe(true);
    expect(hasLength(cubic(0, 0, 100))).toBe(true);
    expect(hasLength(cubic(0, 0, 0))).toBe(false);
  });
});
