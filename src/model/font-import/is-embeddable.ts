import { isBitSet } from "../../math";

/** `fsType` bit 1, restricted licence embedding. */
const RESTRICTED_BIT = 1;

/** `fsType` bit 2, preview and print embedding. */
const PREVIEW_AND_PRINT_BIT = 2;

/** `fsType` bit 3, editable embedding. */
const EDITABLE_BIT = 3;

/** `fsType` bit 9, bitmap embedding only. */
const BITMAP_ONLY_BIT = 9;

/** The first version of the `OS/2` table whose `fsType` bits 4 to 15 are read. */
const BITMAP_ONLY_VERSION = 2;

/**
 * Whether a font file may be embedded whole, outlines included, as its `OS/2` table's `fsType`
 * permits (`DERIV-font-embedding`, step 3): not when only bitmaps may be embedded (bit 9, read
 * from version 2 of the table), nor when the licence is restricted (bit 1) without the
 * preview-and-print (bit 2) or editable (bit 3) permission, the least restrictive winning. Arial
 * (8) and Inter (0) may be embedded; 2 may not.
 *
 * Formula: ¬(v ≥ 2 ∧ bit(9)) ∧ ¬(bit(1) ∧ ¬(bit(2) ∨ bit(3))).
 *
 * @kind domain
 * @param fsType - the `fsType` flags, 0 to 65 535
 * @param version - the version of the `OS/2` table
 * @returns true when the whole file may be embedded
 * @see DERIV-font-embedding
 */
export function isEmbeddable(fsType: number, version: number): boolean {
  const isBitmapOnly = version >= BITMAP_ONLY_VERSION && isBitSet(fsType, BITMAP_ONLY_BIT);
  const isPermitted = isBitSet(fsType, PREVIEW_AND_PRINT_BIT) || isBitSet(fsType, EDITABLE_BIT);
  const isRestricted = isBitSet(fsType, RESTRICTED_BIT) && !isPermitted;

  return !isBitmapOnly && !isRestricted;
}
