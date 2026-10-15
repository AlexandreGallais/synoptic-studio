import type { EmbeddedFace } from "./embedded-face";
import type { LeftOutFace } from "./left-out-face";

/** The faces a document embeds and those it leaves out, each once, in the order of their first use (`DERIV-font-embedding`, step 7). */
export type FontEmbedding = {
  /** The faces written as `@font-face` rules. */
  readonly embedded: readonly EmbeddedFace[];
  /** The faces not written, each with its reason. */
  readonly leftOut: readonly LeftOutFace[];
};
