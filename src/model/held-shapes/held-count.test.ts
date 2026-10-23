import { describe, expect, it } from "vitest";

import { EMPTY_HELD_COUNTS } from "./empty-held-counts";
import { heldCount } from "./held-count";

import type { HeldCounts } from "./held-counts";

/** The key 33 held by 2 states, 33 = 1 × 32 + 1, and the key 2 by 1. */
const TWO_LEVELS: HeldCounts = {
  depth: 2,
  nodes: [
    [undefined, undefined, 1],
    [undefined, 2],
  ],
};

describe("heldCount (EN-147)", () => {
  it("[F41.AC2] reads 0 from the counts of no state", () => {
    expect(heldCount(EMPTY_HELD_COUNTS, 5)).toBe(0);
  });

  it.each([
    // The root's node 1, then its node 1.
    { count: 2, key: 33, title: "the end of its path" },
    // The root's node 0, then its node 2.
    { count: 1, key: 2, title: "another path" },
    // The root's node 1, then its node 2, none.
    { count: 0, key: 34, title: "a path ending before its count" },
    // The root's node 3, none.
    { count: 0, key: 100, title: "a path ending at the root" },
    // 1 024 needs 3 levels, the counts have 2.
    { count: 0, key: 1024, title: "a key past the depth" },
  ])("[F41.AC2] reads $count at $title", ({ count, key }) => {
    expect(heldCount(TWO_LEVELS, key)).toBe(count);
  });
});
