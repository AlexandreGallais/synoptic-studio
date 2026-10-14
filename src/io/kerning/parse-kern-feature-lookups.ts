/** The tag `kern`, as the 32-bit integer of its four bytes. */
const KERN_TAG = 0x6b_65_72_6e;

/** Size of one FeatureRecord: a tag and an offset to its Feature table. */
const FEATURE_RECORD_SIZE = 6;

/** Offset of a FeatureRecord's offset to its Feature table. */
const FEATURE_OFFSET_FIELD = 4;

/** Offset of `lookupListIndices` in a Feature table. */
const LOOKUP_INDICES_OFFSET = 4;

/**
 * Reads which lookups implement kerning: the lookup list indices of every feature tagged `kern`,
 * whatever its script, once each and in lookup list order, the order a client applies them in.
 *
 * @kind format
 * @param view - the font file
 * @param featureList - where the FeatureList table starts in the file
 * @returns the indices of the kerning lookups, ascending
 * @see REF-OT-LAYOUT-COMMON
 */
export function parseKernFeatureLookups(view: DataView, featureList: number): readonly number[] {
  const indices = Array.from(
    { length: view.getUint16(featureList) },
    (unused, index) => featureList + 2 + FEATURE_RECORD_SIZE * index,
  )
    .filter((record) => view.getUint32(record) === KERN_TAG)
    .flatMap((record) => {
      const feature = featureList + view.getUint16(record + FEATURE_OFFSET_FIELD);

      return Array.from({ length: view.getUint16(feature + 2) }, (unused, index) =>
        view.getUint16(feature + LOOKUP_INDICES_OFFSET + 2 * index),
      );
    });

  return [...new Set(indices)].toSorted((first, second) => first - second);
}
