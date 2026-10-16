import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { firstOfEachClass } from "./first-of-each-class";

/**
 * A key that folds the case of a name.
 *
 * @param name - any name
 * @returns the name in lower case
 */
function lowerCase(name: string): string {
  return name.toLowerCase();
}

describe("firstOfEachClass (REF-WIKI-EQUIVALENCE-CLASS)", () => {
  it.each([
    // Arial and arial share the key arial: the first stays; Inter has its own class.
    { expected: ["Arial", "Inter"], items: ["Arial", "arial", "Inter", "INTER"] },
    { expected: ["inter", "Arial"], items: ["inter", "Arial", "Inter", "ARIAL"] },
    { expected: [], items: [] },
    { expected: ["a", "b", "c"], items: ["a", "b", "c"] },
  ])("keeps $expected of $items", ({ expected, items }) => {
    expect(firstOfEachClass(items, lowerCase)).toEqual(expected);
  });

  test.prop({ items: fc.array(fc.constantFrom("a", "A", "b", "B", "c", "d"), { maxLength: 12 }) })(
    "keeps one item of each class, the first, in order",
    ({ items }) => {
      const kept = firstOfEachClass(items, lowerCase);
      const keptKeys = kept.map((item) => lowerCase(item));
      const classes = new Set(items.map((item) => lowerCase(item)));
      const firsts = items.filter(
        (item, index) => items.findIndex((other) => lowerCase(other) === lowerCase(item)) === index,
      );

      // One per class, every class kept, each the first of its class, in the list's order.
      expect(keptKeys).toHaveLength(classes.size);
      expect(new Set(keptKeys)).toEqual(classes);
      expect(kept).toEqual(firsts);
    },
  );
});
