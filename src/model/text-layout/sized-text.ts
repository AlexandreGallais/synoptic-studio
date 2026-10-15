import type { Size } from "../../geometry";
import type { Text } from "../text";

/** A text and its measured block (`textBlock`, or `textDrawing`'s): what its frame is laid out from (`textFrame`). */
export type SizedText = {
  /** Its measured block, as wide as its widest line and its lines tall. */
  readonly block: Size;
  /** The text: its position, its angle and its box. */
  readonly text: Text;
};
