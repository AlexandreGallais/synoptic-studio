import { isBitSet, isSpanWithin } from "../../math";
import { faceWeight } from "../../model";

import type { FontFace } from "../../model";

/** Offset of `usWeightClass` in the `OS/2` table. */
const WEIGHT_CLASS_OFFSET = 4;

/** Offset of `fsSelection` in the `OS/2` table. */
const FS_SELECTION_OFFSET = 62;

/** The bit of `fsSelection` set when the font holds italic or oblique glyphs. */
const ITALIC_BIT = 0;

/**
 * Reads the weight and the style of a face from its `OS/2` table: its weight from
 * `usWeightClass`, brought into 100 to 900 (`faceWeight`), and italic when bit 0 of
 * `fsSelection`, `ITALIC`, "Font contains italic or oblique glyphs", is set. A table too short
 * to hold `fsSelection` gives no style, never an error.
 *
 * @kind format
 * @param os2 - the font's `OS/2` table
 * @returns its weight and whether it is italic; undefined for a table shorter than 64 bytes
 */
export function parseFontStyle(
  os2: Readonly<DataView>,
): Pick<FontFace, "isItalic" | "weight"> | undefined {
  return isSpanWithin(FS_SELECTION_OFFSET, Uint16Array.BYTES_PER_ELEMENT, os2.byteLength)
    ? {
        isItalic: isBitSet(os2.getUint16(FS_SELECTION_OFFSET), ITALIC_BIT),
        weight: faceWeight(os2.getUint16(WEIGHT_CLASS_OFFSET)),
      }
    : undefined;
}
