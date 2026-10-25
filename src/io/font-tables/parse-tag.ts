/** Number of bytes of a tag: four ASCII characters. */
const TAG_LENGTH = 4;

/**
 * Reads a tag: four bytes, each an ASCII character (`cmap`, `OS/2`, `kern`).
 *
 * @kind format
 * @param view - the font file
 * @param offset - where the tag starts, in bytes
 * @returns its four characters
 */
export function parseTag(view: Readonly<DataView>, offset: number): string {
  return String.fromCodePoint(
    ...Array.from({ length: TAG_LENGTH }, (unused, index) => view.getUint8(offset + index)),
  );
}
