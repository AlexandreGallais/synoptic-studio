import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { polygonSegments } from "./polygon-segments";

describe("polygonSegments", () => {
  it("joins each point to the next, the last back to the first", () => {
    expect(
      polygonSegments([
        { x: 0, y: 0 },
        { x: 10, y: 0 },
        { x: 10, y: 10 },
      ]),
    ).toEqual([
      { end: { x: 10, y: 0 }, kind: "segment", start: { x: 0, y: 0 } },
      { end: { x: 10, y: 10 }, kind: "segment", start: { x: 10, y: 0 } },
      { end: { x: 0, y: 0 }, kind: "segment", start: { x: 10, y: 10 } },
    ]);
  });

  it("keeps a side of length 0: one point gives a segment from it to itself", () => {
    expect(polygonSegments([{ x: 5, y: 5 }])).toEqual([
      { end: { x: 5, y: 5 }, kind: "segment", start: { x: 5, y: 5 } },
    ]);
  });

  it("gives none for no point", () => {
    expect(polygonSegments([])).toEqual([]);
  });
});

describe("polygonSegments (properties)", () => {
  /** A point on integers. */
  const integerPoint = fc.record({ x: fc.integer(), y: fc.integer() });

  test.prop({ points: fc.array(integerPoint, { maxLength: 8 }) })(
    "gives one side per point, each from its point to the next, the last back to the first",
    ({ points }) => {
      const sides = polygonSegments(points);

      expect(sides.map((side) => side.start)).toEqual(points);
      expect(sides.map((side) => side.end)).toEqual([...points.slice(1), ...points.slice(0, 1)]);
    },
  );
});
