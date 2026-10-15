import { parseTag } from "./parse-tag";

/** The tag of a font collection's header, its first four bytes. */
const COLLECTION_TAG = "ttcf";

/** Sizes and offsets of a font collection's header, in bytes, versions 1.0 and 2.0 alike. */
const COLLECTION_HEADER = {
  /** Offset of `numFonts`, past the tag and the major and minor versions. */
  numFontsOffset: 8,
  /** Size of one entry of `tableDirectoryOffsets`. */
  offsetSize: 4,
  /** Offset of `tableDirectoryOffsets`. */
  offsetsStart: 12,
};

/**
 * Reads where the faces of a font collection (`.ttc`) sit: its header, the tag `ttcf`, then
 * `numFonts` and `tableDirectoryOffsets`, one table directory per face, each offset counted from
 * the start of the file. Version 2.0 adds its `DSIG` fields after the offsets, not read.
 *
 * @kind format
 * @param view - the file
 * @returns the offset of each face's table directory; none for a file that is not a collection,
 * or whose offsets do not fit in it
 * @see REF-OT-OTFF
 */
export function parseCollectionOffsets(view: DataView): readonly number[] {
  const { numFontsOffset, offsetSize, offsetsStart } = COLLECTION_HEADER;
  const isCollection = view.byteLength >= offsetsStart && parseTag(view, 0) === COLLECTION_TAG;
  const count = isCollection ? view.getUint32(numFontsOffset) : 0;

  return offsetsStart + offsetSize * count > view.byteLength
    ? []
    : Array.from({ length: count }, (unused, index) =>
        view.getUint32(offsetsStart + offsetSize * index),
      );
}
