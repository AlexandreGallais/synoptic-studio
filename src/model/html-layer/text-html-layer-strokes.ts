import { isTextStrokeDrawn, textStrokeFill } from "../text-stroke";

import type { Text } from "../text";
import type { TextStrokeKind } from "../text-stroke";
import type { HtmlLayerStrokes } from "./html-layer-strokes";

/** The strokes in the order the SVG draws them along a line. */
const STROKE_KINDS: readonly TextStrokeKind[] = ["underline", "strikeThrough"];

/**
 * Where a text's HTML layer writes the strokes it draws (US-041): a decoration's color is one for
 * every line its element draws (`REF-CSS-TEXT-DECOR-3`, §2.3), and the ellipsis "…" takes the
 * decorations of its block (`REF-CSS-OVERFLOW-3`, §5.1). Strokes of one color and opacity, as
 * always while they follow the text's, all sit on the content's `div`, so that "…" carries them as
 * the SVG draws them; of two colors, the strike-through sits on the `div`, so that a struck "…"
 * stays struck, and the underline on each paragraph's `span`, so that "…" has none in a browser
 * (agent's reading, provisional, to confirm at VAL-017).
 *
 * @kind domain
 * @param text - its strokes and their colors
 * @returns the strokes of the content's `div` and those of each paragraph's `span`
 * @see REF-CSS-TEXT-DECOR-3
 */
export function textHtmlLayerStrokes(text: Text): HtmlLayerStrokes {
  const drawn = STROKE_KINDS.filter((kind) => isTextStrokeDrawn(text, kind));
  const fills = drawn.map((kind) => textStrokeFill(text, kind));
  const isOneFill = fills.every(
    (fill) => fill.color === fills[0]?.color && fill.opacity === fills[0].opacity,
  );

  return isOneFill
    ? { content: drawn, paragraphs: [] }
    : { content: ["strikeThrough"], paragraphs: ["underline"] };
}
