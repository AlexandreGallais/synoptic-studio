import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { EPSILON } from "../math";

import { hasLength } from "./has-length";

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
const POINT = fc.record({ x: fc.integer(), y: fc.integer() });

describe("hasLength (properties)", () => {
  test.prop({ end: POINT, start: POINT })(
    "keeps a segment between integer points exactly when they differ, whichever way it runs",
    ({ end, start }) => {
      // Two integer points that differ are at least 1 apart, far above EPSILON.
      const isDrawn = start.x !== end.x || start.y !== end.y;

      expect(hasLength({ end, kind: "segment", start })).toBe(isDrawn);
      expect(hasLength({ end: start, kind: "segment", start: end })).toBe(isDrawn);
    },
  );
});
