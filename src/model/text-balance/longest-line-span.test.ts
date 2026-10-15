import { describe, expect, it } from "vitest";

import { longestLineSpan } from "./longest-line-span";

describe("longestLineSpan (DERIV-text-box step 9)", () => {
  it("gives the most tokens a line holds", () => {
    // Lines 0–1, 0–2, 1–2, 1–3, 2–3, 3–4 of "AAA BB CC DDDDD" in a box 6: at most 2 tokens.
    const ending = [[], [0], [0, 1], [1, 2], [3]].map((starts) => ({
      lines: starts.map((start) => ({ room: 0, start })),
    }));

    expect(longestLineSpan(ending)).toBe(2);
  });

  it("gives 1 when no line fits", () => {
    expect(longestLineSpan([{ lines: [] }, { lines: [] }])).toBe(1);
  });
});
