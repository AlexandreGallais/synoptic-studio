import { isSpanWithin } from "../../math";

/** The tag `kern`, as the 32-bit integer of its four bytes. */
const KERN_TAG = 0x6b_65_72_6e;

/** Size of a FeatureList's count, and of a lookup index. */
const FIELD_SIZE = 2;

/** Size of one FeatureRecord: a tag and an offset to its Feature table. */
const FEATURE_RECORD_SIZE = 6;

/** Offset of a FeatureRecord's offset to its Feature table. */
const FEATURE_OFFSET_FIELD = 4;

/** Offset of `lookupListIndices` in a Feature table, after its header. */
const LOOKUP_INDICES_OFFSET = 4;

/**
 * Reads the lookup list indices of one Feature table.
 *
 * @kind format
 * @param gpos - the `GPOS` table
 * @param feature - where the Feature table starts in it
 * @returns its lookup indices; undefined when it passes the table's end
 * @see REF-OT-LAYOUT-COMMON
 */
function parseFeatureLookups(gpos: DataView, feature: number): readonly number[] | undefined {
  const count = isSpanWithin(feature, LOOKUP_INDICES_OFFSET, gpos.byteLength)
    ? gpos.getUint16(feature + 2)
    : Infinity;

  return isSpanWithin(feature, LOOKUP_INDICES_OFFSET + FIELD_SIZE * count, gpos.byteLength)
    ? Array.from({ length: count }, (unused, index) =>
        gpos.getUint16(feature + LOOKUP_INDICES_OFFSET + FIELD_SIZE * index),
      )
    : undefined;
}

/**
 * Reads where the FeatureRecords of a FeatureList sit.
 *
 * @kind format
 * @param gpos - the `GPOS` table
 * @param featureList - where the FeatureList table starts in it
 * @returns the offset of each FeatureRecord in the table; undefined when they pass its end
 * @see REF-OT-LAYOUT-COMMON
 */
function parseFeatureRecords(gpos: DataView, featureList: number): readonly number[] | undefined {
  const count = isSpanWithin(featureList, FIELD_SIZE, gpos.byteLength)
    ? gpos.getUint16(featureList)
    : Infinity;

  return isSpanWithin(featureList, FIELD_SIZE + FEATURE_RECORD_SIZE * count, gpos.byteLength)
    ? Array.from(
        { length: count },
        (unused, index) => featureList + FIELD_SIZE + FEATURE_RECORD_SIZE * index,
      )
    : undefined;
}

/**
 * The lookup indices of several features, once each and ascending.
 *
 * @kind format
 * @param features - the lookup indices of each feature, undefined for one that could not be read
 * @returns the indices; undefined when a feature could not be read
 * @see REF-OT-LAYOUT-COMMON
 */
function featuresToLookups(
  features: readonly (readonly number[] | undefined)[],
): readonly number[] | undefined {
  const found = features.filter((indices): indices is readonly number[] => indices !== undefined);

  return found.length < features.length
    ? undefined
    : [...new Set(found.flat())].toSorted((first, second) => first - second);
}

/**
 * Reads which lookups implement kerning: the lookup list indices of every feature tagged `kern`,
 * whatever its script, once each and in lookup list order, the order a client applies them in.
 *
 * @kind format
 * @param gpos - the `GPOS` table
 * @param featureList - where the FeatureList table starts in it
 * @returns the indices of the kerning lookups, ascending; undefined when the FeatureList or a
 * `kern` Feature table passes the table's end
 * @see REF-OT-LAYOUT-COMMON
 */
export function parseKernFeatureLookups(
  gpos: DataView,
  featureList: number,
): readonly number[] | undefined {
  const records = parseFeatureRecords(gpos, featureList);

  return records === undefined
    ? undefined
    : featuresToLookups(
        records
          .filter((record) => gpos.getUint32(record) === KERN_TAG)
          .map((record) =>
            parseFeatureLookups(gpos, featureList + gpos.getUint16(record + FEATURE_OFFSET_FIELD)),
          ),
      );
}
