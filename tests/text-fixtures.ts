// Fonts and texts shared by the tests of the text's measuring (EN-032): the part of Arial 5.01's
// metrics that `DERIV-text-width` lists, and a text of every default option. Tests only: never
// part of the library.

import type { FontMetrics, HtmlLayer, Kerning, Text } from "../src";

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
 * H e l o space W r d A V T R and its kerning pairs; its cap height, `sCapHeight` 1467 of its
 * `OS/2` table of version 3 (`DERIV-text-box`). The advance of glyph 0 is 1536, its first
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
  capHeight: 1467,
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

/**
 * Arial's measured part without kerning, with the advances `DERIV-text-box` adds for its boxes:
 * `-` 682, `.` 569, `0` and `1` 1139, `P` 1366, `g` 1139, `i` 455, `m` 1706, `n` 1139, `p` 1139,
 * `s` 1024, `u` 1139, the no-break space 569 and the ellipsis `…` 2048, read from the same file.
 */
export const ARIAL_BOX_METRICS: FontMetrics = {
  ...ARIAL_UNKERNED,
  advances: {
    ...ARIAL_METRICS.advances,
    45: 682,
    46: 569,
    48: 1139,
    49: 1139,
    80: 1366,
    103: 1139,
    105: 455,
    109: 1706,
    110: 1139,
    112: 1139,
    115: 1024,
    117: 1139,
    160: 569,
    8230: 2048,
  },
};

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

/**
 * The HTML layer of "Hello" over "World" in Arial 16, automatic line height, at (0, 0)
 * (`DERIV-html-layer`, checks): its block 41.78125 × 36.796875 raised by 0.0078125 · 67 / 2 =
 * 0.26171875, its line height 0.0078125 · 2355 = 18.3984375, no limit.
 */
export const ARIAL_HTML_LAYER: HtmlLayer = {
  frame: { height: 36.796875, width: 41.78125, x: 0, y: -0.26171875 },
  lineHeight: 18.3984375,
  lineLimit: 0,
  paragraphs: ["Hello", "World"],
};
