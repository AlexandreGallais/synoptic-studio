import { describe, expect, it } from "vitest";

import { OPEN_TYPE_VERSION, TRUE_TYPE_VERSION, fontFile, testFontTables } from "../../../tests";

import { parseFontMetrics } from "./parse-font-metrics";

/** A, B, C, a, and Z, which the test font does not map. */
const CHARACTERS = [65, 66, 67, 97, 90];

describe("parseFontMetrics (REF-OT-OTFF)", () => {
  it("[F05.AC1] reads the units per em, the vertical metrics and the advances as written", () => {
    const metrics = parseFontMetrics(fontFile(TRUE_TYPE_VERSION, testFontTables()), CHARACTERS);

    expect(metrics).toEqual({
      // A → glyph 1 (600), B → glyph 2 (700); C → glyph 3 and a → glyph 4, past the three
      // hmtx records: the last record, 700. Z is not mapped: left out, drawn by glyph 0 (500).
      advances: { 65: 600, 66: 700, 67: 700, 97: 700 },
      ascender: 800,
      descender: -200,
      lineGap: 90,
      missingAdvance: 500,
      unitsPerEm: 1000,
    });
  });

  it("reads an OpenType file with CFF outlines the same way", () => {
    const metrics = parseFontMetrics(fontFile(OPEN_TYPE_VERSION, testFontTables()), [65]);

    expect(metrics?.advances).toEqual({ 65: 600 });
  });

  it("reads a file lying inside a larger buffer", () => {
    const file = fontFile(TRUE_TYPE_VERSION, testFontTables());
    const buffer = new Uint8Array(file.length + 8);

    buffer.set(file, 8);

    expect(parseFontMetrics(buffer.subarray(8), [66])?.advances).toEqual({ 66: 700 });
  });

  it("gives undefined for a file that is not a TrueType or OpenType font", () => {
    // A TrueType collection starts with the tag `ttcf`, 0x74746366.
    const collection = fontFile(0x74_74_63_66, testFontTables());

    expect(parseFontMetrics(collection, [65])).toBeUndefined();
    expect(parseFontMetrics(new Uint8Array([0, 1, 0, 0]), [65])).toBeUndefined();
  });
});
