import { textStrokeFill } from "../../model";

import { formatCssColor } from "./format-css-color";

import type { CssDeclaration } from "./css-declaration";
import type { Text, TextStrokeKind } from "../../model";

/** The CSS decoration line of each stroke. */
const CSS_LINES: Readonly<Record<TextStrokeKind, string>> = {
  strikeThrough: "line-through",
  underline: "underline",
};

/**
 * Strokes of one element of a text's HTML layer as CSS decoration lines, `underline` and
 * `line-through`, as thick as the font says (`from-font`, its "preferred thickness",
 * `REF-MDN-TEXT-DECORATION-THICKNESS`), drawn across descenders as the SVG's rectangle is
 * (`text-decoration-skip-ink: none`, `REF-MDN-TEXT-DECORATION-SKIP-INK`), in the color and the opacity they are drawn in
 * (`textStrokeFill`, `formatCssColor`, US-041), one for every line of the element
 * (`REF-CSS-TEXT-DECOR-3`, §2.3): the strokes given share it (`textHtmlLayerStrokes`). None for no
 * stroke.
 *
 * @kind format
 * @param text - its strokes' colors
 * @param kinds - the strokes the element draws, one color between them
 * @returns the declarations, none or four
 * @see REF-CSS-TEXT-DECOR-3
 */
export function textStrokeToCss(
  text: Text,
  kinds: readonly TextStrokeKind[],
): readonly CssDeclaration[] {
  const [first] = kinds;

  if (first === undefined) {
    return [];
  }

  const fill = textStrokeFill(text, first);

  return [
    ["text-decoration-line", kinds.map((kind) => CSS_LINES[kind]).join(" ")],
    ["text-decoration-thickness", "from-font"],
    ["text-decoration-skip-ink", "none"],
    ["text-decoration-color", formatCssColor(fill.color, fill.opacity)],
  ];
}
