import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { HELLO_TEXT } from "../../../tests";
import { rectangleAsFreeShape } from "../rectangle";

import { drawingWithoutShapes } from "./drawing-without-shapes";

import type { Drawing } from "./drawing";
import type { DrawingShape } from "./drawing-shape";
import type { RoundShape } from "../round-shape";

/** A, a rectangle 100 × 100 at (0, 0), sharp. */
const SHAPE_A = rectangleAsFreeShape({
  angle: 0,
  height: 100,
  radii: [0, 0, 0, 0],
  width: 100,
  x: 0,
  y: 0,
});

/** B, the text Hello. */
const SHAPE_B = HELLO_TEXT;

/** C, a round shape 100 × 100 at (300, 0). */
const SHAPE_C: RoundShape = { angle: 0, height: 100, kind: "round", width: 100, x: 300, y: 0 };

/** Shapes A, B and C drawn in that order, C on top. */
const ABC: Drawing = Object.freeze([SHAPE_A, SHAPE_B, SHAPE_C]);

describe("drawingWithoutShapes (F37)", () => {
  it("[F37.AC1] removes B, A and C unchanged and in their order", () => {
    const left = drawingWithoutShapes(ABC, [1]);

    expect(left).toEqual([SHAPE_A, SHAPE_C]);
    expect(left[0]).toBe(SHAPE_A);
    expect(left[1]).toBe(SHAPE_C);
    expect(ABC).toEqual([SHAPE_A, SHAPE_B, SHAPE_C]);
  });

  it("[F37.AC1] removes A and C, B alone left, unchanged", () => {
    const left = drawingWithoutShapes(ABC, [0, 2]);

    expect(left).toEqual([SHAPE_B]);
    expect(left[0]).toBe(SHAPE_B);
  });

  it("[F37.AC1] removes the first and the last drawn", () => {
    expect(drawingWithoutShapes(ABC, [0])).toEqual([SHAPE_B, SHAPE_C]);
    expect(drawingWithoutShapes(ABC, [2])).toEqual([SHAPE_A, SHAPE_B]);
  });

  it("[F37.AC1] removes every shape, none left", () => {
    expect(drawingWithoutShapes(ABC, [0, 1, 2])).toEqual([]);
  });

  it("[F37.AC1] leaves a new drawing, every shape, when none is selected", () => {
    const left = drawingWithoutShapes(ABC, []);

    expect(left).toEqual([SHAPE_A, SHAPE_B, SHAPE_C]);
    expect(left).not.toBe(ABC);
  });

  it.each([
    { removed: [-1], title: "−1, none picked" },
    { removed: [3], title: "an index past the last shape" },
    { removed: [-2, 5], title: "indices on both sides of the drawing" },
  ])("[F37.AC1] removes nothing for $title", ({ removed }) => {
    expect(drawingWithoutShapes(ABC, removed)).toEqual([SHAPE_A, SHAPE_B, SHAPE_C]);
  });

  it("[F37.AC1] removes a shape named twice once, in any order given", () => {
    expect(drawingWithoutShapes(ABC, [2, 0, 2])).toEqual([SHAPE_B]);
  });

  it("[F37.AC1] leaves an empty drawing empty", () => {
    expect(drawingWithoutShapes([], [0])).toEqual([]);
  });

  test.prop({
    drawing: fc.array(fc.constantFrom<DrawingShape>(SHAPE_A, SHAPE_B, SHAPE_C), { maxLength: 8 }),
    removed: fc.array(fc.integer({ max: 9, min: -2 }), { maxLength: 10 }),
  })(
    "[F37.AC1] keeps every shape not named, in the order drawn, and none named",
    ({ drawing, removed }) => {
      const before = [...drawing];
      const named = new Set(removed);
      const kept = drawing.keys().filter((index) => !named.has(index));

      expect(drawingWithoutShapes(drawing, removed)).toEqual(
        kept.map((index) => drawing[index]).toArray(),
      );

      expect(drawing).toEqual(before);
    },
  );
});
