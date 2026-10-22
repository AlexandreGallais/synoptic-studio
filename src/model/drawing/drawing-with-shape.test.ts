import { describe, expect, it } from "vitest";

import { HELLO_TEXT } from "../../../tests";
import { rectangleAsFreeShape } from "../rectangle";

import { drawingWithShape } from "./drawing-with-shape";

import type { Drawing } from "./drawing";
import type { RoundShape } from "../round-shape";

/** A rectangle 100 × 100 at (200, 90), sharp. */
const RECTANGLE = rectangleAsFreeShape({
  angle: 0,
  height: 100,
  radii: [0, 0, 0, 0],
  width: 100,
  x: 200,
  y: 90,
});

/** A round shape 100 × 100 at (0, 0). */
const ROUND: RoundShape = { angle: 0, height: 100, kind: "round", width: 100, x: 0, y: 0 };

describe("drawingWithShape (F35)", () => {
  it("[F35.AC1] makes the first shape of an empty drawing", () => {
    expect(drawingWithShape([], RECTANGLE)).toEqual([RECTANGLE]);
  });

  it("[F35.AC1] draws the shape added above the others, which do not change", () => {
    const drawing: Drawing = Object.freeze([RECTANGLE, HELLO_TEXT]);
    const added = drawingWithShape(drawing, ROUND);

    expect(added).toEqual([RECTANGLE, HELLO_TEXT, ROUND]);
    expect(added[0]).toBe(RECTANGLE);
    expect(added[1]).toBe(HELLO_TEXT);
    expect(drawing).toEqual([RECTANGLE, HELLO_TEXT]);
  });

  it("[F35.AC1] keeps a shape added twice twice, each in the order drawn", () => {
    expect(drawingWithShape([ROUND], ROUND)).toEqual([ROUND, ROUND]);
  });
});
