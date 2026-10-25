import { describe, expect, it } from "vitest";

import { EMPTY_HELD_COUNTS } from "./empty-held-counts";

describe("EMPTY_HELD_COUNTS", () => {
  it("[F41.AC2] is one level and no node", () => {
    expect(EMPTY_HELD_COUNTS).toEqual({ depth: 1, nodes: [] });
  });
});
