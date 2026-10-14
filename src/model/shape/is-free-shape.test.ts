import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { rectangleAsFreeShape } from "../rectangle";
import { regularPolygonAsFreeShape } from "../regular-polygon";

import { isFreeShape } from "./is-free-shape";
import { isRegularPolygon } from "./is-regular-polygon";
import { isRoundShape } from "./is-round-shape";

import type { Shape } from "./shape";
import type { RegularPolygon } from "../regular-polygon";

/** A hexagon 100 × 100, sharp. */
const HEXAGON: RegularPolygon = {
  anchor: { horizontal: "mid", vertical: "mid" },
  angle: 0,
  corners: 6,
  height: 100,
  radii: [0, 0, 0, 0, 0, 0],
  stretched: false,
  width: 100,
  x: 0,
  y: 0,
};

describe("isFreeShape, isRegularPolygon, isRoundShape (F08, F10)", () => {
  it("tells a free shape from a regular polygon and a round shape", () => {
    const shapes: readonly Shape[] = [
      regularPolygonAsFreeShape(HEXAGON),
      HEXAGON,
      { angle: 0, height: 80, kind: "round", width: 120, x: 0, y: 0 },
    ];

    expect(shapes.map((shape) => isFreeShape(shape))).toEqual([true, false, false]);
    expect(shapes.map((shape) => isRegularPolygon(shape))).toEqual([false, true, false]);
    expect(shapes.map((shape) => isRoundShape(shape))).toEqual([false, false, true]);
  });

  test.prop({ height: fc.nat(1000), width: fc.nat(1000) })(
    "holds for every rectangle the tool makes, never for a round shape",
    ({ height, width }) => {
      const box = { angle: 0, height, width, x: 0, y: 0 };
      const shapes: readonly Shape[] = [
        rectangleAsFreeShape({ ...box, radii: [0, 0, 0, 0] }),
        { ...box, kind: "round" },
      ];

      expect(shapes.map((shape) => isFreeShape(shape))).toEqual([true, false]);
    },
  );
});
