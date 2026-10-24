import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { HELLO_TEXT } from "../../../tests";
import { rectangleAsFreeShape } from "../rectangle";

import { splitDrawing } from "./split-drawing";

import type { Drawing } from "./drawing";
import type { DrawingShape } from "./drawing-shape";
import type { RoundShape } from "../round-shape";

/** A, a rectangle 100 × 100 at (200, 90), sharp. */
const SHAPE_A = rectangleAsFreeShape({
  angle: 0,
  height: 100,
  radii: [0, 0, 0, 0],
  width: 100,
  x: 200,
  y: 90,
});

/** B, the text Hello. */
const SHAPE_B = HELLO_TEXT;

/** C, a round shape 100 × 100 at (0, 0). */
const SHAPE_C: RoundShape = { angle: 0, height: 100, kind: "round", width: 100, x: 0, y: 0 };

/** Shapes A, B and C drawn in that order, C on top. */
const ABC: Drawing = Object.freeze([SHAPE_A, SHAPE_B, SHAPE_C]);

describe("splitDrawing (F36)", () => {
  it("[F36.AC3] keeps a shape picked at the bottom below the others, none below it", () => {
    expect(splitDrawing(ABC, 0)).toEqual({
      above: [SHAPE_B, SHAPE_C],
      below: [],
      edited: [SHAPE_A],
    });
  });

  it("[F36.AC3] keeps a shape picked in the middle between those before and after it", () => {
    const split = splitDrawing(ABC, 1);

    expect(split).toEqual({ above: [SHAPE_C], below: [SHAPE_A], edited: [SHAPE_B] });
    expect(split.edited[0]).toBe(SHAPE_B);
    expect(ABC).toEqual([SHAPE_A, SHAPE_B, SHAPE_C]);
  });

  it("[F36.AC3] keeps a shape picked on top above all, none above it", () => {
    expect(splitDrawing(ABC, 2)).toEqual({
      above: [],
      below: [SHAPE_A, SHAPE_B],
      edited: [SHAPE_C],
    });
  });

  it.each([
    { index: -1, title: "none picked" },
    { index: 3, title: "an index past the last shape" },
    { index: -2, title: "an index before the first shape" },
  ])("[F36.AC3] leaves every shape below and none edited for $title", ({ index }) => {
    expect(splitDrawing(ABC, index)).toEqual({
      above: [],
      below: [SHAPE_A, SHAPE_B, SHAPE_C],
      edited: [],
    });
  });

  it("[F36.AC3] splits an empty drawing into three empty parts", () => {
    expect(splitDrawing([], -1)).toEqual({ above: [], below: [], edited: [] });
  });

  test.prop({
    drawing: fc.array(fc.constantFrom<DrawingShape>(SHAPE_A, SHAPE_B, SHAPE_C), { maxLength: 8 }),
    index: fc.integer({ max: 9, min: -2 }),
  })(
    "[F36.AC3] gives the drawing back joined, the shape picked edited alone and in its place",
    ({ drawing, index }) => {
      const { above, below, edited } = splitDrawing(drawing, index);
      const isPicked = index >= 0 && index < drawing.length;

      expect([...below, ...edited, ...above]).toEqual(drawing);
      expect(edited).toEqual(isPicked ? [drawing[index]] : []);
      expect(below).toHaveLength(isPicked ? index : drawing.length);
    },
  );
});
