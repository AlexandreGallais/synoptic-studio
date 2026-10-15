import type { HtmlLayer } from "./html-layer";
import type { Text } from "../text";
import type { DrawnText } from "../text-drawing";

/** A text, its drawing and its HTML layer: what a document writes for a text (ADR-0053). */
export type TextWithHtmlLayer = {
  /** The text, for its options and its angle. */
  readonly text: Text;
  /** Its positioned lines, backgrounds and decorations, from `textDrawing`. */
  readonly drawn: DrawnText;
  /** Its HTML layer, from `textHtmlLayer`. */
  readonly layer: HtmlLayer;
};
