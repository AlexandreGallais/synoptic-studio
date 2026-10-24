import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { scan } from "./scan";

/**
 * The running total after a number.
 *
 * @param total - the total before it
 * @param value - the number
 * @returns their sum
 */
function added(total: number, value: number): number {
  return total + value;
}

describe("scan (REF-WIKI-PREFIX-SUM)", () => {
  it("gives the prefix sums and the prefix products of the source", () => {
    // 1, 2, 3, 4, 5, 6: sums 1, 3, 6, 10, 15, 21; products 1, 2, 6, 24, 120, 720.
    const numbers = [1, 2, 3, 4, 5, 6];

    expect(scan(numbers, added, 0)).toEqual([1, 3, 6, 10, 15, 21]);
    expect(scan(numbers, (product, value) => product * value, 1)).toEqual([1, 2, 6, 24, 120, 720]);
  });

  it("gives no state for no item", () => {
    expect(scan([], added, 7)).toEqual([]);
  });

  it("keeps the order of a step that does not commute, past the halving", () => {
    // Twenty letters, more than a short run: each state is the text read so far.
    const letters = Array.from({ length: 20 }, (unused, index) => String.fromCodePoint(97 + index));

    expect(scan(letters, (text, letter) => text + letter, "").at(-1)).toBe("abcdefghijklmnopqrst");
  });

  // alone: 34 ms, the median of 5 runs of its file (EN-129); 1 s is 29 times it.
  it("scans 200 000 items in well under a second", { tags: ["timing"], timeout: 1000 }, () => {
    // 1 + 2 + … + 200 000 = 200 000 × 200 001 / 2 = 20 000 100 000.
    const numbers = Array.from({ length: 200_000 }, (unused, index) => index + 1);

    expect(scan(numbers, added, 0).at(-1)).toBe(20_000_100_000);
  });

  test.prop({ initial: fc.integer(), values: fc.array(fc.integer({ max: 1000, min: -1000 })) })(
    "gives each state of the fold, the last being the fold's result",
    ({ initial, values }) => {
      const states = scan(values, added, initial);

      expect(states).toHaveLength(values.length);

      expect(states).toEqual(
        values.map((unused, index) =>
          values.slice(0, index + 1).reduce((total, value) => added(total, value), initial),
        ),
      );
    },
  );
});
