import type { KerningRule } from "./kerning-rule";

/** A kerning lookup: its subtables in order, the first that matches a pair giving its kerning (`REF-OT-LAYOUT-COMMON`). */
export type KerningLookup = readonly KerningRule[];
