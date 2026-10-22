import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { circleArcs } from "./circle-arcs";

import type { Point } from "../../math";
import type { Arc } from "../contour";

/**
 * A quarter arc of the circle of center (10, 20) and radius 5, turning positive.
 *
 * @param start - where it starts
 * @param end - where it ends
 * @returns the arc
 */
function quarter(start: Point, end: Point): Arc {
  return { center: { x: 10, y: 20 }, end, kind: "arc", positive: true, radius: 5, start };
}

describe("circleArcs", () => {
  it("gives four quarter arcs from the point right of the center, clockwise on screen", () => {
    // C + (5, 0) = (15, 20); a quarter turn (x, y) → (−y, x): (0, 5) gives (10, 25), (−5, 0)
    // gives (5, 20), (0, −5) gives (10, 15), each exact.
    expect(circleArcs({ x: 10, y: 20 }, 5)).toEqual([
      quarter({ x: 15, y: 20 }, { x: 10, y: 25 }),
      quarter({ x: 10, y: 25 }, { x: 5, y: 20 }),
      quarter({ x: 5, y: 20 }, { x: 10, y: 15 }),
      quarter({ x: 10, y: 15 }, { x: 15, y: 20 }),
    ]);
  });
});

describe("circleArcs (properties)", () => {
  test.prop({
    center: fc.record({
      x: fc.integer({ max: 1000, min: -1000 }),
      y: fc.integer({ max: 1000, min: -1000 }),
    }),
    radius: fc.integer({ max: 50, min: 1 }),
  })("chains four positive arcs of the radius, each end on the circle", ({ center, radius }) => {
    const arcs = circleArcs(center, radius);

    expect(arcs).toHaveLength(4);

    expect(arcs.map((arc) => arc.start)).toEqual([
      ...arcs.slice(-1).map((arc) => arc.end),
      ...arcs.slice(0, -1).map((arc) => arc.end),
    ]);

    for (const arc of arcs) {
      expect(arc).toMatchObject({ center, positive: true, radius });
      expect(Math.hypot(arc.end.x - center.x, arc.end.y - center.y)).toBe(radius);
    }
  });
});
