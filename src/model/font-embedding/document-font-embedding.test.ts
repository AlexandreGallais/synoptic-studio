import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { ARIAL_METRICS, HELLO_TEXT } from "../../../tests";
import { DEFAULT_DOCUMENT_OPTIONS } from "../document";
import { DEFAULT_FONT } from "../font";

import { documentFontEmbedding } from "./document-font-embedding";

import type { FontFace, FontFamily, FontFile } from "../font";

/** Arial's regular file, editable (fsType 8). */
const REGULAR_FILE: FontFile = {
  bytes: Uint8Array.from([0, 1, 0, 0]),
  isEmbeddable: true,
  version: "",
};

/** Arial's bold file, editable (fsType 8). */
const BOLD_FILE: FontFile = {
  bytes: Uint8Array.from([0, 1, 0, 0, 7]),
  isEmbeddable: true,
  version: "",
};

/** Arial regular and bold, both held. */
const ARIAL: FontFamily = {
  faces: [
    { file: REGULAR_FILE, isItalic: false, metrics: ARIAL_METRICS, weight: 400 },
    { file: BOLD_FILE, isItalic: false, metrics: ARIAL_METRICS, weight: 700 },
  ],
  name: "Arial",
  version: "",
};

/** A face whose file's fsType is 2, restricted licence. */
const RESTRICTED_FACE: FontFace = {
  file: { bytes: Uint8Array.from([0, 1, 0, 0]), isEmbeddable: false, version: "" },
  isItalic: false,
  metrics: ARIAL_METRICS,
  weight: 400,
};

/** A family of that face. */
const RESTRICTED: FontFamily = { faces: [RESTRICTED_FACE], name: "Restricted", version: "" };

/** The fonts read: Arial, the restricted font and the default font, Inter, without files. */
const FONTS = [ARIAL, RESTRICTED, DEFAULT_FONT];

/** The embedding on. */
const ON = { hasEmbeddedFonts: true };

describe("documentFontEmbedding (ADR-0053, DERIV-font-embedding step 7)", () => {
  it("[F15.AC2] embeds 10 texts in Arial 400 and 700 as 2 faces, in the order of first use", () => {
    // Texts 0, 2, 4… bold; 1, 3, 5… regular, named in any case: bold comes first.
    const texts = Array.from({ length: 10 }, (unused, index) => ({
      ...HELLO_TEXT,
      font: index % 2 === 0 ? "Arial" : "ARIAL",
      weight: index % 2 === 0 ? 700 : 400,
    }));

    expect(documentFontEmbedding(texts, FONTS, ON)).toEqual({
      embedded: [
        { family: "Arial", file: BOLD_FILE, isItalic: false, weight: 700 },
        { family: "Arial", file: REGULAR_FILE, isItalic: false, weight: 400 },
      ],
      leftOut: [],
    });
  });

  it("[F15.AC2] embeds the face a text is measured with: 700 for a text of 600 in Arial", () => {
    // Arial holds 400 and 700: a weight of 600 searches up from 600 and finds 700 (CSS Fonts 4, §5.2).
    const embedding = documentFontEmbedding(
      [{ ...HELLO_TEXT, font: "Arial", weight: 600 }],
      FONTS,
      ON,
    );

    expect(embedding.embedded.map((face) => face.weight)).toEqual([700]);
  });

  it("[F15.AC2] leaves out a face not permitted and one whose file is not held, with why", () => {
    // HELLO_TEXT is in Inter 400, whose metrics ship without their file.
    const texts = [
      { ...HELLO_TEXT, font: "Restricted" },
      HELLO_TEXT,
      { ...HELLO_TEXT, font: "Arial" },
      { ...HELLO_TEXT, font: "Restricted" },
    ];

    expect(documentFontEmbedding(texts, FONTS, ON)).toEqual({
      embedded: [{ family: "Arial", file: REGULAR_FILE, isItalic: false, weight: 400 }],
      leftOut: [
        { family: "Restricted", isItalic: false, reason: "not-permitted", weight: 400 },
        { family: "Inter", isItalic: false, reason: "no-file", weight: 400 },
      ],
    });
  });

  it("[F15.AC2] embeds nothing for no text, or texts measured with no face", () => {
    expect(documentFontEmbedding([], FONTS, ON)).toEqual({ embedded: [], leftOut: [] });
    expect(documentFontEmbedding([HELLO_TEXT], [], ON)).toEqual({ embedded: [], leftOut: [] });
  });

  test.prop({ count: fc.integer({ max: 20, min: 0 }), isBold: fc.boolean() })(
    "[F15.AC2] embeds nothing and leaves nothing out with the option off, as by default",
    ({ count, isBold }) => {
      const texts = Array.from({ length: count }, () => ({
        ...HELLO_TEXT,
        font: "Arial",
        weight: isBold ? 700 : 400,
      }));

      expect(DEFAULT_DOCUMENT_OPTIONS.hasEmbeddedFonts).toBe(false);

      expect(documentFontEmbedding(texts, FONTS, DEFAULT_DOCUMENT_OPTIONS)).toEqual({
        embedded: [],
        leftOut: [],
      });
    },
  );
});
