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
  ])("[F08.AC3] gives no edge for points $selected of $count", ({ count, selected }) => {
    expect(edgeOfSelection(selected, count)).toBeUndefined();
  });
});
