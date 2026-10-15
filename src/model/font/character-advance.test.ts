import { describe, expect, it } from "vitest";

import { characterAdvance } from "./character-advance";

import type { FontMetrics } from "./font-metrics";

/** A font mapping A and B, `.notdef` 500 units wide. */
const METRICS: FontMetrics = {
  advances: { 65: 600, 66: 700 },
  ascender: 800,
  capHeight: 700,
  descender: -200,
  embeddingPermissions: 0,
  kerning: { leftClasses: {}, rightClasses: {}, values: [[0]] },
  lineGap: 90,
  missingAdvance: 500,
  strikeoutPosition: 300,
  strikeoutSize: 50,
  underlinePosition: -100,
  underlineThickness: 50,
  unitsPerEm: 1000,
};

describe("characterAdvance (REF-OT-CMAP)", () => {
  it("gives the advance of a mapped character", () => {
    expect(characterAdvance(METRICS, 66)).toBe(700);
  });

  it("[F05.AC1] gives the advance of glyph 0 for a character the font does not map", () => {
    // "Z" (90) is not mapped: it is drawn by .notdef.
    expect(characterAdvance(METRICS, 90)).toBe(500);
  });
});
