import type { FontMetrics } from "../font";
import type { Text } from "../text";

/** A text wrapped in its box, with the metrics of the face it is drawn with. */
export type TextInBox = {
  /** The metrics of the face the text is drawn with. */
  readonly metrics: FontMetrics;
  /** Its size, spacing and box. */
  readonly text: Text;
};
