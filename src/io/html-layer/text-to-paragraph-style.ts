import { formatCssColor } from "./format-css-color";
import { textStrokeToCss } from "./text-stroke-to-css";

import type { CssDeclaration } from "./css-declaration";
import type { Text } from "../../model";

/**
 * The style of each paragraph's `span` in a text's HTML layer: with a background, its color with
 * its opacity (`formatCssColor`), which a browser paints behind each line box fragment of the
 * span, from its ascent to its descent, as the library's bands (`REF-CSS-INLINE-3`, §5.3;
 * `DERIV-text-box`, step 10); then its strike-through in its own color (`textStrokeToCss`), on the
 * span rather than on the content's `div`, whose underline may have another color, a decoration's
 * color being one for every line its element draws (`REF-CSS-TEXT-DECOR-3`, §2.3; US-041).
 *
 * @kind format
 * @param text - whether it has a background and a strike-through, their colors and opacities
 * @returns the declarations: the background's one, then the strike-through's three, those it has
 * @see REF-CSS-INLINE-3
 */
export function textToParagraphStyle(text: Text): readonly CssDeclaration[] {
  const background: readonly CssDeclaration[] = text.hasBackground
    ? [["background-color", formatCssColor(text.backgroundColor, text.backgroundOpacity)]]
    : [];

  return [...background, ...textStrokeToCss(text, "strikeThrough")];
}
