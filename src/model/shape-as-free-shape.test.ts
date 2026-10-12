import { describe, expect, it } from "vitest";

import { rectangleAsFreeShape } from "./rectangle-as-free-shape";
import { regularPolygonAsFreeShape } from "./regular-polygon-as-free-shape";
import { roundShapeAsFreeShape } from "./round-shape-as-free-shape";
import { shapeAsFreeShape } from "./shape-as-free-shape";

describe("shapeAsFreeShape (ADR-0036)", () => {
  it("[F10.AC3] converts each kind of shape: a free shape stays, a polygon and a round shape convert", () => {
    const free = rectangleAsFreeShape({
      angle: 0,
      height: 50,
      radii: [0, 0, 0, 0],
      width: 100,
      x: 0,
      y: 0,
    });
    const polygon = {
      anchor: { horizontal: "mid", vertical: "mid" },
      angle: 0,
      corners: 6,
      height: 100,
      radii: [0, 0, 0, 0, 0, 0],
      width: 100,
      x: 0,
      y: 0,
    } as const;
    const round = { angle: 0, height: 80, kind: "round", width: 120, x: 0, y: 0 } as const;

    expect(shapeAsFreeShape(free)).toBe(free);
    expect(shapeAsFreeShape(polygon)).toEqual(regularPolygonAsFreeShape(polygon));
    expect(shapeAsFreeShape(round)).toEqual(roundShapeAsFreeShape(round));
  });
});
