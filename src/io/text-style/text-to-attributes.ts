import { formatSvgNumber } from "../path-data";

import { formatFontFamily } from "./format-font-family";
import { percentToOpacity } from "./percent-to-opacity";

import type { Text } from "../../model";

/** An attribute of an SVG element: its name and its value. */
export type SvgAttribute = readonly [name: string, value: string];

/**
 * The attributes a text's `<text>` carries (ADR-0052, decision 4): its font family, then its
 * fallback and `sans-serif`; its size, weight and style; its color and opacity; its letter and
 * word spacing when not 0; `direction="rtl"` when it is written right to left. No `text-anchor`,
 * no `dominant-baseline` and no `text-decoration`: the library places the lines and draws the
 * decorations itself.
 *
 * @kind format
 * @param text - its content, font, color, spacing and options
 * @returns its attributes, in that order
 * @see REF-SVG2-TEXT
 */
export function textToAttributes(text: Text): readonly SvgAttribute[] {
  const spacings: readonly SvgAttribute[] = [
    ["letter-spacing", formatSvgNumber(text.letterSpacing)],
    ["word-spacing", formatSvgNumber(text.wordSpacing)],
  ];
  const direction: readonly SvgAttribute[] = text.direction === "rtl" ? [["direction", "rtl"]] : [];

  return [
    ["font-family", formatFontFamily(text.font, text.fallback)],
    ["font-size", formatSvgNumber(text.size)],
    ["font-weight", String(text.weight)],
    ["font-style", text.isItalic ? "italic" : "normal"],
    ["fill", text.color],
    ["fill-opacity", formatSvgNumber(percentToOpacity(text.opacity))],
    ...spacings.filter(([, value]) => value !== "0"),
    ...direction,
  ];
}
