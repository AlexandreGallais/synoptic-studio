import { isSpanWithin } from "../../math";
import { SFNT_VERSION } from "../font-tables";

import type { FontFormat } from "./font-format";

/** A file with TrueType outlines: the media type and hint CSS Fonts 4 gives `.ttf`. */
const TRUE_TYPE: FontFormat = { hint: "truetype", mediaType: "font/ttf" };

/** A file with CFF outlines: the media type and hint CSS Fonts 4 gives OpenType. */
const OPEN_TYPE: FontFormat = { hint: "opentype", mediaType: "font/otf" };

/**
 * The format a font file is named by in an `@font-face` rule, from its `sfntVersion`, its first
 * four bytes (`REF-OT-OTFF`): `OTTO`, CFF outlines, gives `font/otf` and `opentype`; any other
 * file, TrueType outlines, `font/ttf` and `truetype` (CSS Fonts 4, §11.2, which makes the two
 * hints synonyms; ADR-0053).
 *
 * @kind format
 * @param bytes - the font file
 * @returns its media type and its format hint
 * @see REF-CSS-FONTS-4
 */
export function parseFontFormat(bytes: Readonly<Uint8Array>): FontFormat {
  const view = new DataView(bytes.buffer, bytes.byteOffset, bytes.byteLength);
  const hasCffOutlines =
    isSpanWithin(0, Uint32Array.BYTES_PER_ELEMENT, view.byteLength) &&
    view.getUint32(0) === SFNT_VERSION.cff;

  return hasCffOutlines ? OPEN_TYPE : TRUE_TYPE;
}
