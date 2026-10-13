import { test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { integerPoint } from "../../../tests";

import { cubicSide } from "./cubic-side";

import type { Point, Vector } from "../../math";
import type { CurveCorner } from "../handles";

/**
 * A corner in handles mode.
 *
 * @param point - its point
 * @param before - handle towards the previous point
 * @param after - handle towards the next point
 * @returns the corner, radius 0
 */
function corner(point: Point, before: Vector, after: Vector): CurveCorner {
  return { handles: { after, before, isOn: true, mode: "none", tension: 0 }, point, radius: 0 };
}

describe("cubicSide (REF-SVG2-PATHS)", () => {
  it("[F12.AC3] runs from A through A + after and B + before to B", () => {
    // A (100, 0), after (30, −15); B (200, 0), before (0, 0): C130 −15 200 0 200 0.
    expect(
      cubicSide(
        corner({ x: 100, y: 0 }, { x: -30, y: 15 }, { x: 30, y: -15 }),
        corner({ x: 200, y: 0 }, { x: 0, y: 0 }, { x: 7, y: 7 }),
      ),
    ).toEqual({
      end: { x: 200, y: 0 },
      first: { x: 130, y: -15 },
      kind: "cubic",
      second: { x: 200, y: 0 },
      start: { x: 100, y: 0 },
    });
  });

  test.prop({
    a: integerPoint(1e3),
    after: integerPoint(100),
    b: integerPoint(1e3),
    before: integerPoint(100),
  })(
    "starts at A, ends at B, its controls one handle away from each",
    ({ a, after, b, before }) => {
      const cubic = cubicSide(corner(a, after, after), corner(b, before, before));

      expect([cubic.start, cubic.end]).toEqual([a, b]);
      expect({ x: cubic.first.x - a.x, y: cubic.first.y - a.y }).toEqual(after);
      expect({ x: cubic.second.x - b.x, y: cubic.second.y - b.y }).toEqual(before);
    },
  );
});
