import type { TextAlignment } from "./text-alignment";
import type { TextCase } from "./text-case";
import type { TextDirection } from "./text-direction";
import type { TextLongWord } from "./text-long-word";
import type { TextVerticalAlignment } from "./text-vertical-alignment";
import type { TextWrapStyle } from "./text-wrap-style";

/** The values a text's choices may take, so that a text read from outside is checked against them (F05, Q36, F14). */
export const TEXT_OPTIONS: {
  /** Alignments of its lines. */
  readonly alignments: readonly TextAlignment[];
  /** Cases it is written in. */
  readonly cases: readonly TextCase[];
  /** Writing directions. */
  readonly directions: readonly TextDirection[];
  /** What becomes of a word too long for its box. */
  readonly longWords: readonly TextLongWord[];
  /** Places of its block down its box. */
  readonly verticalAlignments: readonly TextVerticalAlignment[];
  /** Ways its lines are wrapped in its box. */
  readonly wrapStyles: readonly TextWrapStyle[];
} = {
  alignments: ["left", "center", "right"],
  cases: ["none", "upper", "lower", "title", "camel", "pascal", "kebab", "snake", "constant"],
  directions: ["ltr", "rtl"],
  longWords: ["break", "move"],
  verticalAlignments: ["top", "middle", "bottom"],
  wrapStyles: ["auto", "balance"],
};
