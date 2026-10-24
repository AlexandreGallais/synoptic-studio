import { describe, expect, it } from "vitest";

import { HELLO_TEXT } from "../../../tests";
import { rectangleAsFreeShape } from "../rectangle";

import { isDrawnText } from "./is-drawn-text";

import type { DrawingShape } from "./drawing-shape";

describe("isDrawnText (F35)", () => {
  it("tells a text from the shapes of the other tools", () => {
    const shapes: readonly DrawingShape[] = [
      HELLO_TEXT,
      rectangleAsFreeShape({ angle: 0, height: 10, radii: [0, 0, 0, 0], width: 10, x: 0, y: 0 }),
      {
        anchor: { horizontal: "mid", vertical: "mid" },
        angle: 0,
        corners: 3,
        height: 10,
        radii: [0, 0, 0],
        stretched: false,
        width: 10,
        x: 0,
        y: 0,
      },
      { angle: 0, height: 10, kind: "round", width: 10, x: 0, y: 0 },
    ];

    expect(shapes.map((shape) => isDrawnText(shape))).toEqual([true, false, false, false]);
  });
});
