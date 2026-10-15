// Fonts and texts shared by the tests of the text's measuring (EN-032): the part of Arial 5.01's
// metrics that `DERIV-text-width` lists, and a text of every default option. Tests only: never
// part of the library.

import type { FontMetrics, Kerning, Text } from "../src";

/** Arial's kerning of W o. */
const W_O = -37;

/** Arial's kerning of A V, V A, A T and T A. */
const WITH_A = -152;

/**
 * Arial's kerning between the characters of the derivation (`DERIV-text-width`, cross-check):
 * W o −37; A V, V A, A T, T A −152 each. Left classes: W 1, A 2, V and T 3; right classes: o 1,
 * V and T 2, A 3.
 */
export const ARIAL_KERNING: Kerning = {
  leftClasses: { 65: 2, 84: 3, 86: 3, 87: 1 },
  rightClasses: { 65: 3, 84: 2, 86: 2, 111: 1 },
  values: [
    [0, 0, 0, 0],
    [0, W_O, 0, 0],
    [0, 0, WITH_A, 0],
    [0, 0, 0, WITH_A],
  ],
};

/** No kerning at all: every pair kerns 0. */
export const NO_KERNING: Kerning = { leftClasses: {}, rightClasses: {}, values: [] };

/**
 * The part of Arial 5.01's metrics (macOS `Arial.ttf`) that measuring reads, as
 * `DERIV-text-width` lists it: `unitsPerEm` 2048, `hhea` 1854, −434, 67, the advances of
 * H e l o space W r d A V T R and its kerning pairs. The advance of glyph 0 is 1536, its first
 * `hmtx` record, read from the same file. The decorations, which measuring does not read, are 0.
 */
export const ARIAL_METRICS: FontMetrics = {
  advances: {
    32: 569,
    65: 1366,
    72: 1479,
    82: 1479,
    84: 1251,
    86: 1366,
    87: 1933,
    100: 1139,
    101: 1139,
    108: 455,
    111: 1139,
    114: 682,
  },
  ascender: 1854,
  descender: -434,
  embeddingPermissions: 0,
  kerning: ARIAL_KERNING,
  lineGap: 67,
  missingAdvance: 1536,
  strikeoutPosition: 0,
  strikeoutSize: 0,
  underlinePosition: 0,
  underlineThickness: 0,
  unitsPerEm: 2048,
};

/** The same metrics without kerning: the derivation's widths "without kerning". */
export const ARIAL_UNKERNED: FontMetrics = { ...ARIAL_METRICS, kerning: NO_KERNING };

/** "Hello World" in Inter 16, regular, black, without spacing, at the origin. */
export const HELLO_TEXT: Text = {
  alignment: "left",
  angle: 0,
  backgroundColor: "#ffff00",
  backgroundOpacity: 100,
  boxHeight: 0,
  boxWidth: 0,
  color: "#000000",
  content: "Hello World",
  direction: "ltr",
  fallback: "sans-serif",
  font: "Inter",
  hasBackground: false,
  hasEllipsis: true,
  isInBox: false,
  isItalic: false,
  isStruckThrough: false,
  isTrimmed: false,
  isUnderlined: false,
  letterSpacing: 0,
  lineHeight: 0,
  longWord: "break",
  maximumLines: 0,
  opacity: 100,
  size: 16,
  textCase: "none",
  verticalAlignment: "top",
  weight: 400,
  wordSpacing: 0,
  wrapStyle: "auto",
  x: 0,
  y: 0,
};

/**
 * Arial's measured part without kerning, with round decorations for hand computation, not
 * Arial's: the underline's top 256 units below the baseline, 128 thick, the strike-out's 512 above
 * it, 64 thick; at size 16, 2, 1, 4 and 0.5 user units.
 */
export const DECORATED_METRICS: FontMetrics = {
  ...ARIAL_UNKERNED,
  strikeoutPosition: 512,
  strikeoutSize: 64,
  underlinePosition: -256,
  underlineThickness: 128,
};
