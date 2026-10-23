import { describe, expect, it } from "vitest";

import { historyState } from "../../../tests";

import { stateSlotBytes } from "./state-slot-bytes";

describe("stateSlotBytes (F40)", () => {
  it.each([
    // Two references and one index: 3 × 8 = 24.
    { bytes: 24, state: historyState(["a", "b"], [0]), title: "two shapes, one selected" },
    // One reference, no index: 8.
    { bytes: 8, state: historyState(["a"], []), title: "one shape, none selected" },
    { bytes: 0, state: historyState([], []), title: "an empty state" },
  ])("[F40.AC4] counts $title: $bytes bytes", ({ bytes, state }) => {
    expect(stateSlotBytes(state)).toBe(bytes);
  });
});
