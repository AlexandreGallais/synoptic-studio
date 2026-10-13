import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { integerPoint, rectanglePoints } from "../../tests";

import { cornerTriples } from "./corner-triples";

describe("cornerTriples (DERIV-turning-angle, Q11)", () => {
  it("gives the first vertex of the rectangle of US-001 the last as previous, the last the first as next", () => {
    const triples = cornerTriples(rectanglePoints(100, 50));

    expect(triples).toHaveLength(4);

    expect(triples[0]).toEqual({
      next: { x: 100, y: 0 },
      previous: { x: 0, y: 50 },
      vertex: { x: 0, y: 0 },
    });

    expect(triples[3]).toEqual({
      next: { x: 0, y: 0 },
      previous: { x: 100, y: 50 },
      vertex: { x: 0, y: 50 },
    });
  });

  it("makes a single vertex its own neighbours, and no triple of no vertex", () => {
    const point = { x: 5, y: 5 };

    expect(cornerTriples([point])).toEqual([{ next: point, previous: point, vertex: point }]);
    expect(cornerTriples([])).toEqual([]);
  });
});

describe("cornerTriples (properties)", () => {
  test.prop({ points: fc.array(integerPoint(1000), { minLength: 1 }) })(
    "chains the triples: each next is the following vertex, each previous the one before",
    ({ points }) => {
      const triples = cornerTriples(points);
      const count = points.length;

      expect(triples.map((triple) => triple.vertex)).toEqual(points);

      for (const [index, triple] of triples.entries()) {
        expect(triple.next).toBe(points[(index + 1) % count]);
        expect(triple.previous).toBe(points[(index + count - 1) % count]);
      }
    },
  );
});
