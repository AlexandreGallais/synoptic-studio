import type { NamedFontFace } from "../font-import";

/** A face texts are measured with, and whether one of them names its family. */
export type UsedFace = {
  /** The face and its family's name, as read. */
  readonly named: NamedFontFace;
  /** Whether a text names its family, as its font or its fallback; false for a face reached only as the default font. */
  readonly isNamed: boolean;
};
