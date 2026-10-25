import { describe, expect, it } from "vitest";

import { isKeptInOrder } from "./is-kept-in-order";

/** Three objects, and one equal to the first by its fields but another object. */
const [SHAPE_A, SHAPE_B, SHAPE_C, A_COPY] = [{ x: 0 }, { x: 1 }, { x: 2 }, { x: 0 }];

describe("isKeptInOrder", () => {
  it.each([
    {
      name: "A and C among A, B, C",
      part: [SHAPE_A, SHAPE_C],
      whole: [SHAPE_A, SHAPE_B, SHAPE_C],
      kept: true,
    },
    { name: "none among A", part: [], whole: [SHAPE_A], kept: true },
    { name: "A among A", part: [SHAPE_A], whole: [SHAPE_A], kept: true },
    {
      name: "C, A, out of order, among A, B, C",
      part: [SHAPE_C, SHAPE_A],
      whole: [SHAPE_A, SHAPE_B, SHAPE_C],
      kept: false,
    },
    { name: "a copy of A among A, another object", part: [A_COPY], whole: [SHAPE_A], kept: false },
    { name: "A and B, longer, among A", part: [SHAPE_A, SHAPE_B], whole: [SHAPE_A], kept: false },
  ])("[F41.AC1] finds $name: $kept", ({ part, whole, kept }) => {
    expect(isKeptInOrder(part, whole)).toBe(kept);
  });
});
