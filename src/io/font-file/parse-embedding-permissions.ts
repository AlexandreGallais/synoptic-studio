/** Offset of `fsType` in the `OS/2` table. */
const FS_TYPE_OFFSET = 8;

/**
 * Reads the embedding permissions of a font, its `fsType` flags: 0 installable, 2 restricted
 * licence, 4 preview and print, 8 editable.
 *
 * @kind format
 * @param os2 - the font's `OS/2` table, at least 30 bytes long
 * @returns the `fsType` flags
 * @see REF-OT-OS2
 */
export function parseEmbeddingPermissions(os2: Readonly<DataView>): number {
  return os2.getUint16(FS_TYPE_OFFSET);
}
