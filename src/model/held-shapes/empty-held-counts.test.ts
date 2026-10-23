import { describe, expect, it } from "vitest";

import { EMPTY_HELD_COUNTS } from "./empty-held-counts";

describe("EMPTY_HELD_COUNTS (EN-147)", () => {
  it("[F41.AC2] is one level and no node", () => {
    expect(EMPTY_HELD_COUNTS).toEqual({ depth: 1, nodes: [] });
  });
});
