import type { TextStrokeKind } from "../text-stroke";

/** Where a text's HTML layer writes its strokes, each drawn in one color with the others on its element (US-041). */
export type HtmlLayerStrokes = {
  /** The strokes of the content's `div`, those the ellipsis "…" carries too. */
  readonly content: readonly TextStrokeKind[];
  /** The strokes of each paragraph's `span`. */
  readonly paragraphs: readonly TextStrokeKind[];
};
