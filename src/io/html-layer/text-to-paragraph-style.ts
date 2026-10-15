import { textHtmlLayerStrokes } from "../../model";

import { formatCssColor } from "./format-css-color";
import { textStrokeToCss } from "./text-stroke-to-css";

import type { CssDeclaration } from "./css-declaration";
import type { Text } from "../../model";

/**
 * The style of each paragraph's `span` in a text's HTML layer: with a background, its color with
 * its opacity (`formatCssColor`), which a browser paints behind each line box fragment of the
 * span, from its ascent to its descent, as the library's bands (`REF-CSS-INLINE-3`, §5.3;
 * `DERIV-text-box`, step 10); then its underline when it is of another color than its
 * strike-through, which the content's `div` carries, a decoration's color being one for every
 * line its element draws (`textHtmlLayerStrokes`, `textStrokeToCss`, `REF-CSS-TEXT-DECOR-3`, §2.3;
 * US-041).
 *
 * @kind format
 * @param text - its background and its strokes, their colors and opacities
 * @returns the declarations: the background's one, then the underline's three, those it has
 * @see REF-CSS-INLINE-3
 */
export function textToParagraphStyle(text: Text): readonly CssDeclaration[] {
  const background: readonly CssDeclaration[] = text.hasBackground
    ? [["background-color", formatCssColor(text.backgroundColor, text.backgroundOpacity)]]
    : [];

  return [...background, ...textStrokeToCss(text, textHtmlLayerStrokes(text).paragraphs)];
}
