import { describe, expect, it } from "vitest";

import { pairKerning } from "../kerning";

import { characterAdvance } from "./character-advance";
import { DEFAULT_FONT } from "./default-font";

import type { FontMetrics } from "./font-metrics";

/**
 * The metrics of one face of the default font.
 *
 * @param weight - from 100 to 900
 * @param isItalic - whether the face is italic
 * @returns its metrics
 * @throws {Error} when the font lacks that face, failing the test
 */
function faceMetrics(weight: number, isItalic: boolean): FontMetrics {
  const face = DEFAULT_FONT.faces.find(
    (candidate) => candidate.weight === weight && candidate.isItalic === isItalic,
  );

  if (face === undefined) {
    throw new Error(`no face ${String(weight)}`);
  }

  return face.metrics;
}

describe("DEFAULT_FONT (ADR-0052, REF-INTER-RELEASE)", () => {
  it("[F05.AC1] ships Inter 4.1's nine weights, upright and italic, without the font files", () => {
    expect(DEFAULT_FONT.name).toBe("Inter");
    expect(DEFAULT_FONT.version).toBe("4.1");

    expect(
      DEFAULT_FONT.faces.map((face) => `${String(face.weight)}${face.isItalic ? "i" : ""}`),
    ).toEqual([
      "100",
      "200",
      "300",
      "400",
      "500",
      "600",
      "700",
      "800",
      "900",
      "100i",
      "200i",
      "300i",
      "400i",
      "500i",
      "600i",
      "700i",
      "800i",
      "900i",
    ]);
  });

  it("[F05.AC1] holds Inter Regular's metrics as its file gives them", () => {
    const regular = faceMetrics(400, false);

    // Read from Inter-Regular.ttf by the spike's prototype parser, apart from the library.
    expect(regular).toMatchObject({
      ascender: 1984,
      descender: -494,
      embeddingPermissions: 0,
      lineGap: 0,
      missingAdvance: 1344,
      strikeoutPosition: 671,
      strikeoutSize: 140,
      underlinePosition: -348,
      underlineThickness: 140,
      unitsPerEm: 2048,
    });

    expect([72, 101, 32].map((codePoint) => characterAdvance(regular, codePoint))).toEqual([
      1522, 1194, 576,
    ]);

    // W o, A V, T o: the pair adjustments of the GPOS kern feature.
    expect([pairKerning(regular.kerning, 87, 111), pairKerning(regular.kerning, 65, 86)]).toEqual([
      -104, -140,
    ]);
  });

  it("holds Inter Bold Italic's own advances and kerning", () => {
    const boldItalic = faceMetrics(700, true);

    expect([72, 101, 32].map((codePoint) => characterAdvance(boldItalic, codePoint))).toEqual([
      1530, 1208, 485,
    ]);

    expect(pairKerning(boldItalic.kerning, 65, 86)).toBe(-162);
  });

  it("measures Hebrew with .notdef: Inter has no Hebrew glyph", () => {
    const regular = faceMetrics(400, false);

    // Alef, U+05D0.
    expect(characterAdvance(regular, 0x05_d0)).toBe(1344);
  });
});
