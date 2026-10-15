import type { NamedFontFace } from "../../src";

/** What reading one installed face gave: the face kept, or why it was refused. */
export type InstalledFaceRead = {
  /** The face read and its family; undefined for a face refused. */
  readonly face: NamedFontFace | undefined;
  /** Why the face was refused, said in one line; empty for a face read. */
  readonly refusal: string;
};
