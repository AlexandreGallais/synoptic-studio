import { describe, expect, it } from "vitest";

import { rectangleAsFreeShape } from "../rectangle";

import { shapePickArea } from "./shape-pick-area";

/** A rectangle 100 × 100 at (0, 0), sharp and straight. */
const SQUARE = rectangleAsFreeShape({
  angle: 0,
  height: 100,
  radii: [0, 0, 0, 0],
  width: 100,
  x: 0,
  y: 0,
});

describe("shapePickArea", () => {
  it("gives a rectangle's corners, its four sides and an area", () => {
    const area = shapePickArea(SQUARE);

    expect(area.corners).toEqual([
      { x: 0, y: 0 },
      { x: 100, y: 0 },
      { x: 100, y: 100 },
      { x: 0, y: 100 },
    ]);

    expect(area.isFlat).toBe(false);

    expect(area.pieces.map((piece) => piece.kind)).toEqual([
      "segment",
      "segment",
      "segment",
      "segment",
    ]);
  });

  it("gives a rounded rectangle's arcs as drawn", () => {
    // Radius 20 at each corner: four sides 60 long and four quarter arcs.
    const area = shapePickArea({
      ...SQUARE,
      corners: SQUARE.corners.map((corner) => ({ ...corner, radius: 20 })),
    });

    expect(area.pieces).toHaveLength(8);
    expect(area.pieces.filter((piece) => piece.kind === "arc")).toHaveLength(4);
  });

  it("marks a box 0 × 80 flat, its corners on its line", () => {
    const flat = rectangleAsFreeShape({
      angle: 0,
      height: 80,
      radii: [0, 0, 0, 0],
      width: 0,
      x: 10,
      y: 10,
    });
    const area = shapePickArea(flat);

    expect(area.isFlat).toBe(true);

    expect(area.corners).toEqual([
      { x: 10, y: 10 },
      { x: 10, y: 10 },
      { x: 10, y: 90 },
      { x: 10, y: 90 },
    ]);
  });
});
