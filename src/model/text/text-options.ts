import type { TextAlignment } from "./text-alignment";
import type { TextCase } from "./text-case";
import type { TextDirection } from "./text-direction";
import type { TextVerticalAlignment } from "./text-vertical-alignment";

/** The values a text's choices may take, so that a text read from outside is checked against them (F05, Q36, F14). */
export const TEXT_OPTIONS: {
  /** Alignments of its lines. */
  readonly alignments: readonly TextAlignment[];
  /** Cases it is written in. */
  readonly cases: readonly TextCase[];
  /** Writing directions. */
  readonly directions: readonly TextDirection[];
  /** Places of its block down its box. */
  readonly verticalAlignments: readonly TextVerticalAlignment[];
} = {
  alignments: ["left", "center", "right"],
  cases: ["none", "upper", "lower", "title", "camel", "pascal", "kebab", "snake", "constant"],
  directions: ["ltr", "rtl"],
  verticalAlignments: ["top", "middle", "bottom"],
};
