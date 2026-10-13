import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { expectPoints, integerPoint, rectangleCorners } from "../../../tests";

import { filletArcs } from "./fillet-arcs";

describe("filletArcs (DERIV-fillet-arc)", () => {
  it("gives each corner its arc, a corner of radius 0 an arc without length at its point", () => {
    // 100 × 50, radius 10 at point 1: the arc from (0, 10) to (10, 0) about (10, 10).
    const arcs = filletArcs(rectangleCorners(100, 50, [10, 0, 0, 0]));

    expectPoints(
      arcs.slice(0, 1).flatMap((arc) => [arc.start, arc.end, arc.center]),
      [
        { x: 0, y: 10 },
        { x: 10, y: 0 },
        { x: 10, y: 10 },
      ],
      9,
    );

    expect(arcs.map((arc) => arc.radius)).toEqual([10, 0, 0, 0]);

    expect(arcs.slice(1, 2).flatMap((arc) => [arc.start, arc.end])).toEqual([
      { x: 100, y: 0 },
      { x: 100, y: 0 },
    ]);
  });

  test.prop({ points: fc.array(integerPoint(100), { maxLength: 8, minLength: 3 }) })(
    "gives one arc per corner, at its point when its radius is 0",
    ({ points }) => {
      const arcs = filletArcs(points.map((point) => ({ point, radius: 0 })));

      expect(arcs.map((arc) => arc.start)).toEqual(points);
    },
  );
});
