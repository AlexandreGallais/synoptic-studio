import { formatCssColor } from "./format-css-color";

import type { CssDeclaration } from "./css-declaration";
import type { Text } from "../../model";

/**
 * The style of each paragraph's `span` in a text's HTML layer: with a background, its color with
 * its opacity (`formatCssColor`), which a browser paints behind each line box fragment of the
 * span, from its ascent to its descent, as the library's bands (`REF-CSS-INLINE-3`, §5.3;
 * `DERIV-text-box`, step 10); none without.
 *
 * @kind format
 * @param text - whether it has a background, its color and opacity
 * @returns the declarations, none or one
 * @see REF-CSS-INLINE-3
 */
export function textToParagraphStyle(text: Text): readonly CssDeclaration[] {
  return text.hasBackground
    ? [["background-color", formatCssColor(text.backgroundColor, text.backgroundOpacity)]]
    : [];
}
