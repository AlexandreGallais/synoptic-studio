import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { keptRectangleAt } from "../../../tests";

import { sharedDrawing } from "./shared-drawing";

/** A, 614 characters of JSON, as the present holds it. */
const SHAPE_A = keptRectangleAt(10);

/** B, 615 characters of JSON, as the present holds it. */
const SHAPE_B = keptRectangleAt(200);

/** Any shapes, each a list of up to two small numbers, some equal by their text. */
const ANY_SHAPES = fc.array(fc.array(fc.nat({ max: 3 }), { maxLength: 2 }), { maxLength: 5 });

describe("sharedDrawing (F40)", () => {
  it("[F40.AC4] keeps the present's B for a B read again, and A′, changed, as given", () => {
    const moved = keptRectangleAt(11);
    const shared = sharedDrawing([moved, keptRectangleAt(200)], [SHAPE_A, SHAPE_B]);

    expect(shared).toEqual([keptRectangleAt(11), keptRectangleAt(200)]);
    expect(shared[0]).toBe(moved);
    expect(shared[1]).toBe(SHAPE_B);
  });

  it("[F40.AC4] shares a shape whose text is the present's at another index", () => {
    // B drawn first now, A second: each is still the present's object.
    const shared = sharedDrawing([keptRectangleAt(200), keptRectangleAt(10)], [SHAPE_A, SHAPE_B]);

    expect(shared[0]).toBe(SHAPE_B);
    expect(shared[1]).toBe(SHAPE_A);
  });

  it("[F40.AC4] shares a shape drawn twice with the present's one object", () => {
    const shared = sharedDrawing([keptRectangleAt(10), keptRectangleAt(10)], [SHAPE_A]);

    expect(shared[0]).toBe(SHAPE_A);
    expect(shared[1]).toBe(SHAPE_A);
  });

  it("[F40.AC4] keeps every shape given against an empty present, and an empty drawing empty", () => {
    const added = keptRectangleAt(10);

    expect(sharedDrawing([added], [])[0]).toBe(added);
    expect(sharedDrawing([], [SHAPE_A, SHAPE_B])).toEqual([]);
  });

  test.prop({ drawing: ANY_SHAPES, present: ANY_SHAPES })(
    "[F40.AC4] gives the drawing's texts in its order, each object the present's or the one given",
    ({ drawing, present }) => {
      const before = JSON.stringify({ drawing, present });
      const shared = sharedDrawing(drawing, present);

      expect(shared).toEqual(drawing);

      expect(
        shared.every((shape, index) => shape === drawing[index] || present.includes(shape)),
      ).toBe(true);

      expect(JSON.stringify({ drawing, present })).toBe(before);
    },
  );
});
