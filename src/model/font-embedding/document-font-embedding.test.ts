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
const ON = { hasEmbeddedFonts: true, hasHtmlLayer: false };

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

  it("[F15.AC2] embeds one face for texts in Arial 600 and 700, both measured with 700", () => {
    // 600 searches up from 600 and finds 700 (CSS Fonts 4, §5.2): one face, written once.
    const texts = [600, 700, 600].map((weight) => ({ ...HELLO_TEXT, font: "Arial", weight }));

    expect(documentFontEmbedding(texts, FONTS, ON).embedded).toEqual([
      { family: "Arial", file: BOLD_FILE, isItalic: false, weight: 700 },
    ]);
  });

  it("[F15.AC2] tells an italic face from an upright one of the same weight", () => {
    // Arial 400 upright and italic are two faces; Restricted 400 italic is not permitted.
    const italicFile: FontFile = { ...REGULAR_FILE, bytes: Uint8Array.from([0, 1, 0, 0, 1]) };
    const arial: FontFamily = {
      ...ARIAL,
      faces: [
        { file: REGULAR_FILE, isItalic: false, metrics: ARIAL_METRICS, weight: 400 },
        { file: italicFile, isItalic: true, metrics: ARIAL_METRICS, weight: 400 },
      ],
    };
    const restricted: FontFamily = {
      ...RESTRICTED,
      faces: [{ ...RESTRICTED_FACE, isItalic: true }],
    };
    const texts = [
      { ...HELLO_TEXT, font: "Arial" },
      { ...HELLO_TEXT, font: "Arial", isItalic: true },
      { ...HELLO_TEXT, font: "Restricted", isItalic: true },
    ];

    expect(documentFontEmbedding(texts, [arial, restricted], ON)).toEqual({
      embedded: [
        { family: "Arial", file: REGULAR_FILE, isItalic: false, weight: 400 },
        { family: "Arial", file: italicFile, isItalic: true, weight: 400 },
      ],
      leftOut: [{ family: "Restricted", isItalic: true, reason: "not-permitted", weight: 400 }],
    });
  });

  it("[F15.AC2] leaves a family out whole when one face of it used may not be embedded", () => {
    // Inter 400's file is held, 700's is not: an @font-face family would hide the installed Inter 700.
    const inter: FontFamily = {
      faces: [
        { file: REGULAR_FILE, isItalic: false, metrics: ARIAL_METRICS, weight: 400 },
        { file: undefined, isItalic: false, metrics: ARIAL_METRICS, weight: 700 },
      ],
      name: "Inter",
      version: "",
    };
    const texts = [HELLO_TEXT, { ...HELLO_TEXT, weight: 700 }];

    expect(documentFontEmbedding(texts, [inter, ARIAL], ON)).toEqual({
      embedded: [],
      leftOut: [
        { family: "Inter", isItalic: false, reason: "family-incomplete", weight: 400 },
        { family: "Inter", isItalic: false, reason: "no-file", weight: 700 },
      ],
    });
  });

  it("[F15.AC2] leaves out a family named like a generic family", () => {
    const monospace: FontFamily = { ...ARIAL, name: "Monospace" };

    expect(documentFontEmbedding([{ ...HELLO_TEXT, font: "Monospace" }], [monospace], ON)).toEqual({
      embedded: [],
      leftOut: [{ family: "Monospace", isItalic: false, reason: "generic-name", weight: 400 }],
    });
  });

  it("[F15.AC2] leaves out a face reached only as the default font, no text naming it", () => {
    // Helvetica, then sans-serif: measured with Inter, whose file is held but which no text names.
    const inter: FontFamily = { ...ARIAL, name: "Inter" };
    const helvetica = { ...HELLO_TEXT, fallback: "sans-serif", font: "Helvetica" };

    expect(documentFontEmbedding([helvetica], [inter], ON)).toEqual({
      embedded: [],
      leftOut: [{ family: "Inter", isItalic: false, reason: "not-named", weight: 400 }],
    });

    // A second text in Inter names it: the face is embedded once.
    expect(documentFontEmbedding([helvetica, HELLO_TEXT], [inter], ON)).toEqual({
      embedded: [{ family: "Inter", file: REGULAR_FILE, isItalic: false, weight: 400 }],
      leftOut: [],
    });
  });

  it("[F15.AC2] embeds a face its text names only as its fallback", () => {
    // "Missing" is not held: the text is measured with its fallback, Arial 400, which it names.
    const fallback = { ...HELLO_TEXT, fallback: "Arial", font: "Missing" };

    expect(documentFontEmbedding([fallback], FONTS, ON)).toEqual({
      embedded: [{ family: "Arial", file: REGULAR_FILE, isItalic: false, weight: 400 }],
      leftOut: [],
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
