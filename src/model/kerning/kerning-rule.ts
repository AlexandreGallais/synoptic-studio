import type { ClassKerningRule } from "./class-kerning-rule";
import type { PairKerningRule } from "./pair-kerning-rule";

/** One subtable of a kerning lookup: by glyph pairs or by classes of glyphs (`REF-OT-GPOS`). */
export type KerningRule = PairKerningRule | ClassKerningRule;
