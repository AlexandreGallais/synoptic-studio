import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { edgeOfSelection } from "./edge-of-selection";

describe("edgeOfSelection", () => {
  it.each([
    { count: 4, edge: 0, selected: [0, 1] },
    { count: 4, edge: 0, selected: [1, 0] },
    { count: 4, edge: 2, selected: [2, 3] },
    // The last and the first are neighbours: the edge that closes the contour.
    { count: 4, edge: 3, selected: [0, 3] },
    { count: 4, edge: 3, selected: [3, 0] },
    // In a triangle every two points are neighbours.
    { count: 3, edge: 2, selected: [0, 2] },
  ])("[F08.AC3] gives edge $edge for points $selected of $count", ({ count, edge, selected }) => {
    expect(edgeOfSelection(selected, count)).toBe(edge);
  });

  it.each([
    { count: 4, selected: [] },
    { count: 4, selected: [1] },
    { count: 4, selected: [0, 2] },
    { count: 4, selected: [0, 1, 2] },
    { count: 6, selected: [1, 4] },
    // Outside the shape, repeated or not whole: no edge.
    { count: 4, selected: [4, 1] },
    { count: 4, selected: [-1, 0] },
    { count: 1, selected: [0, 0] },
    { count: 4, selected: [0.5, 1.5] },
  ])("[F08.AC3] gives no edge for points $selected of $count", ({ count, selected }) => {
    expect(edgeOfSelection(selected, count)).toBeUndefined();
  });

  test.prop({ count: fc.integer({ max: 1000, min: 3 }), seed: fc.nat(), isSwapped: fc.boolean() })(
    "[F08.AC3] gives edge i for points i and the next one, in either order",
    ({ count, isSwapped, seed }) => {
      const edge = seed % count;
      const pair = [edge, (edge + 1) % count];

      expect(edgeOfSelection(isSwapped ? pair.toReversed() : pair, count)).toBe(edge);
    },
  );
});
