import { keyDepth } from "./key-depth";
import { keyDigit } from "./key-digit";

import type { HeldCounts } from "./held-counts";
import type { HeldNode } from "./held-node";

/** Where a count is set: its key and the count. */
type HeldEntry = {
  /** The shape's key (`shapeKey`). */
  readonly key: number;
  /** How many states hold it, 0 or more. */
  readonly count: number;
};

/**
 * The held counts given levels above their root until they hold a depth, the root before the
 * first node of each new one; an empty root stays empty.
 *
 * @kind domain
 * @param counts - the held counts before
 * @param depth - the levels they need
 * @returns the counts given when deep enough, otherwise new counts
 * @see ADR-0097
 */
function grownCounts(counts: HeldCounts, depth: number): HeldCounts {
  if (counts.depth >= depth) {
    return counts;
  }

  const nodes = counts.nodes.length === 0 ? [] : [counts.nodes];

  return grownCounts({ depth: counts.depth + 1, nodes }, depth);
}

/**
 * A node once one of its children is replaced, the others kept, ending at its last child: none
 * past it, so that a node holds no empty slot at its end; none when it is left without any.
 *
 * @kind domain
 * @param children - the nodes below it, none past the last one holding a count
 * @param digit - the place of the child replaced, from 0 to 31
 * @param child - the new child, or none
 * @returns a new node, or none
 * @see ADR-0097
 */
function nodeWithChild(
  children: readonly (HeldNode | undefined)[],
  digit: number,
  child: HeldNode | undefined,
): HeldNode | undefined {
  const length = Math.max(children.length, digit + 1);
  const next = Array.from({ length })
    .keys()
    .map((index) => (index === digit ? child : children[index]))
    .toArray();
  const last = next.findLastIndex((item) => item !== undefined);

  return last === -1 ? undefined : next.slice(0, last + 1);
}

/**
 * A node of the held counts once the count of a key below it is set: the node on the key's path
 * copied (`nodeWithChild`), the others kept; none for a count of 0 or a node left without any.
 *
 * @kind domain
 * @param node - the node before, none where no count was held
 * @param height - the levels below it, 0 for a count
 * @param entry - the key and its count
 * @returns the new node, or none
 * @see ADR-0097
 */
function nodeWithCount(
  node: HeldNode | undefined,
  height: number,
  entry: HeldEntry,
): HeldNode | undefined {
  if (height === 0) {
    return entry.count === 0 ? undefined : entry.count;
  }

  const children = typeof node === "object" ? node : [];
  const digit = keyDigit(entry.key, height - 1);

  return nodeWithChild(children, digit, nodeWithCount(children[digit], height - 1, entry));
}

/**
 * The held counts once the count of a key is set: the trie given a level above its root
 * until it holds the key (`keyDepth`, `grownCounts`), then the nodes on the key's path copied,
 * every other node the same object, as path copying copies "it and ancestors"
 * (`REF-MIT-6854-PERSISTENT`); a count of 0 holds no node, and a node left without any is dropped
 * (`nodeWithCount`, ADR-0097).
 *
 * @kind domain
 * @param counts - the held counts before
 * @param key - the shape's key (`shapeKey`)
 * @param count - how many states hold it, 0 or more
 * @returns new held counts, those given unchanged
 * @see ADR-0097
 */
export function withHeldCount(counts: HeldCounts, key: number, count: number): HeldCounts {
  const grown = grownCounts(counts, keyDepth(key));
  const root = nodeWithCount(grown.nodes, grown.depth, { count, key });

  return { depth: grown.depth, nodes: typeof root === "object" ? root : [] };
}
