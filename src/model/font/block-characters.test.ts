import { describe, expect, it } from "vitest";

import { blockCharacters } from "./block-characters";
import { MEASURED_BLOCKS } from "./measured-blocks";

describe("blockCharacters (REF-UNICODE-BLOCKS)", () => {
  it("lists each block's code points from its first to its last, block after block", () => {
    expect(
      blockCharacters([
        { first: 0x41, last: 0x43, name: "A to C" },
        { first: 0x61, last: 0x61, name: "a" },
      ]),
    ).toEqual([0x41, 0x42, 0x43, 0x61]);
  });

  it("lists 1008 code points for the measured blocks", () => {
    // 128 + 128 + 128 (Latin) + 144 (Greek) + 256 (Cyrillic) + 112 (Hebrew) + 112 (punctuation),
    // = 1008, each block's size last − first + 1 read from Blocks.txt.
    expect(blockCharacters(MEASURED_BLOCKS)).toHaveLength(1008);
  });

  it("lists nothing for no block", () => {
    expect(blockCharacters([])).toEqual([]);
  });
});
