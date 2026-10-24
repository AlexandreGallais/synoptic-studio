import { describe, expect, it } from "vitest";

import { historyState } from "../../../tests";

import { mostDroppedStates } from "./most-dropped-states";

/** A history of 3 past states, nothing to come. */
const THREE_PAST = {
  future: [],
  past: [historyState([], []), historyState([], []), historyState([], [])],
  present: historyState([], []),
};

describe("mostDroppedStates (F40)", () => {
  it.each([
    // 3 past states, 1 kept: 3 − 1 = 2.
    { most: 2, steps: 1 },
    // 3 kept: 0.
    { most: 0, steps: 3 },
    // 5 kept, more than are past: 0, never fewer.
    { most: 0, steps: 5 },
  ])("[F40.AC4] drops at most $most of 3 past states with $steps kept", ({ most, steps }) => {
    expect(mostDroppedStates(THREE_PAST, { size: 0, steps })).toBe(most);
  });
});
