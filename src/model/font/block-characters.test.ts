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

  it("lists 1968 code points for the measured blocks", () => {
    // Latin 128 + 128 + 128 + 208 (Extended-B) + 256 (Extended Additional), Greek 144, Cyrillic
    // 256, Hebrew 112, punctuation 112, currency 48, letterlike 80, arrows 112, mathematical
    // operators 256 = 1968, each block's size last − first + 1 read from Blocks.txt.
    expect(blockCharacters(MEASURED_BLOCKS)).toHaveLength(1968);
  });

  it("holds the symbols industrial labels use: € − → ≤ ™ ș ț", () => {
    const characters = new Set(blockCharacters(MEASURED_BLOCKS));

    // € U+20AC, − U+2212, → U+2192, ≤ U+2264, ™ U+2122, ș U+0219, ț U+021B.
    expect(
      [0x20_ac, 0x22_12, 0x21_92, 0x22_64, 0x21_22, 0x02_19, 0x02_1b].every((codePoint) =>
        characters.has(codePoint),
      ),
    ).toBe(true);
  });

  it("lists nothing for no block", () => {
    expect(blockCharacters([])).toEqual([]);
  });
});
