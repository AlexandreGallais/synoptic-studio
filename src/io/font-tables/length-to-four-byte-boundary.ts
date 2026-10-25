import { ceiling } from "../../math";

/** Tables start at a multiple of four bytes, the space between them padded with zeros. */
const TABLE_ALIGNMENT = 4;

/**
 * The room a table takes in a font file: its length padded with zeros up to the next four-byte
 * boundary, so that the next table starts on one, as OpenType asks ("All tables must begin on
 * four-byte boundaries, and any remaining space between tables must be padded with zeros").
 *
 * Formula: ⌈length / 4⌉ · 4.
 *
 * @kind format
 * @param length - the table's length, in bytes
 * @returns its length padded, in bytes
 */
export function lengthToFourByteBoundary(length: number): number {
  return ceiling(length / TABLE_ALIGNMENT) * TABLE_ALIGNMENT;
}
