import { describe, expect, it } from "vitest";

import { DEFAULT_HISTORY_BOUND } from "./default-history-bound";
import { movesWithinSize } from "./moves-within-size";

describe("movesWithinSize (F40)", () => {
  it.each([
    // 100 000 000 / 1 244 = 80 385.9, rounded down to tens.
    { size: 100_000_000, want: 80_380 },
    // 20 000 000 / 1 244 = 16 077.2.
    { size: 20_000_000, want: 16_070 },
    // 10 000 000 / 1 244 = 8 038.6.
    { size: 10_000_000, want: 8030 },
    // 1 000 000 / 1 244 = 803.9.
    { size: 1_000_000, want: 800 },
    // 12 440 / 1 244 = 10 exactly.
    { size: 12_440, want: 10 },
    // 12 439 / 1 244 = 9.999…, below 10.
    { size: 12_439, want: 0 },
    { size: 0, want: 0 },
  ])("[F40.AC4] holds about $want moves of one shape in $size bytes", ({ size, want }) => {
    expect(movesWithinSize(size)).toBe(want);
  });

  it("[F40.AC4] holds about 80 380 moves in the default size, 100 MB", () => {
    expect(movesWithinSize(DEFAULT_HISTORY_BOUND.size)).toBe(80_380);
  });
});
