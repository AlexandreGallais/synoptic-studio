import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { ellipseCorners } from "../../geometry";

import { roundShapeCorners } from "./round-shape-corners";

describe("roundShapeCorners (DERIV-round-shape-curve)", () => {
  it("places the circle's points at its position", () => {
    const shape = { angle: 90, height: 100, kind: "round", width: 100, x: 10, y: 20 } as const;

    // The angle is applied later (turnedCorners): the top point is at (10 + 50, 20).
    expect(roundShapeCorners(shape).map((corner) => corner.point)).toEqual([
      { x: 60, y: 20 },
      { x: 110, y: 70 },
      { x: 60, y: 120 },
      { x: 10, y: 70 },
    ]);
  });

  test.prop({
    height: fc.nat(1000),
    width: fc.nat(1000),
    x: fc.integer({ max: 1000, min: -1000 }),
    y: fc.integer({ max: 1000, min: -1000 }),
  })("keeps the handles of the ellipse, moved by its position", ({ height, width, x, y }) => {
    const corners = roundShapeCorners({ angle: 0, height, kind: "round", width, x, y });
    const unmoved = ellipseCorners({ height, width });

    expect(corners.map((corner) => corner.handles)).toEqual(
      unmoved.map((corner) => corner.handles),
    );

    expect(corners.map((corner) => corner.point)).toEqual(
      unmoved.map((corner) => ({ x: corner.point.x + x, y: corner.point.y + y })),
    );
  });
});
