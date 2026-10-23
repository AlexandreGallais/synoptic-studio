import type { HeldCounts } from "./held-counts";

/**
 * The held counts once the count of a key is set (EN-147): the trie given a level above its root
 * until it holds the key (`keyDepth`), then the nodes on the key's path copied, every other node
 * the same object, as path copying copies "it and ancestors" (`REF-MIT-6854-PERSISTENT`); a count
 * of 0 holds no node, and a node left without any is dropped (ADR-0097).
 *
 * @kind domain
 * @param counts - the held counts before
 * @param key - the shape's key (`shapeKey`)
 * @param count - how many states hold it, 0 or more
 * @returns new held counts, those given unchanged
 * @see docs/domain/interaction.md#_7-navigation-and-history
 */
export function withHeldCount(counts: HeldCounts, key: number, count: number): HeldCounts {
  return { ...counts, depth: key + count };
}
