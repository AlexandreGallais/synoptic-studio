import { keyDepth } from "./key-depth";
import { keyDigit } from "./key-digit";

import type { HeldCounts } from "./held-counts";
import type { HeldNode } from "./held-node";

/**
 * The count below a node of the held counts, by the key's digits from the node's level down.
 *
 * @kind domain
 * @param node - the node, a count at the last level, none where no count is held
 * @param height - the levels below it, 0 for a count
 * @param key - the shape's key (`shapeKey`)
 * @returns the count at the end of the key's path, 0 when it stops before
 * @see ADR-0097
 */
function nodeCount(node: HeldNode | undefined, height: number, key: number): number {
  if (typeof node === "number") {
    return node;
  }

  return node === undefined ? 0 : nodeCount(node[keyDigit(key, height - 1)], height - 1, key);
}

/**
 * How many states hold the shape of a key: the count at the end of its digits' path in
 * the held counts (`nodeCount`), 0 when the path stops before it or the key is past the counts'
 * depth (ADR-0097).
 *
 * @kind domain
 * @param counts - the held counts read
 * @param key - the shape's key (`shapeKey`)
 * @returns the count, 0 for a shape no state holds
 * @see ADR-0097
 */
export function heldCount(counts: HeldCounts, key: number): number {
  return keyDepth(key) > counts.depth ? 0 : nodeCount(counts.nodes, counts.depth, key);
}
