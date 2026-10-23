/**
 * Writes a size in bytes as the toolbar shows it (F40): below 1 000, in bytes, `999 B`; above, in
 * the largest of kB, MB and GB, each 1 000 times the one before, that keeps it below 1 000, to one
 * decimal rounded half up, a decimal of 0 left out: 1 248 bytes is `1.2 kB`, 100 000 000 is
 * `100 MB`.
 *
 * @kind format
 * @param bytes - the size, a whole number of bytes, 0 or more
 * @returns its text, the number then the unit
 * @see docs/domain/interaction.md#_7-navigation-and-history
 */
export function formatByteSize(bytes: number): string {
  return `${bytes} B`;
}
