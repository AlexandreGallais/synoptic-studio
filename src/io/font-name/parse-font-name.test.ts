import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import {
  ENGLISH_US,
  MACINTOSH_PLATFORM,
  WINDOWS_PLATFORM,
  nameTable,
  viewOf,
} from "../../../tests";

import { parseFontName } from "./parse-font-name";

/** Language 0x040C of Windows, French (France) (`REF-MS-LCID`). */
const FRENCH = 0x04_0c;

/** Language 0x0439 of Windows, Hindi (India) (`REF-MS-LCID`). */
const HINDI = 0x04_39;

/** Name ID 1, the family. */
const FAMILY = 1;

/** A Windows Unicode BMP name in English (United States). */
const WINDOWS_ENGLISH = {
  encoding: 1,
  language: ENGLISH_US,
  nameId: FAMILY,
  platform: WINDOWS_PLATFORM,
};

/** A Macintosh Roman name in English, language 0 (`REF-APPLE-NAME`). */
const MAC_ENGLISH = { encoding: 0, language: 0, nameId: FAMILY, platform: MACINTOSH_PLATFORM };

/** A Unicode platform name, full repertoire, language 0 (`REF-OT-NAME`). */
const UNICODE_NAME = { encoding: 4, language: 0, nameId: FAMILY, platform: 0 };

/**
 * The family name read from a `name` table built from its entries.
 *
 * @param entries - the names, as `nameTable` takes them
 * @returns the name read
 */
function familyOf(entries: Parameters<typeof nameTable>[0]): string | undefined {
  return parseFontName(viewOf(nameTable(entries)), FAMILY);
}

describe("parseFontName (REF-OT-NAME)", () => {
  it("[F15.AC1] reads a Windows name, its string in UTF-16BE", () => {
    expect(familyOf([{ ...WINDOWS_ENGLISH, text: "Arial" }])).toBe("Arial");
  });

  it("reads the name of the ID asked among several", () => {
    const name = viewOf(
      nameTable([
        { ...WINDOWS_ENGLISH, text: "Inter Light" },
        { ...WINDOWS_ENGLISH, nameId: 5, text: "Version 4.1" },
        { ...WINDOWS_ENGLISH, nameId: 16, text: "Inter" },
      ]),
    );

    expect(parseFontName(name, 5)).toBe("Version 4.1");
    expect(parseFontName(name, 16)).toBe("Inter");
    expect(parseFontName(name, 4)).toBeUndefined();
  });

  it("chooses Windows English (United States) first, else the first Windows name", () => {
    const french = { ...WINDOWS_ENGLISH, language: FRENCH, text: "Fr" };

    expect(familyOf([french, { ...WINDOWS_ENGLISH, text: "En" }])).toBe("En");
    expect(familyOf([french, { ...french, text: "Fr 2" }])).toBe("Fr");
  });

  it("reads a font named only on the Macintosh platform, in ASCII", () => {
    expect(familyOf([{ ...MAC_ENGLISH, text: "Geneva" }])).toBe("Geneva");
  });

  it("reads a font named only on the Unicode platform, in UTF-16BE", () => {
    expect(familyOf([{ ...UNICODE_NAME, text: "Unicode Sans" }])).toBe("Unicode Sans");
  });

  it("chooses the Macintosh English name over a Windows name in Hindi", () => {
    const hindi = { ...WINDOWS_ENGLISH, language: HINDI, text: "मंगल" };

    expect(familyOf([hindi, { ...MAC_ENGLISH, text: "Mangal" }])).toBe("Mangal");
    // Without an English name, the Hindi one.
    expect(familyOf([hindi])).toBe("मंगल");
  });

  it("chooses English names in order: Windows, then Macintosh, then Unicode, then another", () => {
    const french = { ...WINDOWS_ENGLISH, language: FRENCH, text: "Fr" };
    const unicode = { ...UNICODE_NAME, text: "Uni" };
    const mac = { ...MAC_ENGLISH, text: "Mac" };

    expect(familyOf([french, unicode, mac, { ...WINDOWS_ENGLISH, text: "Win" }])).toBe("Win");
    expect(familyOf([french, unicode, mac])).toBe("Mac");
    expect(familyOf([french, unicode])).toBe("Uni");
  });

  it("skips a Windows Big5 record (encoding 4) listed before the Unicode BMP one", () => {
    // Encodings 3, 4 and 5 use code pages 936, 950 and 949, which the library does not read.
    const big5 = { ...WINDOWS_ENGLISH, encoding: 4, text: "Big5" };

    expect(familyOf([big5, { ...WINDOWS_ENGLISH, text: "MingLiU" }])).toBe("MingLiU");
    expect(familyOf([big5])).toBeUndefined();
  });

  it("reads characters past the BMP as their UTF-16 surrogate pairs", () => {
    // U+1D504 𝔄: 0xD835 0xDD04 in UTF-16BE.
    expect(familyOf([{ ...WINDOWS_ENGLISH, text: "𝔄rial" }])).toBe("𝔄rial");
  });

  it("leaves out a name of no character, and a Macintosh name past ASCII", () => {
    // macOS Roman 0x8A is ä, not ASCII: not read.
    const accented = { ...MAC_ENGLISH, text: "\u{8A}" };

    expect(familyOf([{ ...WINDOWS_ENGLISH, text: "" }])).toBeUndefined();
    expect(familyOf([accented])).toBeUndefined();

    expect(
      familyOf([
        { ...WINDOWS_ENGLISH, text: "" },
        { ...MAC_ENGLISH, text: "Mac" },
      ]),
    ).toBe("Mac");
  });

  test.prop({ text: fc.string({ maxLength: 30, minLength: 1, unit: "binary" }) })(
    "reads back any name written in UTF-16BE",
    ({ text }) => {
      expect(familyOf([{ ...WINDOWS_ENGLISH, text }])).toBe(text);
    },
  );
});
