import { describe, expect, it } from "vitest";

import { viewOf } from "../../../tests";

import { parseNameString } from "./parse-name-string";

/** A Windows Unicode BMP record of name ID 1, English (United States), its string at 0. */
const RECORD = { encoding: 1, language: 0x04_09, length: 0, nameId: 1, offset: 0, platform: 3 };

describe("parseNameString (REF-OT-NAME)", () => {
  it("reads UTF-16BE for the Unicode platform and Windows encodings 1 and 10", () => {
    // "Ab": 0x0041 0x0062.
    const bytes = viewOf([0x00, 0x41, 0x00, 0x62]);

    expect(
      [
        { encoding: 1, platform: 3 },
        { encoding: 10, platform: 3 },
        { encoding: 0, platform: 0 },
        { encoding: 3, platform: 0 },
        { encoding: 4, platform: 0 },
      ].map((place) => parseNameString(bytes, { ...RECORD, ...place, length: 4 })),
    ).toEqual(["Ab", "Ab", "Ab", "Ab", "Ab"]);
  });

  it("leaves out the last byte of an odd length, never reading past the table", () => {
    // 5 bytes in a table of 5: two code units, "Ab"; a third would read bytes 4 and 5.
    const bytes = viewOf([0x00, 0x41, 0x00, 0x62, 0x00]);

    expect(parseNameString(bytes, { ...RECORD, length: 5 })).toBe("Ab");
    expect(parseNameString(bytes, { ...RECORD, length: 1 })).toBe("");
  });

  it("reads a Macintosh Roman string in ASCII, nothing past it", () => {
    const ascii = viewOf([0x41, 0x62]);
    const roman = viewOf([0x41, 0x8a]);
    const mac = { ...RECORD, encoding: 0, language: 0, length: 2, platform: 1 };

    expect(parseNameString(ascii, mac)).toBe("Ab");
    expect(parseNameString(roman, mac)).toBeUndefined();
  });

  it("reads no Windows code page (encodings 0, 3, 4, 5) and no other Macintosh script", () => {
    const bytes = viewOf([0x41, 0x62]);

    expect(
      [
        { encoding: 0, platform: 3 },
        { encoding: 3, platform: 3 },
        { encoding: 4, platform: 3 },
        { encoding: 5, platform: 3 },
        { encoding: 1, platform: 1 },
        { encoding: 0, platform: 2 },
      ].map((place) => parseNameString(bytes, { ...RECORD, ...place, length: 2 })),
    ).toEqual([undefined, undefined, undefined, undefined, undefined, undefined]);
  });
});
