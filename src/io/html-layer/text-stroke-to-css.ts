import { textStrokeFill } from "../../model";

import { formatCssColor } from "./format-css-color";

import type { CssDeclaration } from "./css-declaration";
import type { Text, TextStrokeKind } from "../../model";

/**
 * A text's underline or strike-through as a CSS decoration line, `underline` or `line-through`,
 * as thick as the font says (`from-font`, its "preferred thickness", `REF-MDN-TEXT-DECORATION-THICKNESS`), in the color and the
 * opacity the stroke is drawn in (`textStrokeFill`, `formatCssColor`, US-041); none when the text
 * has no such stroke. A decoration's color applies to every line its element draws, so that each
 * stroke sits on an element of its own (`REF-CSS-TEXT-DECOR-3`, §2.3).
 *
 * @kind format
 * @param text - whether it has the stroke, and its colors
 * @param kind - the underline or the strike-through
 * @returns the declarations, none or three
 * @see REF-CSS-TEXT-DECOR-3
 */
export function textStrokeToCss(text: Text, kind: TextStrokeKind): readonly CssDeclaration[] {
  const isDrawn = kind === "underline" ? text.isUnderlined : text.isStruckThrough;
  const fill = textStrokeFill(text, kind);

  return isDrawn
    ? [
        ["text-decoration-line", kind === "underline" ? "underline" : "line-through"],
        ["text-decoration-thickness", "from-font"],
        ["text-decoration-color", formatCssColor(fill.color, fill.opacity)],
      ]
    : [];
}
