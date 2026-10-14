import { integerRange } from "../../math";

import type { UnicodeBlock } from "./unicode-block";

/**
 * Every code point of some Unicode blocks, block after block, each from its first to its last
 * code point.
 *
 * @kind domain
 * @param blocks - Unicode blocks, each a range of code points
 * @returns their code points, in order
 * @see REF-UNICODE-BLOCKS
 */
export function blockCharacters(blocks: readonly UnicodeBlock[]): readonly number[] {
  return blocks.flatMap((block) => integerRange(block.first, block.last));
}
