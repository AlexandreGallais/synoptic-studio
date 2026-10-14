import type { TextAlignment } from "./text-alignment";
import type { TextCase } from "./text-case";
import type { TextDirection } from "./text-direction";

/** The values a text's choices may take, so that a text read from outside is checked against them (F05, Q36). */
export const TEXT_OPTIONS: {
  /** Alignments of its lines. */
  readonly alignments: readonly TextAlignment[];
  /** Cases it is written in. */
  readonly cases: readonly TextCase[];
  /** Writing directions. */
  readonly directions: readonly TextDirection[];
} = {
  alignments: ["left", "center", "right"],
  cases: [
    "none",
    "upper",
    "lower",
    "sentence",
    "title",
    "camel",
    "pascal",
    "kebab",
    "snake",
    "constant",
  ],
  directions: ["ltr", "rtl"],
};
