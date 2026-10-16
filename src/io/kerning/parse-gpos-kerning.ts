import { elementAddress, isSpanWithin } from "../../math";

import { parseKernFeatureLookups } from "./parse-kern-feature-lookups";
import { parsePairLookup } from "./parse-pair-lookup";

import type { KerningLookup } from "../../model";

/** Offset of `featureListOffset` in the GPOS header. */
const FEATURE_LIST_FIELD = 6;

/** Offset of `lookupListOffset` in the GPOS header. */
const LOOKUP_LIST_FIELD = 8;

/** Size of the LookupList's count, and of each offset in it. */
const FIELD_SIZE = 2;

/**
 * Reads where the LookupList's lookups sit.
 *
 * @kind format
 * @param gpos - the `GPOS` table, at least its 10-byte header
 * @param lookupList - where the LookupList table starts in it
 * @returns the offset of each Lookup table in the table; undefined when the list passes its end
 * @see REF-OT-LAYOUT-COMMON
 */
function parseLookupOffsets(gpos: DataView, lookupList: number): readonly number[] | undefined {
  const count = isSpanWithin(lookupList, FIELD_SIZE, gpos.byteLength)
    ? gpos.getUint16(lookupList)
    : Infinity;

  return isSpanWithin(lookupList, elementAddress(FIELD_SIZE, FIELD_SIZE, count), gpos.byteLength)
    ? Array.from(
        { length: count },
        (unused, index) =>
          lookupList + gpos.getUint16(elementAddress(lookupList + FIELD_SIZE, FIELD_SIZE, index)),
      )
    : undefined;
}

/**
 * Reads the kerning lookups at some indices of the LookupList.
 *
 * @kind format
 * @param gpos - the `GPOS` table, at least its 10-byte header
 * @param indices - the indices of the `kern` feature's lookups
 * @param glyphs - the glyphs asked, first and second
 * @returns the lookups, in index order; undefined when an index or a lookup passes the table
 * @see REF-OT-LAYOUT-COMMON
 */
function indicesToLookups(
  gpos: DataView,
  indices: readonly number[],
  glyphs: ReadonlySet<number>,
): readonly KerningLookup[] | undefined {
  const offsets = parseLookupOffsets(gpos, gpos.getUint16(LOOKUP_LIST_FIELD)) ?? [];
  const lookups = indices.map((index) => {
    const lookup = offsets[index];

    return lookup === undefined ? undefined : parsePairLookup(gpos, lookup, glyphs);
  });
  const found = lookups.filter((lookup): lookup is KerningLookup => lookup !== undefined);

  return found.length < lookups.length ? undefined : found;
}

/**
 * Reads the kerning of a `GPOS` table for some glyphs: the lookups of its `kern` feature, in
 * lookup list order, each as its pair adjustment subtables.
 *
 * @kind format
 * @param gpos - the `GPOS` table, at least its 10-byte header
 * @param glyphs - the glyphs asked, first and second
 * @returns its kerning lookups, none when it has no `kern` feature; undefined when a part of it
 * passes the table's end
 * @see REF-OT-GPOS
 */
export function parseGposKerning(
  gpos: DataView,
  glyphs: ReadonlySet<number>,
): readonly KerningLookup[] | undefined {
  const indices = parseKernFeatureLookups(gpos, gpos.getUint16(FEATURE_LIST_FIELD));

  return indices === undefined ? undefined : indicesToLookups(gpos, indices, glyphs);
}
