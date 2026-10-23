import { describe, expect, it } from "vitest";

import { DEFAULT_HISTORY_BOUND } from "./default-history-bound";

describe("DEFAULT_HISTORY_BOUND (F40)", () => {
  it("[F40.AC4] is 100 MB, 100 000 000 bytes, and 5 steps kept whatever their size", () => {
    expect(DEFAULT_HISTORY_BOUND).toEqual({ size: 100_000_000, steps: 5 });
  });
});
