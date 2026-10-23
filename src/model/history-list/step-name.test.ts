import { describe, expect, it } from "vitest";

import { keptRectangleAt } from "../../../tests";

import { stepName } from "./step-name";

import type { KeptRectangle } from "../../../tests";

/** A rectangle as the page keeps it, its kind any the panel shows. */
type KeptShapeLike = Omit<KeptRectangle, "kind"> & {
  /** The kind the panel shows it as. */
  readonly kind: string;
};

/**
 * The fields of a kept rectangle, its kind among them.
 *
 * @param kept - a rectangle as the page keeps it, its kind possibly changed
 * @returns its shape's fields and its kind
 */
function keptFields(kept: KeptShapeLike): Readonly<Record<string, unknown>> {
  return { ...kept.shape, kind: kept.kind };
}

/** A at (10, 20). */
const SHAPE_A = keptRectangleAt(10);

/** A′, A nudged to (11, 20). */
const A_NUDGED = keptRectangleAt(11);

/** B at (200, 20). */
const SHAPE_B = keptRectangleAt(200);

/** C at (400, 20). */
const SHAPE_C = keptRectangleAt(400);

/**
 * A with some fields of its shape changed.
 *
 * @param fields - the fields changed and their values
 * @returns a new object, A otherwise
 */
function changedA(fields: object): KeptRectangle {
  return { ...SHAPE_A, shape: { ...SHAPE_A.shape, ...fields } };
}

describe("stepName (US-077)", () => {
  it.each([
    // x alone changed, B the same object.
    { after: [A_NUDGED, SHAPE_B], before: [SHAPE_A, SHAPE_B], name: "Move", title: "A nudged" },
    // x and y changed.
    {
      after: [changedA({ x: 41, y: 39 }), SHAPE_B],
      before: [SHAPE_A, SHAPE_B],
      name: "Move",
      title: "A dragged",
    },
    // A and B kept in order, C after them.
    {
      after: [SHAPE_A, SHAPE_B, SHAPE_C],
      before: [SHAPE_A, SHAPE_B],
      name: "Add",
      title: "C added",
    },
    // A and C kept in order, B gone.
    {
      after: [SHAPE_A, SHAPE_C],
      before: [SHAPE_A, SHAPE_B, SHAPE_C],
      name: "Delete",
      title: "B deleted",
    },
    // B kept, A gone.
    { after: [SHAPE_B], before: [SHAPE_A, SHAPE_B], name: "Delete", title: "A deleted" },
    // width alone changed.
    {
      after: [changedA({ width: 150 })],
      before: [SHAPE_A],
      name: "Width",
      title: "150 typed in W",
    },
    // width and x changed: the field beside x and y names it.
    { after: [changedA({ width: 150, x: 0 })], before: [SHAPE_A], name: "Width", title: "W and X" },
    // width and height changed.
    {
      after: [changedA({ height: 70, width: 150 })],
      before: [SHAPE_A],
      name: "Change",
      title: "W and H",
    },
    // kind changed, the shape the same.
    {
      after: [{ ...SHAPE_A, kind: "free" }],
      before: [SHAPE_A],
      name: "Convert",
      title: "A converted",
    },
    // kind and corners changed.
    {
      after: [{ ...changedA({ corners: [] }), kind: "free" }],
      before: [SHAPE_A],
      name: "Convert",
      title: "A converted, its points changed",
    },
    // C added, A changed: A not kept.
    {
      after: [A_NUDGED, SHAPE_B, SHAPE_C],
      before: [SHAPE_A, SHAPE_B],
      name: "Change",
      title: "C added, A nudged",
    },
    // B removed, A changed.
    { after: [A_NUDGED], before: [SHAPE_A, SHAPE_B], name: "Change", title: "B deleted, A nudged" },
    // Another object of A's text: no field changed.
    { after: [keptRectangleAt(10)], before: [SHAPE_A], name: "Change", title: "A read again" },
  ])("[F41.AC1] names $title '$name'", ({ after, before, name }) => {
    expect(stepName(before, after, keptFields)).toBe(name);
  });

  it("[F41.AC1] names a text's content changed 'Content'", () => {
    expect(stepName([{ content: "Valve" }], [{ content: "Pump" }], (text) => text)).toBe("Content");
  });
});
