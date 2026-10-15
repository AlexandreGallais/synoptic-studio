import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { ENGLISH_US, WINDOWS_PLATFORM, nameTable, viewOf, viewOfFirst } from "../../../tests";

import { parseFontName } from "./parse-font-name";

/** Language 0x040C of platform 3, French (France) (`REF-OT-NAME`). */
const FRENCH = 0x04_0c;

/** Platform 1, Macintosh. */
const MACINTOSH = 1;

/** Name ID 1, the family. */
const FAMILY = 1;

/** A Windows name in English (United States). */
const ENGLISH_NAME = { language: ENGLISH_US, platform: WINDOWS_PLATFORM };

describe("parseFontName (REF-OT-NAME)", () => {
  it("[F15.AC1] reads a Windows name, its string in UTF-16BE", () => {
    const name = viewOf(nameTable([{ ...ENGLISH_NAME, nameId: FAMILY, text: "Arial" }]));

    expect(parseFontName(name, FAMILY)).toBe("Arial");
  });

  it("reads the name of the ID asked among several", () => {
    const name = viewOf(
      nameTable([
        { ...ENGLISH_NAME, nameId: FAMILY, text: "Inter Light" },
        { ...ENGLISH_NAME, nameId: 2, text: "Regular" },
        { ...ENGLISH_NAME, nameId: 16, text: "Inter" },
      ]),
    );

    expect(parseFontName(name, 2)).toBe("Regular");
    expect(parseFontName(name, 16)).toBe("Inter");
    expect(parseFontName(name, 4)).toBeUndefined();
  });

  it("chooses English (United States) first, else the first Windows name", () => {
    const french = { language: FRENCH, nameId: FAMILY, platform: WINDOWS_PLATFORM, text: "Fr" };
    const english = { ...ENGLISH_NAME, nameId: FAMILY, text: "En" };

    const both = viewOf(nameTable([french, english]));
    const frenchOnly = viewOf(nameTable([french]));

    expect(parseFontName(both, FAMILY)).toBe("En");
    expect(parseFontName(frenchOnly, FAMILY)).toBe("Fr");
  });

  it("reads characters past the BMP as their UTF-16 surrogate pairs", () => {
    // U+1D504 𝔄: 0xD835 0xDD04 in UTF-16BE.
    const name = viewOf(nameTable([{ ...ENGLISH_NAME, nameId: FAMILY, text: "𝔄rial" }]));

    expect(parseFontName(name, FAMILY)).toBe("𝔄rial");
  });

  it("leaves out the names of another platform, and a name of no character", () => {
    const mac = { language: 0, nameId: FAMILY, platform: MACINTOSH, text: "Arial" };
    const empty = { ...ENGLISH_NAME, nameId: FAMILY, text: "" };

    const macOnly = viewOf(nameTable([mac]));
    const emptyWindows = viewOf(nameTable([empty, mac]));

    expect(parseFontName(macOnly, FAMILY)).toBeUndefined();
    expect(parseFontName(emptyWindows, FAMILY)).toBeUndefined();
  });

  it("gives undefined, never an error, for a table too short for its header or records", () => {
    const bytes = nameTable([{ ...ENGLISH_NAME, nameId: FAMILY, text: "Arial" }]);

    expect(parseFontName(viewOf([]), FAMILY)).toBeUndefined();
    expect(parseFontName(viewOf([0, 0, 0, 1, 0]), FAMILY)).toBeUndefined();
    // Header 6 + one record 12 = 18 bytes: 17 cut the record.
    expect(parseFontName(viewOfFirst(bytes, 17), FAMILY)).toBeUndefined();
  });

  it("leaves out a name whose string passes the table's end", () => {
    const bytes = nameTable([{ ...ENGLISH_NAME, nameId: FAMILY, text: "Arial" }]);

    // The string's 10 bytes end the table: one byte fewer cuts it.
    expect(parseFontName(viewOfFirst(bytes, -1), FAMILY)).toBeUndefined();
    expect(parseFontName(viewOf(bytes), FAMILY)).toBe("Arial");
  });

  test.prop({ bytes: fc.array(fc.integer({ max: 255, min: 0 }), { maxLength: 80 }) })(
    "reads any bytes without an error, giving a name or undefined",
    ({ bytes }) => {
      const name = parseFontName(viewOf(bytes), FAMILY);

      expect(name === undefined || name.length > 0).toBe(true);
    },
  );

  test.prop({ text: fc.string({ maxLength: 30, minLength: 1, unit: "binary" }) })(
    "reads back any name written in UTF-16BE",
    ({ text }) => {
      const name = viewOf(nameTable([{ ...ENGLISH_NAME, nameId: FAMILY, text }]));

      expect(parseFontName(name, FAMILY)).toBe(text);
    },
  );
});
