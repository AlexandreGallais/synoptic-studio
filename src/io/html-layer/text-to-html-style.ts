import { blockHeight, textHtmlLayerStrokes } from "../../model";
import { formatFontFamily } from "../text-style";

import { formatCssColor } from "./format-css-color";
import { formatCssPixels } from "./format-css-pixels";
import { textStrokeToCss } from "./text-stroke-to-css";

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
 * off, its block is cut, down only, at the height of as many lines as its box shows (`max-height`
 * of `blockHeight`, the lines untrimmed), clipped vertically and left visible across
 * (`overflow-x: visible`, `overflow-y: clip`), so that a word moved whole overflows sideways as
 * the library draws it (`REF-CSS-OVERFLOW-3`). A text that fits its content, or one in a box
 * without limit and the ellipsis off, has none.
 *
 * @kind format
 * @param text - whether it sits in a box and its ellipsis
 * @param layer - its line height and the most lines its box shows
 * @returns none, the ellipsis's, or the overflow and `max-height`
 * @see REF-CSS-OVERFLOW-3
 */
function boxCutToCss(text: Text, layer: HtmlLayer): readonly CssDeclaration[] {
  if (text.isInBox && text.hasEllipsis) {
    return lineLimitToEllipsisCss(layer.lineLimit);
  }

  const line = layer.lineHeight;

  return layer.lineLimit > 0
    ? [
        ["overflow-x", "visible"],
        ["overflow-y", "clip"],
        ["max-height", formatCssPixels(blockHeight({ firstLine: line, line }, layer.lineLimit))],
      ]
    : [];
}

/**
 * The content's `div` as an item of its box's flex column: it never shrinks (`flex-shrink: 0`,
 * §7.2.2), since an item that clips its overflow may shrink to nothing (§4.5, its automatic
 * minimum size zero), which would move a block taller than its box and its lines with it; a text
 * that fits its content is not in a flex column.
 *
 * @kind format
 * @param text - whether it sits in a box
 * @returns the declaration, or none
 * @see REF-CSS-FLEXBOX-1
 */
function boxToFlexItemCss(text: Text): readonly CssDeclaration[] {
  return text.isInBox ? [["flex-shrink", "0"]] : [];
}

/**
 * The style of the `div` holding a text's content in its HTML layer (ADR-0053, decision 1), as
 * plain declarations: its font and color with the library's line height, its spacing, direction
 * and alignment, its strokes of the content's `div` in their color (`textHtmlLayerStrokes`,
 * `textStrokeToCss`, US-041), how its lines break, how a text cut by its box ends, and, in a
 * box, an item of its flex column that never shrinks.
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
    ...textStrokeToCss(text, textHtmlLayerStrokes(text).content),
    ...wrapStyleToCss(text),
    ...boxCutToCss(text, layer),
    ...boxToFlexItemCss(text),
  ];
}
