import type { FontTableRecord } from "./font-table-record";

/** Offset of `fsType` in the `OS/2` table. */
const FS_TYPE_OFFSET = 8;

/**
 * Reads the embedding permissions of a font, its `fsType` flags: 0 installable, 2 restricted
 * licence, 4 preview and print, 8 editable.
 *
 * @kind format
 * @param view - the font file
 * @param os2 - where its `OS/2` table sits
 * @returns the `fsType` flags
 * @see REF-OT-OS2
 */
export function parseEmbeddingPermissions(view: DataView, os2: FontTableRecord): number {
  return view.getUint16(os2.offset + FS_TYPE_OFFSET);
}
