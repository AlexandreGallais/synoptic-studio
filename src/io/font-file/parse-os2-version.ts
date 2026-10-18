/** Offset of the table's version in the `OS/2` table. */
const VERSION_OFFSET = 0;

/**
 * Reads the version of a font's `OS/2` table, which says which of its fields exist and how its
 * `fsType` bits are read: `sCapHeight` from version 2, `fsType` bits 4 to 15 from version 2.
 *
 * @kind format
 * @param os2 - the font's `OS/2` table, at least 2 bytes long
 * @returns its version, 0 to 65 535; 0 to 5 in the specification
 * @see REF-OT-OS2
 */
export function parseOs2Version(os2: Readonly<DataView>): number {
  return os2.getUint16(VERSION_OFFSET);
}
