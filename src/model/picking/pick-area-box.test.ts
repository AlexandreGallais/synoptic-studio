import { describe, expect, it } from "vitest";

import { pickAreaBox } from "./pick-area-box";

describe("pickAreaBox (AUD-040)", () => {
  it("is the box of the contour as drawn for a shape with area", () => {
    // A triangle (0, 0), (100, 0), (0, 50): [0, 100] × [0, 50].
    const corners = [
      { x: 0, y: 0 },
      { x: 100, y: 0 },
      { x: 0, y: 50 },
    ];

    expect(
      pickAreaBox({
        corners,
        isFlat: false,
        pieces: [
          { end: { x: 100, y: 0 }, kind: "segment", start: { x: 0, y: 0 } },
          { end: { x: 0, y: 50 }, kind: "segment", start: { x: 100, y: 0 } },
          { end: { x: 0, y: 0 }, kind: "segment", start: { x: 0, y: 50 } },
        ],
      }),
    ).toEqual({ maxX: 100, maxY: 50, minX: 0, minY: 0 });
  });

  it("is the point a shape 0 × 0 lies on, its contour having no piece", () => {
    // Four corners at (400, 200): [400, 400] × [200, 200], not the empty box's infinite sides.
    const point = { x: 400, y: 200 };

    expect(
      pickAreaBox({ corners: [point, point, point, point], isFlat: true, pieces: [] }),
    ).toEqual({ maxX: 400, maxY: 200, minX: 400, minY: 200 });
  });
});
