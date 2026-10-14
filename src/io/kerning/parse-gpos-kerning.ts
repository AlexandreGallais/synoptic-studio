import { parseKernFeatureLookups } from "./parse-kern-feature-lookups";
import { parsePairLookup } from "./parse-pair-lookup";

import type { KerningLookup } from "../../model";

/** Offset of `featureListOffset` in the GPOS header. */
const FEATURE_LIST_FIELD = 6;

/** Offset of `lookupListOffset` in the GPOS header. */
const LOOKUP_LIST_FIELD = 8;

/**
 * Reads the kerning of a `GPOS` table: the lookups of its `kern` feature, in lookup list order,
 * each as its pair adjustment subtables.
 *
 * @kind format
 * @param view - the font file
 * @param gpos - where the `GPOS` table starts in the file
 * @returns its kerning lookups; none when it has no `kern` feature
 * @see REF-OT-GPOS
 */
export function parseGposKerning(view: DataView, gpos: number): readonly KerningLookup[] {
  const lookupList = gpos + view.getUint16(gpos + LOOKUP_LIST_FIELD);

  return parseKernFeatureLookups(view, gpos + view.getUint16(gpos + FEATURE_LIST_FIELD)).map(
    (index) => parsePairLookup(view, lookupList + view.getUint16(lookupList + 2 + 2 * index)),
  );
}
