import { formatFontFamily } from "../text-style";

import { formatCssColor } from "./format-css-color";
import { formatCssPixels } from "./format-css-pixels";

import type { CssDeclaration } from "./css-declaration";
import type { HtmlLayer, Text } from "../../model";

/**
 * The font, color and line height of a text's HTML layer: its font list as its `<text>` writes it
 * (`formatFontFamily`), its size, weight and style, its color with its opacity, and the library's
 * line height, so that the browser spaces its lines as the library does (`DERIV-html-layer`,
 * step 2).
 *
 * @kind format
 * @param text - its font, size, weight, style, color and opacity
 * @param layer - its line height
 * @returns the declarations, in that order
 * @see REF-CSS-FONTS-4
 */
function fontToCss(text: Text, layer: HtmlLayer): readonly CssDeclaration[] {
  return [
    ["font-family", formatFontFamily(text.font, text.fallback)],
    ["font-size", formatCssPixels(text.size)],
    ["font-weight", String(text.weight)],
    ["font-style", text.isItalic ? "italic" : "normal"],
    ["color", formatCssColor(text.color, text.opacity)],
    ["line-height", formatCssPixels(layer.lineHeight)],
  ];
}

/**
 * The spacing, direction and alignment of a text's lines: its letter and word spacing when not 0,
 * `direction: rtl` right to left, and its alignment, `left`, `center` or `right`, the physical
 * sides for both directions as the library reads them (`REF-CSS-TEXT-3`, §6.1, §7.1, §7.2).
 *
 * @kind format
 * @param text - its spacing, direction and alignment
 * @returns the declarations, in that order
 * @see REF-CSS-TEXT-3
 */
function spacingToCss(text: Text): readonly CssDeclaration[] {
  const spacings: readonly CssDeclaration[] = [
    ["letter-spacing", formatCssPixels(text.letterSpacing)],
    ["word-spacing", formatCssPixels(text.wordSpacing)],
  ];
  const direction: readonly CssDeclaration[] =
    text.direction === "rtl" ? [["direction", "rtl"]] : [];

  return [
    ...spacings.filter(([, value]) => value !== "0px"),
    ...direction,
    ["text-align", text.alignment],
  ];
}

/**
 * The underline and strike-through of a text as CSS decoration lines, as thick as the font says
 * (`from-font`, its "preferred thickness"); none when it has neither.
 *
 * @kind format
 * @param text - whether it is underlined and struck through
 * @returns the declarations, none or two
 * @see REF-MDN-TEXT-DECORATION-THICKNESS
 */
function decorationsToCss(text: Text): readonly CssDeclaration[] {
  const lines = [
    ...(text.isUnderlined ? ["underline"] : []),
    ...(text.isStruckThrough ? ["line-through"] : []),
  ];

  return lines.length > 0
    ? [
        ["text-decoration-line", lines.join(" ")],
        ["text-decoration-thickness", "from-font"],
      ]
    : [];
}

/**
 * How a text's lines break: a text that fits its content keeps every space and breaks only at its
 * line breaks (`pre`); in a box, its lines also wrap to fill it (`pre-wrap`), a word too long
 * broken at any character (`overflow-wrap: anywhere`) or moved whole (`normal`,
 * `REF-MDN-OVERFLOW-WRAP`), balanced when asked (`text-wrap: balance`, `REF-MDN-TEXT-WRAP`).
 *
 * @kind format
 * @param text - whether it sits in a box, its long words and its wrap style
 * @returns the declarations: one for a text that fits its content, two or three in a box
 * @see REF-MDN-WHITE-SPACE
 */
function wrapStyleToCss(text: Text): readonly CssDeclaration[] {
  if (!text.isInBox) {
    return [["white-space", "pre"]];
  }

  const balance: readonly CssDeclaration[] =
    text.wrapStyle === "balance" ? [["text-wrap", "balance"]] : [];

  return [
    ["white-space", "pre-wrap"],
    ["overflow-wrap", text.longWord === "break" ? "anywhere" : "normal"],
    ...balance,
  ];
}

/**
 * The ellipsis of a text in a box: a line wider than the box ends with "…" (`text-overflow:
 * ellipsis`, `REF-MDN-TEXT-OVERFLOW`, with `overflow: hidden`) and, when the box shows at most `n`
 * lines, the block stops at its `n`-th line with "…" (`-webkit-line-clamp: n` with `display:
 * -webkit-box` and `-webkit-box-orient: vertical`).
 *
 * @kind format
 * @param lineLimit - the most lines its box shows, 0 for no limit
 * @returns the declarations, two, or five with a limit
 * @see REF-MDN-LINE-CLAMP
 */
function lineLimitToEllipsisCss(lineLimit: number): readonly CssDeclaration[] {
  const clamp: readonly CssDeclaration[] = [
    ["display", "-webkit-box"],
    ["-webkit-box-orient", "vertical"],
    ["-webkit-line-clamp", String(lineLimit)],
  ];

  return [["overflow", "hidden"], ["text-overflow", "ellipsis"], ...(lineLimit > 0 ? clamp : [])];
}

/**
 * How a text cut by its box ends: with the ellipsis on, as `lineLimitToEllipsisCss` says; with it
 * off, its block is cut at as many line heights as its box shows lines (`max-height` with
 * `overflow: hidden`, the lines spaced by the line height, `REF-CSS-INLINE-3`, §5.1). A text that
 * fits its content, or one in a box without limit and the ellipsis off, has none.
 *
 * @kind format
 * @param text - whether it sits in a box and its ellipsis
 * @param layer - its line height and the most lines its box shows
 * @returns none, the ellipsis's, or `overflow` and `max-height`
 * @see REF-CSS-INLINE-3
 */
function boxCutToCss(text: Text, layer: HtmlLayer): readonly CssDeclaration[] {
  if (text.isInBox && text.hasEllipsis) {
    return lineLimitToEllipsisCss(layer.lineLimit);
  }

  return layer.lineLimit > 0
    ? [
        ["overflow", "hidden"],
        ["max-height", formatCssPixels(layer.lineLimit * layer.lineHeight)],
      ]
    : [];
}

/**
 * The style of the `div` holding a text's content in its HTML layer (ADR-0053, decision 1), as
 * plain declarations: its font and color with the library's line height, its spacing, direction
 * and alignment, its decorations, how its lines break, and how a text cut by its box ends.
 * Lengths are written as the SVG writes its numbers (`formatCssPixels`); the font's names are
 * escaped by `formatFontFamily`.
 *
 * @kind format
 * @param text - the text's options
 * @param layer - its line height and the most lines its box shows (`textHtmlLayer`)
 * @returns the declarations, in order
 * @see REF-MDN-WHITE-SPACE
 */
export function textToHtmlStyle(text: Text, layer: HtmlLayer): readonly CssDeclaration[] {
  return [
    ...fontToCss(text, layer),
    ...spacingToCss(text),
    ...decorationsToCss(text),
    ...wrapStyleToCss(text),
    ...boxCutToCss(text, layer),
  ];
}
