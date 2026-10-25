import { describe, expect, it } from "vitest";

import { isReordered } from "./is-reordered";

/** A, B and C, three objects. */
const [SHAPE_A, SHAPE_B, SHAPE_C] = [["a"], ["b"], ["c"]];

/** A′, a new object of A's text. */
const A_COPY = ["a"];

describe("isReordered", () => {
  it.each([
    // B at 0, A at 1: both moved.
    {
      after: [SHAPE_B, SHAPE_A],
      before: [SHAPE_A, SHAPE_B],
      name: "A and B swapped",
      reordered: true,
    },
    // C before B: B from 1 to 2.
    {
      after: [SHAPE_A, SHAPE_C, SHAPE_B],
      before: [SHAPE_A, SHAPE_B, SHAPE_C],
      name: "B sent to the top",
      reordered: true,
    },
    // B from 1 to 0, A′ a new object at 1.
    {
      after: [SHAPE_B, A_COPY],
      before: [SHAPE_A, SHAPE_B],
      name: "B moved, A changed",
      reordered: true,
    },
    // A replaced at its own index by a new object of its text.
    {
      after: [A_COPY, SHAPE_B],
      before: [SHAPE_A, SHAPE_B],
      name: "A replaced in place",
      reordered: false,
    },
    {
      after: [SHAPE_A, SHAPE_B],
      before: [SHAPE_A, SHAPE_B],
      name: "the same drawing",
      reordered: false,
    },
    { after: [], before: [], name: "two empty drawings", reordered: false },
  ])("[F41.AC1] finds $name: $reordered", ({ after, before, reordered }) => {
    expect(isReordered(before, after)).toBe(reordered);
  });
});
