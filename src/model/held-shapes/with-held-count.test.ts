import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { EMPTY_HELD_COUNTS } from "./empty-held-counts";
import { heldCount } from "./held-count";
import { withHeldCount } from "./with-held-count";

import type { HeldCounts } from "./held-counts";

/** The key 33 held by 2 states, 33 = 1 × 32 + 1, and the key 2 by 1. */
const TWO_LEVELS: HeldCounts = {
  depth: 2,
  nodes: [
    [undefined, undefined, 1],
    [undefined, 2],
  ],
};

/**
 * Any key up to 40 000, four levels: 40 000 = 1 × 32³ + 7 × 32² + 2 × 32 + 0, past
 * 32³ = 32 768; and a count from 0 to 3.
 */
const ANY_KEY_COUNT = fc.tuple(fc.nat({ max: 40_000 }), fc.nat({ max: 3 }));

describe("withHeldCount", () => {
  it("[F41.AC2] sets the key 5 in one level, its node the 6th", () => {
    expect(withHeldCount(EMPTY_HELD_COUNTS, 5, 2)).toEqual({
      depth: 1,
      nodes: [undefined, undefined, undefined, undefined, undefined, 2],
    });
  });

  it("[F41.AC2] grows the trie by one level for the key 33, 1 × 32 + 1", () => {
    expect(withHeldCount(EMPTY_HELD_COUNTS, 33, 1)).toEqual({
      depth: 2,
      nodes: [undefined, [undefined, 1]],
    });
  });

  it("[F41.AC2] keeps the root below the new one when the trie grows: the key 2 under 0", () => {
    // The key 2 in one level, then 1 024 = 1 × 32²: three levels, the old root at 0, 0.
    const one = withHeldCount(EMPTY_HELD_COUNTS, 2, 1);
    const grown = withHeldCount(one, 1024, 3);

    expect(grown.depth).toBe(3);
    expect(grown.nodes[0]).toEqual([one.nodes]);
    expect([heldCount(grown, 2), heldCount(grown, 1024)]).toEqual([1, 3]);
  });

  it("[F41.AC2] shares every node off the key's path: the key 2's node kept for 33", () => {
    const changed = withHeldCount(TWO_LEVELS, 33, 3);

    expect(changed.nodes[0]).toBe(TWO_LEVELS.nodes[0]);
    expect(changed.nodes[1]).toEqual([undefined, 3]);
    expect(TWO_LEVELS.nodes[1]).toEqual([undefined, 2]);
  });

  it("[F41.AC2] drops a node left without any count, then the root's", () => {
    // 33 to 0: its node [undefined, undefined], dropped, and the root's slot 1 with it, the root
    // ending at its last node; then 2 to 0: the root holds none.
    const without = withHeldCount(TWO_LEVELS, 33, 0);

    expect(without).toEqual({ depth: 2, nodes: [[undefined, undefined, 1]] });
    expect(without.nodes).toHaveLength(1);
    expect(withHeldCount(without, 2, 0)).toEqual({ depth: 2, nodes: [] });
  });

  it("[F41.AC2] ends a node at its last child: the key 2 set to 0 beside 1", () => {
    // The keys 1 and 2 in one level: [undefined, 1, 1]; 2 to 0 leaves [undefined, 1].
    const both = withHeldCount(withHeldCount(EMPTY_HELD_COUNTS, 1, 1), 2, 1);

    expect(withHeldCount(both, 2, 0).nodes).toHaveLength(2);
  });

  it("[F41.AC2] keeps an empty root empty when the trie grows", () => {
    expect(withHeldCount(EMPTY_HELD_COUNTS, 1024, 0)).toEqual({ depth: 3, nodes: [] });
  });

  test.prop({ counts: fc.array(ANY_KEY_COUNT, { maxLength: 30 }) })(
    "[F41.AC2] reads back the last count set for each key, 0 for any other, the counts before unchanged",
    ({ counts }) => {
      const middle = Math.floor(counts.length / 2);
      const half = counts
        .slice(0, middle)
        .reduce((before, [key, count]) => withHeldCount(before, key, count), EMPTY_HELD_COUNTS);
      const text = JSON.stringify(half);
      const end = counts
        .slice(middle)
        .reduce((before, [key, count]) => withHeldCount(before, key, count), half);
      const last = new Map(counts);
      const read = last
        .keys()
        .map((key) => heldCount(end, key))
        .toArray();

      expect(read).toEqual(last.values().toArray());
      expect(heldCount(end, 40_001)).toBe(0);
      expect(JSON.stringify(half)).toBe(text);
    },
  );
});
