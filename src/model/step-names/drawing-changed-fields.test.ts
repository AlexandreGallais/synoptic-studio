import { describe, expect, it } from "vitest";

import { drawingChangedFields } from "./drawing-changed-fields";

/**
 * A shape's own fields.
 *
 * @param shape - a shape, its fields
 * @returns the shape itself
 */
function own(shape: Readonly<Record<string, unknown>>): Readonly<Record<string, unknown>> {
  return shape;
}

/** A at (10, 20), 100 wide. */
const SHAPE_A = { width: 100, x: 10, y: 20 };

/** A′, A nudged to (11, 20). */
const A_NUDGED = { width: 100, x: 11, y: 20 };

/** B at (200, 20). */
const SHAPE_B = { width: 100, x: 200, y: 20 };

describe("drawingChangedFields", () => {
  it("[F41.AC1] names x for A nudged, B the same object", () => {
    expect(drawingChangedFields([SHAPE_A, SHAPE_B], [A_NUDGED, SHAPE_B], own)).toEqual(["x"]);
  });

  it("[F41.AC1] names each field once, sorted, A nudged and B widened to 150", () => {
    const widened = { ...SHAPE_B, width: 150 };

    expect(drawingChangedFields([SHAPE_A, SHAPE_B], [A_NUDGED, widened], own)).toEqual([
      "width",
      "x",
    ]);
  });

  it("[F41.AC1] compares the indices both drawings hold: A against A′, B left out", () => {
    expect(drawingChangedFields([SHAPE_A, SHAPE_B], [A_NUDGED], own)).toEqual(["x"]);
    expect(drawingChangedFields([SHAPE_A], [A_NUDGED, SHAPE_B], own)).toEqual(["x"]);
  });

  it("[F41.AC1] names none when every shape is the same object, or the drawings empty", () => {
    expect(drawingChangedFields([SHAPE_A, SHAPE_B], [SHAPE_A, SHAPE_B], own)).toEqual([]);
    expect(drawingChangedFields([], [], own)).toEqual([]);
  });

  it("[F41.AC1] never reads the fields of a shape shared, the same object", () => {
    const read: unknown[] = [];

    drawingChangedFields([SHAPE_A, SHAPE_B], [A_NUDGED, SHAPE_B], (shape) => {
      read.push(shape);

      return shape;
    });

    expect(read).toEqual([SHAPE_A, A_NUDGED]);
  });

  it("[F41.AC1] reads the fields through the function given", () => {
    const kept = [{ kind: "rectangle", shape: SHAPE_A }];
    const moved = [{ kind: "rectangle", shape: A_NUDGED }];

    expect(drawingChangedFields(kept, moved, (item) => item.shape)).toEqual(["x"]);
  });
});
