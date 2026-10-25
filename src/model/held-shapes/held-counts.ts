import type { HeldNode } from "./held-node";

/**
 * How many states of a history hold each shape: a trie of 32 branches by the shape's key
 * (`shapeKey`), its digits in base 32 read from the highest; a count changed copies only the
 * nodes on its key's path, every other node shared with the counts before, so that a history and
 * the next share what a step did not change (ADR-0097, `REF-MIT-6854-PERSISTENT`).
 */
export type HeldCounts = {
  /** The levels of the trie, the keys it holds below 32 to that power. */
  readonly depth: number;
  /** The root's nodes, by the key's highest digit; none past the last one holding a count. */
  readonly nodes: readonly (HeldNode | undefined)[];
};
