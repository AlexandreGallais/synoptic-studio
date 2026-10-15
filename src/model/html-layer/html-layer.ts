import type { TurnedBox } from "../placement";

/** What a text's HTML layer is written from (ADR-0053, `DERIV-html-layer`): where it sits, how its lines are spaced and cut, and its paragraphs, which the browser lays out itself. */
export type HtmlLayer = {
  /** The rectangle of its `foreignObject`, in user units: its block's or its box's, moved so that the browser's first baseline meets the library's. */
  readonly frame: Omit<TurnedBox, "angle">;
  /** The distance between two baselines, the library's (`textLineHeight`), in user units. */
  readonly lineHeight: number;
  /** The most lines its box shows, which the browser shows at most; 0 for no limit, a text that fits its content among them. */
  readonly lineLimit: number;
  /** Its content written in its case, cut at each line break typed, one `span` each; a last one left empty holds a zero-width space (`htmlLayerParagraphs`). */
  readonly paragraphs: readonly string[];
};
