import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { cornersWithRadii } from "./corners-with-radii";

describe("cornersWithRadii", () => {
  it("[F03.AC1] gives each vertex the radius of the same number, in contour order", () => {
    const points = [
      { x: 0, y: 0 },
      { x: 100, y: 0 },
      { x: 100, y: 100 },
      { x: 0, y: 100 },
    ];

    expect(cornersWithRadii(points, [50, 0, 0, 0])).toEqual([
      { point: { x: 0, y: 0 }, radius: 50 },
      { point: { x: 100, y: 0 }, radius: 0 },
      { point: { x: 100, y: 100 }, radius: 0 },
      { point: { x: 0, y: 100 }, radius: 0 },
    ]);
  });

  it("gives no corner to no vertex", () => {
    expect(cornersWithRadii([], [])).toEqual([]);
  });

  it("leaves a vertex without radius sharp, as a new vertex is (ADR-0029)", () => {
    expect(
      cornersWithRadii(
        [
          { x: 0, y: 0 },
          { x: 10, y: 0 },
        ],
        [5],
      ),
    ).toEqual([
      { point: { x: 0, y: 0 }, radius: 5 },
      { point: { x: 10, y: 0 }, radius: 0 },
    ]);
  });

  const anyPoint = fc.record({ x: fc.integer(), y: fc.integer() });

  test.prop([
    fc.array(anyPoint, { maxLength: 12 }),
    fc.array(fc.nat({ max: 100 }), { maxLength: 12 }),
  ])("gives one corner per vertex, with the radius of its number or 0", (points, radii) => {
    const corners = cornersWithRadii(points, radii);

    expect(corners.map((corner) => corner.point)).toEqual(points);

    expect(corners.map((corner) => corner.radius)).toEqual(
      points.map((point, index) => radii[index] ?? 0),
    );
  });
});
