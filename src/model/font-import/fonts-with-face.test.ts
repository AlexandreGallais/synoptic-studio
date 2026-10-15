import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { ARIAL_METRICS, ARIAL_UNKERNED } from "../../../tests";
import { DEFAULT_FONT } from "../font";

import { fontsWithFace } from "./fonts-with-face";

import type { FontFace, FontFamily, FontMetrics } from "../font";

/** Two bytes standing for a font file. */
const BYTES = Uint8Array.from([0, 1]);

/**
 * A face read from a file.
 *
 * @param weight - from 100 to 900
 * @param metrics - its metrics, which tell two files apart
 * @returns the face, upright, with an embeddable file
 */
function arialFace(weight: number, metrics: FontMetrics): FontFace {
  return {
    file: { bytes: BYTES, isEmbeddable: true, version: "" },
    isItalic: false,
    metrics,
    weight,
  };
}

/**
 * A face read from a file whose single byte tells it apart.
 *
 * @param byte - the only byte of its file, 0 to 255
 * @param weight - from 100 to 900
 * @returns the face, upright
 */
function fileFace(byte: number, weight: number): FontFace {
  return {
    ...arialFace(weight, ARIAL_METRICS),
    file: { bytes: Uint8Array.from([byte]), isEmbeddable: true, version: "" },
  };
}

/** Arial 400 upright, from a first file. */
const REGULAR = arialFace(400, ARIAL_METRICS);

/** Arial 400 upright, from another file. */
const OTHER_REGULAR = arialFace(400, ARIAL_UNKERNED);

/** Arial 700 upright. */
const BOLD = arialFace(700, ARIAL_METRICS);

describe("fontsWithFace (REF-CSS-FONTS-4, §5.1)", () => {
  it("[F15.AC1] lists Arial with its face, Inter unchanged", () => {
    const fonts = fontsWithFace([DEFAULT_FONT], { face: REGULAR, family: "Arial" });

    expect(fonts).toHaveLength(2);
    expect(fonts[0]).toBe(DEFAULT_FONT);
    expect(fonts[1]).toEqual({ faces: [REGULAR], name: "Arial", version: "" });
  });

  it("[F15.AC1] replaces a face of the same family, weight and style, the name without case", () => {
    const first = fontsWithFace([DEFAULT_FONT], { face: REGULAR, family: "Arial" });
    const fonts = fontsWithFace(first, { face: OTHER_REGULAR, family: "ARIAL" });

    // The family keeps the name it was listed under.
    expect(fonts[1]).toEqual({ faces: [OTHER_REGULAR], name: "Arial", version: "" });
    expect(fonts[0]).toBe(DEFAULT_FONT);
  });

  it("[F15.AC1] adds Arial 700 beside Arial 400: two faces", () => {
    const first = fontsWithFace([DEFAULT_FONT], { face: REGULAR, family: "Arial" });
    const fonts = fontsWithFace(first, { face: BOLD, family: "Arial" });

    expect(fonts[1]?.faces).toEqual([REGULAR, BOLD]);
  });

  it("keeps an italic face of the same weight beside the upright one", () => {
    const first = fontsWithFace([], { face: REGULAR, family: "Arial" });
    const italic = { ...OTHER_REGULAR, isItalic: true };

    expect(fontsWithFace(first, { face: italic, family: "Arial" })[0]?.faces).toEqual([
      REGULAR,
      italic,
    ]);
  });

  it("[F15.AC1] never changes the collection given", () => {
    const fonts = [DEFAULT_FONT];
    const first = fontsWithFace(fonts, { face: REGULAR, family: "Arial" });
    const firstFaces = first[1]?.faces;

    fontsWithFace(first, { face: OTHER_REGULAR, family: "Arial" });
    fontsWithFace(first, { face: BOLD, family: "Arial" });

    expect(fonts).toEqual([DEFAULT_FONT]);
    expect(first[1]?.faces).toBe(firstFaces);
    expect(firstFaces).toEqual([REGULAR]);
  });

  test.prop({
    isItalic: fc.boolean(),
    weights: fc.array(fc.integer({ max: 9, min: 1 }), { maxLength: 20 }),
  })("holds each weight once, from the last file added for it", ({ isItalic, weights }) => {
    // The i-th file added is the byte i; its weight 100 × weights[i]; the name's case alternates.
    const fonts = weights.reduce<readonly FontFamily[]>(
      (families, weight, index) =>
        fontsWithFace(families, {
          face: { ...fileFace(index, 100 * weight), isItalic },
          family: index % 2 === 0 ? "Arial" : "arial",
        }),
      [DEFAULT_FONT],
    );
    const [inter, arial] = fonts;
    const faces = arial?.faces ?? [];
    const lastFiles = Object.fromEntries(weights.map((weight, index) => [100 * weight, index]));

    expect(inter).toBe(DEFAULT_FONT);
    expect(fonts).toHaveLength(weights.length === 0 ? 1 : 2);
    expect(faces).toHaveLength(Object.keys(lastFiles).length);

    expect(Object.fromEntries(faces.map((face) => [face.weight, face.file?.bytes[0]]))).toEqual(
      lastFiles,
    );
  });
});
